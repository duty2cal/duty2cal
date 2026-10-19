// SPDX-License-Identifier: MPL-2.0
// Converter page module. It initializes the WASM instance from the inlined
// bytes (no fetch: the CSP forbids one) and reveals the controls. The file
// handling and conversion wiring land with the scenarios.

import init from './pkg/duty2cal_wasm.js';
import { wasmBase64 } from './pkg/wasm-inline.js';

/** Decode the build-time base64 string into the bytes `init` instantiates. */
function decodeBase64(value) {
  const binary = atob(value);
  const bytes = new Uint8Array(binary.length);
  for (let index = 0; index < binary.length; index += 1) {
    bytes[index] = binary.charCodeAt(index);
  }
  return bytes;
}

/** Show the startup failure and leave the picker hidden. */
function showStartupFailure() {
  const reason = document.getElementById('cannot-run-reason');
  if (reason) {
    reason.textContent = 'The converter could not start in this browser.';
  }
  const panel = document.getElementById('cannot-run');
  if (panel) {
    panel.hidden = false;
  }
}

/** Instantiate the module and reveal the controls on success. */
async function start() {
  if (document.documentElement.dataset.duty2calSupported !== 'true') {
    return;
  }
  try {
    await init(decodeBase64(wasmBase64));
  } catch {
    showStartupFailure();
    return;
  }
  const actions = document.getElementById('plan-actions');
  if (actions) {
    actions.hidden = false;
  }
}

start();
