// SPDX-License-Identifier: MPL-2.0
// Long-lived Playwright driver for the converter page (design.md "Test
// driver protocol"). `apps/cli/tests/steps/browser.rs` writes one JSON command
// per line and reads one JSON result per line. The driver serves `web/` on
// loopback, owns one browser context per scenario, and never reports an
// unperformed action as success.

import { createReadStream } from 'node:fs';
import { mkdir, readFile, stat } from 'node:fs/promises';
import { createServer } from 'node:http';
import { createInterface } from 'node:readline';
import { basename, dirname, extname, resolve, sep } from 'node:path';

import AxeBuilder from '@axe-core/playwright';
import { chromium } from 'playwright';

// Every browser scenario serves the assembled Site output, so the check
// inspects the artifact the publication would gate. `./run-tests.sh` assembles
// `dist/site/` and exports it; the publication's gate points it at its own
// output. There is no source-tree fallback: an unnamed root is a setup bug.
const servedRoot = process.env.DUTY2CAL_SITE_ROOT;
if (!servedRoot) {
  throw new Error(
    'DUTY2CAL_SITE_ROOT is required: the driver serves the assembled Site output (./run-tests.sh exports it)',
  );
}
const SERVED_ROOT = resolve(servedRoot);

const CONTENT_TYPES = {
  '.css': 'text/css; charset=utf-8',
  '.html': 'text/html; charset=utf-8',
  '.js': 'text/javascript; charset=utf-8',
  '.json': 'application/json; charset=utf-8',
  '.png': 'image/png',
  '.svg': 'image/svg+xml',
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

/** Serve one file from the served root, refusing every path outside it. */
async function serve(request, response) {
  const requested = new URL(request.url, baseUrl);
  let pathname = decodeURIComponent(requested.pathname);
  if (pathname === '/' || pathname === '') {
    pathname = '/index.html';
  }
  const filePath = resolve(SERVED_ROOT, `.${pathname}`);
  if (filePath !== SERVED_ROOT && !filePath.startsWith(`${SERVED_ROOT}${sep}`)) {
    response.writeHead(403, { 'content-type': 'text/plain' });
    response.end('outside the served root');
    return;
  }
  // GitHub Pages serves `/impressum` for `impressum.html`; the loopback server
  // resolves an extensionless path the same way, so the published address the
  // profile and the README link is the address the steps exercise.
  const candidates = [filePath];
  if (extname(pathname) === '') {
    candidates.push(`${filePath}.html`);
  }
  for (const candidate of candidates) {
    const info = await stat(candidate).catch(() => null);
    if (info?.isFile()) {
      response.writeHead(200, {
        'content-type':
          CONTENT_TYPES[extname(candidate)] || 'application/octet-stream',
        'content-length': info.size,
        'cache-control': 'no-store',
      });
      createReadStream(candidate).pipe(response);
      return;
    }
  }
  response.writeHead(404, { 'content-type': 'text/plain' });
  response.end('not found');
}

/** Fail closed when the directory to serve does not exist. */
async function requireServedRoot() {
  try {
    const info = await stat(SERVED_ROOT);
    if (!info.isDirectory()) {
      throw new Error('not a directory');
    }
  } catch (error) {
    throw new Error(
      `the served root ${SERVED_ROOT} is not a directory: ${error?.message || error}`,
    );
  }
}

/** Start the loopback static server and return its origin. */
async function startServer() {
  await requireServedRoot();
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

/**
 * Read the visible state and the observations the steps assert on. With
 * `withBaseline`, also collect the rendered-state facts the visual baseline
 * asserts (labels, language, keyboard focus, reduced motion and the elements
 * that would ask for an account). With `withSite`, collect the published
 * page's surfaces (header mark, statement panels, footer revision and tokens).
 * This is observation only: it asserts nothing and restores focus afterwards.
 */
async function readState(withBaseline = false, withSite = false) {
  return page.evaluate(async ({ baseline, site }) => {
    const statusText =
      document.getElementById('status')?.textContent?.trim() ||
      document.getElementById('ready-status')?.textContent?.trim() ||
      '';
    const failure = document.getElementById('failure');
    const offerArea = document.getElementById('offer');
    const cannotRun = document.getElementById('cannot-run');
    const offers = [
      ...document.querySelectorAll(
        '#offer a[download], #failure-offer a[download]',
      ),
    ].map((link) => {
      const name = link.getAttribute('download') || '';
      let kind = 'diagnostic';
      if (name.endsWith('.ics')) {
        kind = 'calendar';
      } else if (name.endsWith('.parse-warnings.txt')) {
        kind = 'warnings';
      }
      return {
        name,
        text: link.textContent || '',
        kind,
        scheme: (() => {
          try {
            return new URL(link.href).protocol;
          } catch {
            return '';
          }
        })(),
      };
    });

    const listText = (selector) =>
      [...document.querySelectorAll(selector)]
        .map((item) => item.textContent?.trim() || '')
        .filter(Boolean);
    const warningsList = document.getElementById('warnings');
    const failureErrors = document.getElementById('failure-errors');
    const failureWarnings = document.getElementById('failure-warnings');

    let state = 'empty';
    if (cannotRun && !cannotRun.hidden) {
      state = 'cannot-run';
    } else if (failure && !failure.hidden) {
      state = 'failure';
    } else if (offerArea && !offerArea.hidden && warningsList && !warningsList.hidden) {
      state = 'ready-with-warnings';
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
    const observation = {
      state,
      status: statusText,
      visibleText: document.body.innerText,
      offers,
      failureReason: failure && !failure.hidden
        ? document.getElementById('failure-reason')?.textContent?.trim() || ''
        : null,
      lists: {
        errors:
          failureErrors && !failureErrors.hidden
            ? listText('#error-list li')
            : [],
        warnings:
          warningsList && !warningsList.hidden
            ? listText('#warning-list li')
            : failureWarnings && !failureWarnings.hidden
              ? listText('#failure-warning-list li')
              : [],
      },
      storage,
      address: { url: location.href, historyLength: history.length },
      controls: {
        picker:
          chooseButton != null &&
          chooseButton.offsetParent !== null &&
          chooseButton.disabled === false &&
          pickerInput?.disabled === false,
        chooseAnother:
          (resetButton != null && resetButton.offsetParent !== null) ||
          (document.getElementById('reset-ready')?.offsetParent !== null),
      },
    };
    if (baseline) {
      const visible = (element) => {
        if (!element || element.hidden) {
          return false;
        }
        const style = getComputedStyle(element);
        if (style.display === 'none' || style.visibility !== 'visible') {
          return false;
        }
        const rect = element.getBoundingClientRect();
        return rect.width > 1 && rect.height > 1;
      };
      const text = (value) => (value || '').replace(/\s+/g, ' ').trim();
      const controls = [
        ...document.querySelectorAll(
          'button, a[href], input:not([type="hidden"])',
        ),
      ]
        .filter(visible)
        .map((element) => {
          const labels = element.labels
            ? [...element.labels].map((label) => label.textContent).join(' ')
            : '';
          const label = text(
            labels || element.getAttribute('aria-label') || element.textContent,
          );
          let focusVisible = false;
          try {
            element.focus({ focusVisible: true });
            const style = getComputedStyle(element);
            focusVisible =
              style.outlineStyle !== 'none' &&
              Number.parseFloat(style.outlineWidth) > 0;
          } catch {
            focusVisible = false;
          }
          return {
            tag: element.tagName.toLowerCase(),
            type: element.getAttribute('type') || '',
            label,
            focusVisible,
          };
        });
      // Observation only: leave no control focused for the committed capture.
      document.activeElement?.blur?.();
      const blocks = [
        ...document.querySelectorAll('h1, h2, p, a, button, label, li'),
      ]
        .filter(visible)
        .map((element) => text(element.textContent))
        .filter(Boolean);
      const duplicates = [
        ...new Set(
          blocks.filter((block, index) => blocks.indexOf(block) !== index),
        ),
      ];
      const h1 = document.querySelector('h1');
      observation.baseline = {
        lang: document.documentElement.lang,
        title: document.title,
        h1: text(h1?.textContent),
        h1Count: document.querySelectorAll('h1').length,
        h1Visible: visible(h1),
        controls,
        blocks,
        duplicates,
        reducedMotion: getComputedStyle(document.documentElement)
          .getPropertyValue('--motion-duration')
          .trim(),
        forms: document.forms.length,
        emailInputs: document.querySelectorAll(
          'input[type="email"], input[type="password"], input[type="tel"]',
        ).length,
        submitControls: document.querySelectorAll(
          'button[type="submit"], input[type="submit"]',
        ).length,
        accountHints: [
          ...document.querySelectorAll('[placeholder], [aria-label], [name]'),
        ].filter((element) =>
          /account|e-?mail|sign ?in|log ?in|register|upload/i.test(
            `${element.getAttribute('placeholder') || ''} ${
              element.getAttribute('aria-label') || ''
            } ${element.getAttribute('name') || ''}`,
          ),
        ).length,
        fileInputsInForms: [
          ...document.querySelectorAll('input[type="file"]'),
        ].filter((input) => input.form !== null).length,
      };
    }
    if (site) {
      const tidy = (value) => (value || '').replace(/\s+/g, ' ').trim();
      const sections = [...document.querySelectorAll('main > section')];
      const sectionHeading = (section) =>
        tidy(section.querySelector('h2')?.textContent);
      const terms = sections.find(
        (section) => sectionHeading(section) === 'Before you choose a plan',
      );
      const licence = sections.find(
        (section) => sectionHeading(section) === 'Source and licence',
      );
      const picker = document.getElementById('plan-actions');
      const footer = document.querySelector('footer');
      const header = document.querySelector('header');
      const icon = header?.querySelector('img');
      const pageHeading = header?.querySelector('h1');
      const subheadline = header?.querySelector('.lede');
      const box = (element) => {
        if (!element) {
          return null;
        }
        const bounds = element.getBoundingClientRect();
        return {
          left: bounds.left,
          right: bounds.right,
          top: bounds.top,
          bottom: bounds.bottom,
          width: bounds.width,
          height: bounds.height,
        };
      };
      const statementsIn = (scope) =>
        [...(scope?.querySelectorAll('[data-site-statement]') || [])].map(
          (element) => ({
            name: element.getAttribute('data-site-statement'),
            tag: element.tagName.toLowerCase(),
            text: tidy(element.textContent),
            links: [...element.querySelectorAll('a[href]')].map((link) => ({
              text: tidy(link.textContent),
              href: link.href,
            })),
          }),
        );
      const tab = document.querySelector('link[rel="icon"]');
      const revision = footer?.querySelector(
        '[data-site-statement="built-revision"]',
      );
      const revisionLink = revision?.closest('a[href]');
      const follows = (first, second) =>
        Boolean(
          first &&
            second &&
            (first.compareDocumentPosition(second) &
              Node.DOCUMENT_POSITION_FOLLOWING) !==
              0,
        );
      const tokens = getComputedStyle(document.documentElement);
      const iconBox = box(icon);
      const headingBox = box(pageHeading);
      const subheadlineBox = box(subheadline);
      observation.site = {
        documentStatements: [
          ...document.querySelectorAll('[data-site-statement]'),
        ].map((element) => element.getAttribute('data-site-statement')),
        terms: terms
          ? {
              heading: sectionHeading(terms),
              statements: statementsIn(terms),
              abovePicker: follows(terms, picker),
            }
          : null,
        licence: licence
          ? {
              heading: sectionHeading(licence),
              statements: statementsIn(licence),
              belowPicker: follows(picker, licence),
              aboveFooter: follows(licence, footer),
            }
          : null,
        footer: footer
          ? {
              text: tidy(footer.innerText),
              statements: statementsIn(footer).map(
                (statement) => statement.name,
              ),
              revisionText: tidy(revision?.textContent),
              revisionHref: revisionLink?.href || null,
            }
          : null,
        header: {
          iconAlt: icon?.getAttribute('alt') ?? null,
          iconSrc: icon?.src ?? null,
          iconBox,
          headingBox,
          subheadlineBox,
          iconLeftOfHeading: Boolean(
            iconBox && headingBox && iconBox.right <= headingBox.left,
          ),
          iconLeftOfSubheadline: Boolean(
            iconBox && subheadlineBox && iconBox.right <= subheadlineBox.left,
          ),
          iconOverlapsHeadingRow: Boolean(
            iconBox &&
              headingBox &&
              subheadlineBox &&
              iconBox.top < subheadlineBox.bottom &&
              iconBox.bottom > headingBox.top,
          ),
        },
        tabIcon: { rel: tab?.getAttribute('rel') ?? null, href: tab?.href ?? null },
        tokens: {
          motionDuration: tokens.getPropertyValue('--motion-duration').trim(),
          radius: tokens.getPropertyValue('--radius').trim(),
          space: tokens.getPropertyValue('--space').trim(),
        },
      };
    }
    return observation;
  }, { baseline: withBaseline, site: withSite });
}

/** The offer link matching one kind. */
function offerSelector(kind) {
  if (kind === 'calendar') {
    return '#offer a[download$=".ics"]';
  }
  if (kind === 'warnings') {
    return '#offer a[download$=".parse-warnings.txt"]';
  }
  return '#failure-offer a[download$=".parse-error.txt"]';
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
    // A request the page attempts but the browser refuses — the page's CSP
    // `connect-src 'none'` blocks every fetch — never reaches the network
    // stack, so it produces no `request`/`requestfailed` events. The violation
    // event is the observable record of the attempt, and the request log the
    // steps read carries it, so an attempted request to another origin is
    // never invisible.
    await context.addInitScript(() => {
      window.__duty2calBlockedRequests = [];
      document.addEventListener('securitypolicyviolation', (event) => {
        window.__duty2calBlockedRequests.push({
          url: event.blockedURI,
          directive: event.effectiveDirective || event.violatedDirective || 'blocked',
        });
      });
    });
    const blockedType =
      mode === 'calendar-offer-failure'
        ? 'text/calendar'
        : mode === 'diagnostic-offer-failure' || mode === 'warning-offer-failure'
          ? 'text/plain'
          : undefined;
    if (mode && !blockedType && !['no-webassembly', 'no-modules', 'broken-module'].includes(mode)) {
      throw new Error(`unknown start mode: ${mode}`);
    }
    if (blockedType) {
      // `warning-offer-failure` refuses only the conversion's first plain-text
      // offer. The success path creates exactly one — the warning report — so
      // the calendar offer survives and the page can state what is missing.
      const onlyFirst = mode === 'warning-offer-failure';
      await context.addInitScript(
        ({ type, first }) => {
          const create = URL.createObjectURL.bind(URL);
          let refused = false;
          URL.createObjectURL = (blob) => {
            const matches = blob instanceof Blob && blob.type.startsWith(type);
            if (matches && (!first || !refused)) {
              refused = true;
              throw new Error(`the driver refuses a ${type} object URL`);
            }
            return create(blob);
          };
        },
        { type: blockedType, first: onlyFirst },
      );
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

  async read_state(params) {
    if (!page) {
      throw new Error('the page is not started');
    }
    const blocked = await page.evaluate(
      () => window.__duty2calBlockedRequests || [],
    );
    return {
      ...(await readState(
        Boolean(params && params.baseline),
        Boolean(params && params.site),
      )),
      requestLog: [
        ...requestLog,
        ...blocked.map((entry) => ({
          url: entry.url,
          method: '',
          resourceType: 'blocked',
          failed: entry.directive,
        })),
      ],
      initialAddress,
    };
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
    // The baseline observation focuses every visible control, and focusing a
    // footer link scrolls the page. A capture that inherited that scroll would
    // not be the screen's top-of-page state, so the viewport is returned to the
    // top before the frame is settled.
    await page.evaluate(async () => {
      window.scrollTo(0, 0);
      await document.fonts.ready;
      await new Promise((frame) => requestAnimationFrame(frame));
      await new Promise((frame) => requestAnimationFrame(frame));
      window.scrollTo(0, 0);
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

  async drop_file(params) {
    if (!page) {
      throw new Error('the page is not started');
    }
    if (!params.path) {
      throw new Error('drop_file needs a path');
    }
    const bytes = await readFile(params.path);
    const name = basename(params.path);
    const defaultPrevented = await page.evaluate(
      ({ filename, base64 }) => {
        const binary = atob(base64);
        const data = new Uint8Array(binary.length);
        for (let index = 0; index < binary.length; index += 1) {
          data[index] = binary.charCodeAt(index);
        }
        const file = new File([data], filename, {
          type: 'application/pdf',
        });
        const transfer = new DataTransfer();
        transfer.items.add(file);
        const zone = document.getElementById('drop-zone');
        if (!zone) {
          throw new Error('the page has no drop zone');
        }
        let prevented = false;
        for (const type of ['dragenter', 'dragover', 'drop']) {
          const event = new DragEvent(type, {
            bubbles: true,
            cancelable: true,
            dataTransfer: transfer,
          });
          zone.dispatchEvent(event);
          prevented = prevented || event.defaultPrevented;
        }
        return prevented;
      },
      { filename: name, base64: bytes.toString('base64') },
    );
    await waitUntilSettled();
    return { defaultPrevented };
  },

  async dismiss_save() {
    if (!page) {
      throw new Error('the page is not started');
    }
    const link = page.locator(offerSelector('calendar'));
    if ((await link.count()) === 0) {
      throw new Error('no calendar offer is present to dismiss');
    }
    const suggestedFilename = await link.getAttribute('download');
    const [download] = await Promise.all([
      page.waitForEvent('download', { timeout: 30000 }),
      link.click(),
    ]);
    await download.cancel();
    return { dismissed: true, suggestedFilename };
  },

  async navigate(params) {
    if (!page) {
      throw new Error('the page is not started');
    }
    if (!params.path) {
      throw new Error('navigate needs a path');
    }
    await page.goto(`${baseUrl}${params.path}`, { waitUntil: 'load' });
    return { url: page.url(), title: await page.title() };
  },

  async click(params) {
    if (!page) {
      throw new Error('the page is not started');
    }
    if (!params.text) {
      throw new Error('click needs the visible text of the link');
    }
    const link = page.getByRole('link', { name: params.text, exact: true });
    if ((await link.count()) === 0) {
      throw new Error(`no link "${params.text}" is present`);
    }
    await Promise.all([page.waitForLoadState('load'), link.first().click()]);
    return { url: page.url(), title: await page.title() };
  },

  async read_text() {
    if (!page) {
      throw new Error('the page is not started');
    }
    return page.evaluate(() => {
      const tidy = (value) => (value || '').replace(/\s+/g, ' ').trim();
      return {
        url: location.href,
        title: document.title,
        lang: document.documentElement.lang,
        h1: tidy(document.querySelector('h1')?.innerText),
        text: document.body.innerText,
        links: [...document.querySelectorAll('a[href]')].map((link) => ({
          text: tidy(link.textContent),
          href: link.href,
          scheme: (() => {
            try {
              return new URL(link.href).protocol;
            } catch {
              return '';
            }
          })(),
        })),
        cookies: document.cookie,
        storage: {
          local: Object.keys(localStorage),
          session: Object.keys(sessionStorage),
        },
      };
    });
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
