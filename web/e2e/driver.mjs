// SPDX-License-Identifier: MPL-2.0
// Long-lived Playwright driver for the converter page (design.md "Test
// driver protocol"). `apps/cli/tests/steps/browser.rs` writes one JSON command
// per line and reads one JSON result per line. The driver serves `web/` on
// loopback, owns one browser context per scenario, and never reports an
// unperformed action as success.

import { createReadStream } from 'node:fs';
import { mkdir, stat } from 'node:fs/promises';
import { createServer } from 'node:http';
import { createInterface } from 'node:readline';
import { dirname, extname, resolve, sep } from 'node:path';
import { fileURLToPath } from 'node:url';

import AxeBuilder from '@axe-core/playwright';
import { chromium } from 'playwright';

const WEB_ROOT = resolve(dirname(fileURLToPath(import.meta.url)), '..');

const CONTENT_TYPES = {
  '.css': 'text/css; charset=utf-8',
  '.html': 'text/html; charset=utf-8',
  '.js': 'text/javascript; charset=utf-8',
  '.json': 'application/json; charset=utf-8',
  '.png': 'image/png',
  '.txt': 'text/plain; charset=utf-8',
  '.wasm': 'application/wasm',
};

let server;
let browser;
let context;
let page;
let baseUrl;
let requestLog = [];
let initialAddress;

/** Write one protocol result line. */
function answer(result) {
  process.stdout.write(`${JSON.stringify(result)}\n`);
}

/** Serve one file from `web/`, refusing every path outside it. */
async function serve(request, response) {
  const requested = new URL(request.url, baseUrl);
  let pathname = decodeURIComponent(requested.pathname);
  if (pathname === '/' || pathname === '') {
    pathname = '/index.html';
  }
  const filePath = resolve(WEB_ROOT, `.${pathname}`);
  if (filePath !== WEB_ROOT && !filePath.startsWith(`${WEB_ROOT}${sep}`)) {
    response.writeHead(403, { 'content-type': 'text/plain' });
    response.end('outside the served root');
    return;
  }
  try {
    const info = await stat(filePath);
    if (!info.isFile()) {
      throw new Error('not a file');
    }
    response.writeHead(200, {
      'content-type':
        CONTENT_TYPES[extname(filePath)] || 'application/octet-stream',
      'content-length': info.size,
      'cache-control': 'no-store',
    });
    createReadStream(filePath).pipe(response);
  } catch {
    response.writeHead(404, { 'content-type': 'text/plain' });
    response.end('not found');
  }
}

/** Start the loopback static server and return its origin. */
async function startServer() {
  const loopback = createServer((request, response) => {
    serve(request, response).catch((error) => {
      response.writeHead(500, { 'content-type': 'text/plain' });
      response.end(String(error));
    });
  });
  await new Promise((accept, reject) => {
    loopback.once('error', reject);
    loopback.listen(0, '127.0.0.1', accept);
  });
  const address = loopback.address();
  server = loopback;
  baseUrl = `http://127.0.0.1:${address.port}`;
  return baseUrl;
}

/** Wait until the page has initialized and revealed the controls. */
async function waitUntilUsable() {
  await page.waitForFunction(
    () => document.getElementById('plan-actions')?.hidden === false,
    null,
    { timeout: 60000 },
  );
}

/** Wait until a supplied file has settled (ready, failure, or refused). */
async function waitUntilSettled() {
  await page.waitForFunction(
    () => {
      const text =
        document.getElementById('status')?.textContent?.trim() || '';
      return text !== 'No plan selected.';
    },
    null,
    { timeout: 180000 },
  );
  await page.waitForFunction(
    () =>
      !(document.getElementById('status')?.textContent || '').includes(
        'Converting',
      ),
    null,
    { timeout: 180000 },
  );
}

/** Read the visible state and the observations the steps assert on. */
async function readState() {
  return page.evaluate(async () => {
    const statusText =
      document.getElementById('status')?.textContent?.trim() || '';
    const failure = document.getElementById('failure');
    const offerArea = document.getElementById('offer');
    const cannotRun = document.getElementById('cannot-run');
    const offers = [
      ...document.querySelectorAll(
        '#offer a[download], #failure-offer a[download]',
      ),
    ].map((link) => ({
      name: link.getAttribute('download') || '',
      text: link.textContent || '',
      kind: (link.getAttribute('download') || '').endsWith('.ics')
        ? 'calendar'
        : 'diagnostic',
      scheme: (() => {
        try {
          return new URL(link.href).protocol;
        } catch {
          return '';
        }
      })(),
    }));

    let state = 'empty';
    if (cannotRun && !cannotRun.hidden) {
      state = 'cannot-run';
    } else if (failure && !failure.hidden) {
      state = 'failure';
    } else if (offerArea && !offerArea.hidden) {
      state = 'ready';
    } else if (statusText.includes('Converting')) {
      state = 'converting';
    }

    const storage = {
      local: Object.fromEntries(
        Object.keys(localStorage).map((key) => [key, localStorage.getItem(key)]),
      ),
      session: Object.fromEntries(
        Object.keys(sessionStorage).map((key) => [
          key,
          sessionStorage.getItem(key),
        ]),
      ),
      cookies: document.cookie,
      databases: [],
    };
    try {
      storage.databases = (await indexedDB.databases()).map(
        (database) => database.name,
      );
    } catch {
      storage.databases = [];
    }

    const resetButton = document.getElementById('reset');
    const chooseButton = document.getElementById('choose');
    const pickerInput = document.getElementById('plan');
    return {
      state,
      status: statusText,
      visibleText: document.body.innerText,
      offers,
      failureReason: failure && !failure.hidden
        ? document.getElementById('failure-reason')?.textContent?.trim() || ''
        : null,
      storage,
      address: { url: location.href, historyLength: history.length },
      controls: {
        picker:
          chooseButton != null &&
          chooseButton.offsetParent !== null &&
          chooseButton.disabled === false &&
          pickerInput?.disabled === false,
        chooseAnother:
          resetButton != null && resetButton.offsetParent !== null,
      },
    };
  });
}

/** The offer link matching one kind. */
function offerSelector(kind) {
  return kind === 'calendar'
    ? '#offer a[download$=".ics"]'
    : '#failure-offer a[download$=".parse-error.txt"]';
}

const actions = {
  async start(params) {
    if (browser) {
      throw new Error('the driver is already started');
    }
    await startServer();
    browser = await chromium.launch({
      executablePath: process.env.CHROMIUM_PATH || undefined,
      headless: true,
    });
    context = await browser.newContext({
      viewport: {
        width: params.width || 1280,
        height: params.height || 900,
      },
      colorScheme: 'light',
      reducedMotion: 'reduce',
    });
    const mode = params.mode;
    const blockedType =
      mode === 'calendar-offer-failure'
        ? 'text/calendar'
        : mode === 'diagnostic-offer-failure'
          ? 'text/plain'
          : undefined;
    if (mode && !blockedType && !['no-webassembly', 'no-modules', 'broken-module'].includes(mode)) {
      throw new Error(`unknown start mode: ${mode}`);
    }
    if (blockedType) {
      await context.addInitScript((type) => {
        const create = URL.createObjectURL.bind(URL);
        URL.createObjectURL = (blob) => {
          if (blob instanceof Blob && blob.type.startsWith(type)) {
            throw new Error(`the driver refuses a ${type} object URL`);
          }
          return create(blob);
        };
      }, blockedType);
    }
    if (mode === 'no-webassembly') {
      await context.addInitScript(() => {
        try {
          delete window.WebAssembly;
        } catch {
          // A non-configurable binding is replaced through its descriptor.
        }
        if (typeof window.WebAssembly !== 'undefined') {
          Object.defineProperty(window, 'WebAssembly', {
            configurable: true,
            get: () => undefined,
          });
        }
      });
    }
    if (mode === 'no-modules') {
      await context.addInitScript(() => {
        const unsupported = () => false;
        try {
          Object.defineProperty(HTMLScriptElement, 'supports', {
            configurable: true,
            value: unsupported,
          });
        } catch {
          HTMLScriptElement.supports = unsupported;
        }
      });
    }
    if (mode === 'broken-module') {
      await context.addInitScript(() => {
        const reject = () =>
          Promise.reject(new Error('the driver refuses to instantiate the module'));
        try {
          WebAssembly.instantiate = reject;
          if (typeof WebAssembly.instantiateStreaming === 'function') {
            WebAssembly.instantiateStreaming = reject;
          }
        } catch {
          Object.defineProperty(WebAssembly, 'instantiate', {
            configurable: true,
            value: reject,
          });
          Object.defineProperty(WebAssembly, 'instantiateStreaming', {
            configurable: true,
            value: reject,
          });
        }
      });
    }
    page = await context.newPage();
    requestLog = [];
    page.on('request', (request) => {
      requestLog.push({
        url: request.url(),
        method: request.method(),
        resourceType: request.resourceType(),
      });
    });
    page.on('requestfailed', (request) => {
      requestLog.push({
        url: request.url(),
        method: request.method(),
        resourceType: request.resourceType(),
        failed: request.failure()?.errorText || 'failed',
      });
    });
    await page.goto(`${baseUrl}/index.html`, { waitUntil: 'load' });
    if (['no-webassembly', 'no-modules', 'broken-module'].includes(mode)) {
      await page.waitForFunction(
        () => document.getElementById('cannot-run')?.hidden === false,
        null,
        { timeout: 60000 },
      );
    } else {
      await waitUntilUsable();
    }
    initialAddress = {
      url: page.url(),
      historyLength: await page.evaluate(() => history.length),
    };
    return { url: page.url(), origin: baseUrl };
  },

  async set_file(params) {
    if (!page) {
      throw new Error('the page is not started');
    }
    if (!params.path) {
      throw new Error('set_file needs a path');
    }
    await page.setInputFiles('#plan', params.path);
    await waitUntilSettled();
    return {};
  },

  async read_state() {
    if (!page) {
      throw new Error('the page is not started');
    }
    return { ...(await readState()), requestLog, initialAddress };
  },

  async save_offer(params) {
    if (!page) {
      throw new Error('the page is not started');
    }
    if (!params.path) {
      throw new Error('save_offer needs a path');
    }
    const selector = offerSelector(params.kind || 'calendar');
    const link = page.locator(selector);
    if ((await link.count()) === 0) {
      throw new Error(`no ${params.kind || 'calendar'} offer is present`);
    }
    const suggestedFilename = await link.getAttribute('download');
    const [download] = await Promise.all([
      page.waitForEvent('download', { timeout: 30000 }),
      link.click(),
    ]);
    await mkdir(dirname(params.path), { recursive: true });
    await download.saveAs(params.path);
    return { savedTo: params.path, suggestedFilename };
  },

  async reload() {
    if (!page) {
      throw new Error('the page is not started');
    }
    await page.reload({ waitUntil: 'load' });
    await waitUntilUsable();
    return {};
  },

  async screenshot(params) {
    if (!page) {
      throw new Error('the page is not started');
    }
    if (!params.path) {
      throw new Error('screenshot needs a path');
    }
    if (params.width && params.height) {
      await page.setViewportSize({
        width: params.width,
        height: params.height,
      });
    }
    await page.evaluate(async () => {
      await document.fonts.ready;
      await new Promise((frame) => requestAnimationFrame(frame));
      await new Promise((frame) => requestAnimationFrame(frame));
    });
    await mkdir(dirname(params.path), { recursive: true });
    await page.screenshot({
      path: params.path,
      fullPage: Boolean(params.fullPage),
      animations: 'disabled',
    });
    return { savedTo: params.path };
  },

  async axe() {
    if (!page) {
      throw new Error('the page is not started');
    }
    const analysis = await new AxeBuilder({ page }).analyze();
    return { violations: analysis.violations };
  },

  async drop_file() {
    throw new Error('drop_file is not implemented yet');
  },

  async dismiss_save() {
    throw new Error('dismiss_save is not implemented yet');
  },

  async stop() {
    await cleanup();
    return {};
  },
};

/** Close the context, browser and server, in that order. */
async function cleanup() {
  if (context) {
    await context.close().catch(() => {});
    context = undefined;
  }
  if (browser) {
    await browser.close().catch(() => {});
    browser = undefined;
  }
  if (server) {
    await new Promise((accept) => server.close(accept));
    server = undefined;
  }
  page = undefined;
}

let stopping = false;

const lines = createInterface({ input: process.stdin, crlfDelay: Infinity });
for await (const line of lines) {
  const text = line.trim();
  if (!text) {
    continue;
  }
  let command;
  try {
    command = JSON.parse(text);
  } catch (error) {
    answer({ ok: false, error: `malformed command: ${error.message}` });
    continue;
  }
  const action = command.action;
  const handler = actions[action];
  if (typeof handler !== 'function') {
    answer({ ok: false, error: `unknown action: ${action}` });
    continue;
  }
  try {
    const result = await handler(command);
    answer({ ok: true, ...result });
  } catch (error) {
    answer({ ok: false, error: error?.message || String(error) });
  }
  if (action === 'stop') {
    stopping = true;
    break;
  }
}

if (!stopping) {
  await cleanup();
}
process.exit(0);
