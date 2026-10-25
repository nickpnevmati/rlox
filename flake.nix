{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        devShell = pkgs.mkShell {
          packages = [
            toolchain
            pkgs.cargo-nextest
            pkgs.cargo-watch
            pkgs.bacon
            pkgs.pkg-config
          ];
          buildInputs = [ pkgs.openssl ];
          RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";
        };
      in {
        devShells.default = devShell;

        # GC-root target. nix-direnv only roots the dev shell's runtime
        # closure, so the weekly nix-gc reaps the build-time deps (stdenv
        # bootstrap, perl, m4, bison, bash patches) that `nix develop`
        # needs on a cache miss -- which then needs the network. Rooting
        # this keeps a cold `nix develop` working offline:
        #
        #   nix build .#devShellInputs \
        #     --out-link ~/.local/state/nix/gcroots/rlox-devshell
        packages.devShellInputs = devShell.inputDerivation;
      });
}
