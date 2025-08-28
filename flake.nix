{
  description = "amalgam: Generate type-safe Nickel configurations from any schema source";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, fenix, crane, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          config.allowUnfree = true;
        };

        # Use latest stable Rust with all components
        rustWithComponents = fenix.packages.${system}.stable.withComponents [
          "cargo"
          "rustc"
          "rust-src"
          "rustfmt"
          "clippy"
          "rust-analyzer"
        ];

        # Crane lib configured with our toolchain
        craneLib = (crane.mkLib pkgs).overrideToolchain rustWithComponents;

        # Build dependencies only
        cargoArtifacts = craneLib.buildDepsOnly {
          src = craneLib.cleanCargoSource ./.;

          # Build-time dependencies
          buildInputs = with pkgs; [
            openssl
            pkg-config
          ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [];
        };

        # Build the actual crate
        amalgam = craneLib.buildPackage {
          inherit cargoArtifacts;
          src = craneLib.cleanCargoSource ./.;

          buildInputs = with pkgs; [
            openssl
            pkg-config
          ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [];
        };

      in
      {
        # Packages
        packages = {
          default = amalgam;
          amalgam = amalgam;
        };

        # Apps
        apps = {
          default = flake-utils.lib.mkApp {
            drv = amalgam;
            name = "amalgam";
          };
        };

        # Development shell
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Rust toolchain from fenix
            rustWithComponents

            # Build dependencies
            openssl
            pkg-config

            # Development tools
            claude-code
            cargo-watch
            cargo-edit
            cargo-expand
            cargo-outdated
            cargo-audit
            cargo-license
            cargo-tarpaulin  # code coverage
            cargo-criterion  # benchmarking

            # For WASM builds (future)
            wasm-pack
            wasm-bindgen-cli

            # General dev tools
            just
            bacon
            hyperfine  # benchmarking
            tokei      # code statistics

            # For working with schemas
            jq
            yq

            # For Kubernetes integration
            kubectl
            kind

          ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [];

          shellHook = ''
            echo "🦀 Rust toolchain: $(rustc --version)"
            echo "📦 Cargo version: $(cargo --version)"
            echo ""
            echo "Available commands:"
            echo "  cargo build    - Build the project"
            echo "  cargo test     - Run tests"
            echo "  cargo check    - Check compilation"
            echo "  cargo clippy   - Run linter"
            echo "  cargo fmt      - Format code"
            echo "  cargo watch    - Watch for changes"
            echo "  cargo audit    - Check for vulnerabilities"
            echo ""
            echo "amalgam development environment ready!"
          '';

          # Environment variables
          RUST_SRC_PATH = "${rustWithComponents}/lib/rustlib/src/rust/library";
          RUST_BACKTRACE = "1";
          RUST_LOG = "debug";
        };

        # Checks
        checks = {
          inherit amalgam;

          amalgam-clippy = craneLib.cargoClippy {
            inherit cargoArtifacts;
            src = craneLib.cleanCargoSource ./.;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          };

          amalgam-fmt = craneLib.cargoFmt {
            src = craneLib.cleanCargoSource ./.;
          };

          amalgam-tests = craneLib.cargoTest {
            inherit cargoArtifacts;
            src = craneLib.cleanCargoSource ./.;
          };
        };
      });
}
