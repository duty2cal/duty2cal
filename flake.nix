# SPDX-License-Identifier: MPL-2.0
{
  description = "duty2cal developer environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
    }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
      };
      toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      mingw = pkgs.pkgsCross.mingwW64;
      # Its packaging tests fail in this nixpkgs revision; the tool itself is
      # pinned by version in scripts/dev-tool-versions.env.
      cargo-llvm-cov = pkgs.cargo-llvm-cov.overrideAttrs (_: {
        doCheck = false;
      });
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        packages = [
          toolchain
          cargo-llvm-cov
          pkgs.cargo-deny
          pkgs.cargo-about
          pkgs.cargo-bloat
          pkgs.git-cliff
          pkgs.wasm-pack
          pkgs.nodejs_24
          pkgs.python3
          pkgs.jq
          pkgs.bats
          pkgs.actionlint
          pkgs.playwright-driver.browsers
        ];
        buildInputs = [
          mingw.stdenv.cc
          mingw.windows.pthreads
          mingw.windows.mcfgthreads
        ];
        env = {
          PLAYWRIGHT_BROWSERS_PATH = "${pkgs.playwright-driver.browsers}";
          PLAYWRIGHT_SKIP_VALIDATE_HOST_REQUIREMENTS = "1";
          # The cross gcc wrapper reads its flags from this prefixed variable;
          # the two libraries' own setup hooks do not populate it in a devShell.
          NIX_LDFLAGS_x86_64_w64_mingw32 =
            "-L${mingw.windows.pthreads}/lib -L${mingw.windows.mcfgthreads}/lib";
        };
        shellHook = ''
          echo "duty2cal: $(rustc -V)"
          echo "run: ./scripts/check-toolchain.sh && ./run-tests.sh"
        '';
      };
    };
}
