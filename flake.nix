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

        # Publishing scripts
        publish-dry-run = pkgs.writeShellScriptBin "publish-dry-run" ''
          echo "🔍 Running publish dry-run for all crates..."
          echo ""
          for crate in amalgam-core amalgam-codegen amalgam-parser amalgam-daemon; do
            echo "Checking $crate..."
            (cd crates/$crate && ${rustWithComponents}/bin/cargo publish --dry-run) || exit 1
          done
          echo "Checking amalgam (CLI)..."
          (cd crates/amalgam-cli && ${rustWithComponents}/bin/cargo publish --dry-run) || exit 1
          echo ""
          echo "✅ All crates ready for publishing!"
        '';

        publish-all = pkgs.writeShellScriptBin "publish-all" ''
          publish_crate() {
            local crate_path=$1
            local crate_name=$2
            
            echo "📦 Publishing $crate_name..."
            (cd "$crate_path" && ${rustWithComponents}/bin/cargo publish --dry-run && ${rustWithComponents}/bin/cargo publish)
            
            if [ $? -eq 0 ]; then
              echo "⏳ Waiting 30s for crates.io to index $crate_name..."
              sleep 30
              echo "✅ $crate_name published!"
            else
              echo "❌ Failed to publish $crate_name"
              return 1
            fi
          }

          echo "🚀 Publishing all amalgam crates to crates.io..."
          echo ""
          echo "⚠️  Make sure you're logged in: cargo login <token>"
          echo ""
          read -p "Continue with publishing? (y/N) " -n 1 -r
          echo
          if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            echo "Publishing cancelled"
            exit 1
          fi
          
          # Publish in dependency order
          publish_crate "crates/amalgam-core" "amalgam-core" || exit 1
          publish_crate "crates/amalgam-codegen" "amalgam-codegen" || exit 1
          publish_crate "crates/amalgam-parser" "amalgam-parser" || exit 1
          publish_crate "crates/amalgam-daemon" "amalgam-daemon" || exit 1
          publish_crate "crates/amalgam-cli" "amalgam" || exit 1
          
          echo ""
          echo "🎉 All crates published successfully!"
        '';

        bump-version = pkgs.writeShellScriptBin "bump-version" ''
          new_version=$1
          if [ -z "$new_version" ]; then
            echo "Usage: bump-version <new-version>"
            echo "Example: bump-version 0.1.1"
            exit 1
          fi
          
          echo "📝 Bumping version to $new_version..."
          
          # Update all Cargo.toml files
          for toml in crates/*/Cargo.toml; do
            ${pkgs.gnused}/bin/sed -i "s/^version = \"[^\"]*\"/version = \"$new_version\"/" "$toml"
            ${pkgs.gnused}/bin/sed -i "s/amalgam-core = { version = \"[^\"]*\"/amalgam-core = { version = \"$new_version\"/" "$toml"
            ${pkgs.gnused}/bin/sed -i "s/amalgam-codegen = { version = \"[^\"]*\"/amalgam-codegen = { version = \"$new_version\"/" "$toml"
            ${pkgs.gnused}/bin/sed -i "s/amalgam-parser = { version = \"[^\"]*\"/amalgam-parser = { version = \"$new_version\"/" "$toml"
            ${pkgs.gnused}/bin/sed -i "s/amalgam-daemon = { version = \"[^\"]*\"/amalgam-daemon = { version = \"$new_version\"/" "$toml"
          done
          
          # Update workspace version
          ${pkgs.gnused}/bin/sed -i "s/^version = \"[^\"]*\"/version = \"$new_version\"/" Cargo.toml
          
          echo "✅ Version bumped to $new_version"
          echo "📝 Don't forget to:"
          echo "  1. Update Cargo.lock: cargo update"
          echo "  2. Commit changes: git commit -am 'chore: bump version to $new_version'"
          echo "  3. Tag release: git tag v$new_version"
        '';

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
          buildInputs = [
            # Rust toolchain from fenix
            rustWithComponents

            # Publishing commands (defined above)
            publish-dry-run
            publish-all
            bump-version
          ] ++ (with pkgs; [
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

          ]) ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [];

          shellHook = ''
            echo "🦀 Rust toolchain: $(rustc --version)"
            echo "📦 Cargo version: $(cargo --version)"
            echo ""
            echo "Available commands:"
            echo "  cargo build        - Build the project"
            echo "  cargo test         - Run tests"
            echo "  cargo check        - Check compilation"
            echo "  cargo clippy       - Run linter"
            echo "  cargo fmt          - Format code"
            echo "  cargo watch        - Watch for changes"
            echo "  cargo audit        - Check for vulnerabilities"
            echo ""
            echo "Publishing commands:"
            echo "  publish-dry-run    - Test if all crates are ready to publish"
            echo "  publish-all        - Publish all crates to crates.io (in order)"
            echo "  bump-version X.Y.Z - Bump version in all Cargo.toml files"
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
