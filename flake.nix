{
  description = "king-of-mountain — Soroban contract dev environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    stellar-cli = {
      url = "path:./stellar-cli-fork";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-overlay.follows = "rust-overlay";
    };
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, stellar-cli }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        rustVersion = (pkgs.rust-bin.stable."1.97.0".default).override {
          targets = [ "wasm32v1-none" ];
        };
      in {
        devShells.default = pkgs.mkShell {
          packages = [ stellar-cli.packages.${system}.default ];
          shellHook = ''
            umask 002
            chgrp -R hermes . 2>/dev/null || true
            find . -type d -exec chmod g+s {} + 2>/dev/null || true
            rustc --version; stellar --version
          '';
          nativeBuildInputs = with pkgs; [
            rustVersion rustfmt clippy rust-analyzer pkg-config jq cargo-watch
          ];
          buildInputs = with pkgs; [ openssl ];
        };
      });
}
