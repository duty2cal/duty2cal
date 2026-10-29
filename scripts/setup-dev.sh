#!/usr/bin/env bash
# SPDX-License-Identifier: MPL-2.0
# Bring a Linux or macOS machine to the pinned developer environment without
# root: rustup and the pinned toolchain, the pinned cargo tools, Node, the
# browser test packages and Chromium. Operating-system packages are reported,
# never installed. `--check` reports without changing anything.
#
# See design.md "Non-Nix installer", "Node" and "Playwright".
set -euo pipefail
cd "$(dirname "$0")/.."

check=0
case "${1:-}" in
  --check) check=1 ;;
  "") ;;
  *)
    printf 'setup-dev: unknown argument: %s\n' "$1" >&2
    exit 2
    ;;
esac

missing=0
mismatch=0
report() {
  printf 'setup-dev: %s\n' "$*" >&2
  missing=1
}
note() {
  printf 'setup-dev: %s\n' "$*"
}

. scripts/dev-tool-versions.env

export CARGO_HOME="${CARGO_HOME:-$HOME/.cargo}"
if [ -f "$CARGO_HOME/env" ]; then . "$CARGO_HOME/env"; fi
export PATH="$CARGO_HOME/bin:$PATH"

if ! command -v rustup >/dev/null 2>&1; then
  if [ "$check" = 1 ]; then
    report "rustup is not installed; run this script without --check, or install it from https://rustup.rs"
  else
    note "installing rustup"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
      | sh -s -- -y --profile minimal --default-toolchain none
    if [ -f "$CARGO_HOME/env" ]; then . "$CARGO_HOME/env"; fi
  fi
fi

if command -v rustup >/dev/null 2>&1; then
  if [ "$check" = 1 ]; then
    command -v rustc >/dev/null 2>&1 || report "rustc is not installed under rustup; run 'rustup show'"
  else
    note "installing the pinned toolchain, components and targets"
    rustup show
  fi
fi

probe_llvm_cov() { cargo llvm-cov --version; }
probe_deny() { cargo deny --version; }
probe_about() { cargo about --version; }
probe_bloat() { cargo bloat --version; }
probe_cliff() { git-cliff --version; }

install_tool() {
  crate="$1"
  version="$2"
  probe="$3"
  if installed="$($probe 2>/dev/null | awk '{print $NF}')" && [ "$installed" = "$version" ]; then
    return 0
  fi
  if [ "$check" = 1 ]; then
    report "$crate $version is not installed; run 'cargo install --locked $crate --version $version'"
  else
    note "installing $crate $version"
    cargo install --locked "$crate" --version "$version"
  fi
}

if command -v cargo >/dev/null 2>&1; then
  install_tool cargo-llvm-cov "$CARGO_LLVM_COV_VERSION" probe_llvm_cov
  install_tool cargo-deny "$CARGO_DENY_VERSION" probe_deny
  install_tool cargo-about "$CARGO_ABOUT_VERSION" probe_about
  install_tool cargo-bloat "$CARGO_BLOAT_VERSION" probe_bloat
  install_tool git-cliff "$GIT_CLIFF_VERSION" probe_cliff
else
  report "cargo is not installed; run 'rustup show' after rustup is available"
fi

node_major="$(tr -d '[:space:]' <web/e2e/.nvmrc)"
if [ -n "${NVM_DIR:-}" ] && [ -s "$NVM_DIR/nvm.sh" ]; then . "$NVM_DIR/nvm.sh"; fi
if command -v nvm >/dev/null 2>&1; then
  if [ "$check" = 1 ]; then
    current="$(node -v 2>/dev/null | sed 's/^v//' || true)"
    case "$current" in
      "$node_major".*) : ;;
      *) report "Node $node_major is not active (found ${current:-none}); run 'nvm install $node_major'" ;;
    esac
  else
    note "installing Node $node_major"
    nvm install "$node_major"
  fi
elif command -v node >/dev/null 2>&1; then
  current="$(node -v | sed 's/^v//')"
  case "$current" in
    "$node_major".*) : ;;
    *) report "Node $node_major is required (found $current); install nvm or Node $node_major" ;;
  esac
else
  report "Node $node_major is not installed; install nvm (https://github.com/nvm-sh/nvm) or Node $node_major"
fi

if command -v npm >/dev/null 2>&1; then
  if [ "$check" = 1 ]; then
    [ -d web/e2e/node_modules/playwright ] \
      || report "the web/e2e packages are not installed; run 'npm ci --prefix web/e2e'"
  else
    note "installing the browser test packages"
    npm ci --prefix web/e2e
    npx --prefix web/e2e playwright install chromium
    if [ "$(uname -s)" = Linux ]; then
      note "host packages the browser needs (dry run):"
      npx --prefix web/e2e playwright install-deps --dry-run chromium 2>&1 \
        | sed 's/^/setup-dev: /' || true
      chrome_bin="$(find "${PLAYWRIGHT_BROWSERS_PATH:-$HOME/.cache/ms-playwright}" \
        -path '*/chrome-linux/chrome' -type f 2>/dev/null | head -n 1 || true)"
      if [ -n "$chrome_bin" ]; then
        missing_libs="$(ldd "$chrome_bin" 2>/dev/null | awk '/not found/ {print $1}' | sort -u | tr '\n' ' ' || true)"
        if [ -n "$missing_libs" ]; then
          report "Chromium needs host libraries: $missing_libs; install them with your package manager (see 'npx --prefix web/e2e playwright install-deps --dry-run')"
        fi
      fi
    fi
  fi
else
  report "npm is not installed; install Node $node_major"
fi

if [ -n "${PLAYWRIGHT_BROWSERS_PATH:-}" ] && [ -n "${PLAYWRIGHT_DRIVER_VERSION:-}" ] \
  && [ -f web/e2e/node_modules/playwright/package.json ]; then
  npm_playwright="$(node -p "require('./web/e2e/node_modules/playwright/package.json').version" 2>/dev/null || true)"
  if [ -n "$npm_playwright" ] && [ "$npm_playwright" != "$PLAYWRIGHT_DRIVER_VERSION" ]; then
    mismatch=1
    report "Playwright $npm_playwright in web/e2e does not match the browser build $PLAYWRIGHT_DRIVER_VERSION; run 'npm ci --prefix web/e2e' or update the browser pin"
  fi
fi

need() {
  command -v "$1" >/dev/null 2>&1 || report "$2"
}
need python3 "python3 is required by the scripts"
need jq "jq is required by the publication scripts"
need bats "bats is required for scripts/tests"
need actionlint "actionlint is required for the workflow check"
need reuse "reuse is required for the licence check"
need cc "a C compiler (cc) is required for the build"
if ! command -v x86_64-w64-mingw32-gcc >/dev/null 2>&1; then
  case "$(uname -s)" in
    Linux) report "x86_64-w64-mingw32-gcc is required for the Windows cross build; install the 'mingw-w64' package" ;;
    *) report "the Windows cross build needs x86_64-w64-mingw32-gcc, which this script does not install on $(uname -s)" ;;
  esac
fi

if [ "$mismatch" -ne 0 ]; then
  exit 1
fi
if [ "$check" = 1 ]; then
  if [ "$missing" -ne 0 ]; then
    exit 1
  fi
  note "developer environment matches rust-toolchain.toml and scripts/dev-tool-versions.env"
else
  if [ "$missing" -ne 0 ]; then
    note "review the reported items above; nothing was installed with root"
  fi
fi
