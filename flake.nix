{
  description = "amalgam: Generate type-safe Nickel configurations from any schema source";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    crane.url = "github:ipetkov/crane";

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, fenix, crane, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          config.allowUnfree = true;
        };

        # Build Nickel with package support by overriding the derivation
        nickel-with-packages = pkgs.nickel.overrideAttrs (oldAttrs: {
          # Add package-experimental to the build features
          buildFeatures = (oldAttrs.buildFeatures or [ "default" ]) ++ [ "package-experimental" ];

          # Update the pname to distinguish it
          pname = "nickel-with-packages";
        });

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

        # Smart publishing tool that handles everything
        publish = pkgs.writeShellScriptBin "publish" ''
          set -euo pipefail

          # Color output
          RED='\033[0;31m'
          GREEN='\033[0;32m'
          YELLOW='\033[1;33m'
          NC='\033[0m' # No Color

          # Parse arguments - default to publish mode
          MODE="''${1:-publish}"
          SKIP_CHECKS="false"

          while [[ $# -gt 0 ]]; do
            case $1 in
              --skip-checks)
                SKIP_CHECKS="true"
                shift
                ;;
              check|dry-run|publish)
                MODE="$1"
                shift
                ;;
              *)
                shift
                ;;
            esac
          done

          # Function to check if a crate is already published
          check_published() {
            local crate=$1
            local version=$2
            ${rustWithComponents}/bin/cargo search "$crate" --limit 1 | grep -q "^$crate = \"$version\""
          }

          # Function to update dependencies for publishing
          prepare_for_publish() {
            local dir=$1

            # Save original content instead of creating a .bak file (preserve exact content)
            ORIGINAL_CONTENT=$(cat "$dir/Cargo.toml"; printf x)
            ORIGINAL_CONTENT=''${ORIGINAL_CONTENT%x}

            # Replace path dependencies with version-only dependencies
            ${pkgs.gnused}/bin/sed -i 's/amalgam-core = {[^}]*path[^}]*}/amalgam-core = "'"$CURRENT_VERSION"'"/g' "$dir/Cargo.toml"
            ${pkgs.gnused}/bin/sed -i 's/amalgam-codegen = {[^}]*path[^}]*}/amalgam-codegen = "'"$CURRENT_VERSION"'"/g' "$dir/Cargo.toml"
            ${pkgs.gnused}/bin/sed -i 's/amalgam-parser = {[^}]*path[^}]*}/amalgam-parser = "'"$CURRENT_VERSION"'"/g' "$dir/Cargo.toml"
            ${pkgs.gnused}/bin/sed -i 's/amalgam-daemon = {[^}]*path[^}]*}/amalgam-daemon = "'"$CURRENT_VERSION"'"/g' "$dir/Cargo.toml"

            # Also handle cases where they're already version-only deps that need updating
            ${pkgs.gnused}/bin/sed -i 's/amalgam-core = "[^"]*"/amalgam-core = "'"$CURRENT_VERSION"'"/g' "$dir/Cargo.toml"
            ${pkgs.gnused}/bin/sed -i 's/amalgam-codegen = "[^"]*"/amalgam-codegen = "'"$CURRENT_VERSION"'"/g' "$dir/Cargo.toml"
            ${pkgs.gnused}/bin/sed -i 's/amalgam-parser = "[^"]*"/amalgam-parser = "'"$CURRENT_VERSION"'"/g' "$dir/Cargo.toml"
            ${pkgs.gnused}/bin/sed -i 's/amalgam-daemon = "[^"]*"/amalgam-daemon = "'"$CURRENT_VERSION"'"/g' "$dir/Cargo.toml"
          }

          # Function to restore original Cargo.toml
          restore_cargo_toml() {
            local dir=$1
            if [ -n "$ORIGINAL_CONTENT" ]; then
              printf "%s" "$ORIGINAL_CONTENT" > "$dir/Cargo.toml"
            fi
          }

          # Get current version
          CURRENT_VERSION=$(${pkgs.toml2json}/bin/toml2json < Cargo.toml | ${pkgs.jq}/bin/jq -r '.workspace.package.version')
          echo -e "''${GREEN}Current version: $CURRENT_VERSION''${NC}"

          # Note: Version bumping is handled by cargo-release
          # Use: cargo release version patch/minor/major --execute
          # Or: cargo release patch/minor/major --execute (to bump and publish)

          # Define crates in dependency order
          CRATES=(
            "amalgam-core:crates/amalgam-core"
            "amalgam-codegen:crates/amalgam-codegen"
            "amalgam-parser:crates/amalgam-parser"
            "amalgam-daemon:crates/amalgam-daemon"
            "amalgam:crates/amalgam-cli"
          )

          case $MODE in
            check)
              echo -e "\n''${GREEN}Checking publish readiness...''${NC}\n"

              # Run tests unless skipped
              if [ "$SKIP_CHECKS" != "true" ]; then
                echo "Running tests..."
                ${rustWithComponents}/bin/cargo test --workspace || exit 1
                echo "Running clippy..."
                ${rustWithComponents}/bin/cargo clippy --workspace --all-targets || exit 1
                echo "Checking format..."
                ${rustWithComponents}/bin/cargo fmt --check || exit 1
              fi

              # Check each crate
              for crate_info in "''${CRATES[@]}"; do
                IFS=':' read -r crate_name crate_path <<< "$crate_info"
                echo -e "\n''${YELLOW}Checking $crate_name...''${NC}"

                if check_published "$crate_name" "$CURRENT_VERSION"; then
                  echo -e "''${YELLOW}⚠ $crate_name v$CURRENT_VERSION is already published''${NC}"
                else
                  echo -e "''${GREEN}✓ $crate_name v$CURRENT_VERSION is not yet published''${NC}"
                fi

                # Try packaging
                prepare_for_publish "$crate_path"
                (cd "$crate_path" && ${rustWithComponents}/bin/cargo package --list > /dev/null 2>&1)
                if [ $? -eq 0 ]; then
                  echo -e "''${GREEN}✓ $crate_name can be packaged''${NC}"
                else
                  echo -e "''${RED}✗ $crate_name cannot be packaged''${NC}"
                fi
                restore_cargo_toml "$crate_path"
              done
              ;;

            dry-run)
              echo -e "\n''${YELLOW}Running publish dry-run...''${NC}\n"

              for crate_info in "''${CRATES[@]}"; do
                IFS=':' read -r crate_name crate_path <<< "$crate_info"
                echo -e "\n''${YELLOW}Dry-run for $crate_name...''${NC}"

                prepare_for_publish "$crate_path"
                (cd "$crate_path" && ${rustWithComponents}/bin/cargo publish --dry-run --allow-dirty)
                restore_cargo_toml "$crate_path"
              done
              ;;

            publish)
              echo -e "\n''${RED}⚠ PUBLISHING TO CRATES.IO''${NC}\n"

              # Check if logged in by looking for credentials file
              if [ ! -f "$HOME/.cargo/credentials.toml" ] && [ ! -f "$HOME/.cargo/credentials" ]; then
                echo -e "''${RED}Error: Not logged in to crates.io''${NC}"
                echo "Run: cargo login <your-api-token>"
                echo ""
                echo "Get your token from: https://crates.io/settings/tokens"
                exit 1
              fi

              # Confirm
              echo -e "''${YELLOW}This will publish the following crates to crates.io:''${NC}"
              for crate_info in "''${CRATES[@]}"; do
                IFS=':' read -r crate_name crate_path <<< "$crate_info"
                echo "  - $crate_name v$CURRENT_VERSION"
              done
              echo ""
              read -p "Continue? (y/N) " -n 1 -r
              echo
              if [[ ! $REPLY =~ ^[Yy]$ ]]; then
                echo "Cancelled"
                exit 1
              fi

              # Publish each crate
              for crate_info in "''${CRATES[@]}"; do
                IFS=':' read -r crate_name crate_path <<< "$crate_info"

                if check_published "$crate_name" "$CURRENT_VERSION"; then
                  echo -e "''${YELLOW}Skipping $crate_name (already published)''${NC}"
                  continue
                fi

                echo -e "\n''${GREEN}Publishing $crate_name...''${NC}"
                prepare_for_publish "$crate_path"

                if (cd "$crate_path" && ${rustWithComponents}/bin/cargo publish --allow-dirty); then
                  restore_cargo_toml "$crate_path"
                  echo -e "''${GREEN}✓ $crate_name published!''${NC}"

                  # Wait for crates.io to index
                  if [ "$crate_name" != "amalgam" ]; then
                    echo "Waiting 30s for crates.io to index..."
                    sleep 30
                  fi
                else
                  restore_cargo_toml "$crate_path"
                  echo -e "''${RED}Failed to publish $crate_name''${NC}"
                  exit 1
                fi
              done

              echo -e "\n''${GREEN}🎉 All crates published successfully!''${NC}"
              echo -e "''${YELLOW}Don't forget to:''${NC}"
              echo "  - git tag v$CURRENT_VERSION"
              echo "  - git push --tags"
              ;;

            *)
              echo "Usage: publish [check|dry-run]"
              echo ""
              echo "Default behavior: Publishes all crates to crates.io"
              echo ""
              echo "Commands:"
              echo "  (none)   - Publish to crates.io (default)"
              echo "  check    - Check if crates are ready to publish"
              echo "  dry-run  - Run cargo publish --dry-run for all crates"
              echo ""
              echo "Options:"
              echo "  --skip-checks  - Skip tests/clippy/fmt in check mode"
              echo ""
              echo "Note: Use 'release' command first to bump version and create tags"
              echo ""
              echo "Examples:"
              echo "  release patch  # Bump version, test, commit, tag"
              echo "  publish        # Publish to crates.io"
              echo "  git push && git push --tags  # Push everything"
              exit 1
              ;;
          esac
        '';

        # Workspace dependency manager (Python-based for smart error handling)
        workspace-deps = pkgs.writeShellScriptBin "workspace-deps" ''
          exec ${pkgs.python3.withPackages (ps: with ps; [ tomli ])}/bin/python3 ${./nix/packages/workspace-deps/workspace-deps.py} "$@"
        '';

        # Version bump tool (Python-based for reliability)
        version-bump = pkgs.writeShellScriptBin "version-bump" ''
          exec ${pkgs.python3.withPackages (ps: with ps; [ tomli ])}/bin/python3 ${./nix/packages/version-bump/version-bump.py} "$@"
        '';


        # Release helper that validates everything before version bump
        release = pkgs.writeShellScriptBin "release" ''
          set -euo pipefail

          # Color output
          RED='\033[0;31m'
          GREEN='\033[0;32m'
          YELLOW='\033[1;33m'
          NC='\033[0m' # No Color

          BUMP_TYPE="''${1:-patch}"

          echo -e "''${YELLOW}Starting release process for $BUMP_TYPE version bump...''${NC}"
          echo ""

          # Step 1: Run CI checks
          echo -e "''${YELLOW}Step 1: Running CI checks...''${NC}"
          if ! ci; then
            echo -e "''${RED}✗ CI checks failed! Fix issues before releasing.''${NC}"
            exit 1
          fi
          echo ""

          # Step 2: Check snapshot tests
          echo -e "''${YELLOW}Step 2: Checking snapshot tests...''${NC}"
          if ! ${rustWithComponents}/bin/cargo insta test; then
            echo -e "''${RED}✗ Snapshot tests failed! Review with 'cargo insta review'.''${NC}"
            exit 1
          fi
          echo -e "''${GREEN}✓ Snapshot tests passed''${NC}"
          echo ""

          # Step 3: Get current version
          CURRENT_VERSION=$(${pkgs.toml2json}/bin/toml2json < Cargo.toml | ${pkgs.jq}/bin/jq -r '.workspace.package.version')
          echo -e "''${GREEN}Current version: $CURRENT_VERSION''${NC}"

          # Step 4: Bump version
          echo -e "''${YELLOW}Step 4: Bumping $BUMP_TYPE version...''${NC}"
          if ! version-bump $BUMP_TYPE; then
            echo -e "''${RED}✗ Failed to bump version!''${NC}"
            exit 1
          fi

          # Get the new version
          NEW_VERSION=$(${pkgs.toml2json}/bin/toml2json < Cargo.toml | ${pkgs.jq}/bin/jq -r '.workspace.package.version')
          echo -e "''${GREEN}✓ Version bumped to $NEW_VERSION''${NC}"
          echo ""

          # Step 5: Update Cargo.lock
          echo -e "''${YELLOW}Step 5: Updating Cargo.lock...''${NC}"
          ${rustWithComponents}/bin/cargo update
          echo -e "''${GREEN}✓ Cargo.lock updated''${NC}"
          echo ""

          # Step 6: Check publish readiness
          echo -e "''${YELLOW}Step 6: Checking publish readiness...''${NC}"
          if ! publish check --skip-checks; then
            echo -e "''${RED}✗ Not ready to publish!''${NC}"
            exit 1
          fi
          echo -e "''${GREEN}✓ Ready to publish''${NC}"
          echo ""

          # Step 7: Commit changes
          echo -e "''${YELLOW}Step 7: Committing version bump...''${NC}"
          ${pkgs.git}/bin/git add -A
          ${pkgs.git}/bin/git commit -m "release: v$NEW_VERSION"
          echo -e "''${GREEN}✓ Changes committed''${NC}"
          echo ""

          # Step 8: Tag the release
          echo -e "''${YELLOW}Step 8: Creating git tag...''${NC}"
          ${pkgs.git}/bin/git tag "v$NEW_VERSION"
          echo -e "''${GREEN}✓ Tagged as v$NEW_VERSION''${NC}"
          echo ""

          echo -e "''${GREEN}🎉 Release v$NEW_VERSION prepared successfully!''${NC}"
          echo ""
          echo -e "''${YELLOW}Next steps:''${NC}"
          echo "  1. Review the changes: git diff HEAD~1"
          echo "  2. Publish to crates.io: publish"
          echo "  3. Push to GitHub: git push && git push --tags"
          echo ""
          echo -e "''${YELLOW}To undo:''${NC}"
          echo "  git reset --hard HEAD~1"
          echo "  git tag -d v$NEW_VERSION"
        '';

        # CI runner - the primary test command
        ci = pkgs.writeShellScriptBin "ci" ''
          set -euo pipefail

          echo "Running CI test suite..."
          echo ""

          # Ensure we're in local dev mode
          workspace-deps local > /dev/null 2>&1

          echo "1. Running cargo check..."
          ${rustWithComponents}/bin/cargo check --workspace --all-targets

          echo ""
          echo "2. Running tests..."
          ${rustWithComponents}/bin/cargo test --workspace

          echo ""
          echo "3. Running clippy..."
          ${rustWithComponents}/bin/cargo clippy --workspace --all-targets -- --deny warnings

          echo ""
          echo "4. Checking formatting..."
          ${rustWithComponents}/bin/cargo fmt --check

          echo ""
          echo "✓ All CI checks passed!"
        '';

        # Auto-fix command for formatting and clippy
        fix = pkgs.writeShellScriptBin "fix" ''
          set -euo pipefail

          echo "🔧 Auto-fixing code issues..."
          echo ""

          # Ensure we're in local dev mode
          workspace-deps local > /dev/null 2>&1

          echo "1. Formatting code..."
          ${rustWithComponents}/bin/cargo fmt --all

          echo ""
          echo "2. Applying clippy fixes..."
          ${rustWithComponents}/bin/cargo clippy --workspace --all-targets --fix --allow-dirty --allow-staged

          echo ""
          echo "3. Checking if everything is fixed..."
          if ci > /dev/null 2>&1; then
            echo "✓ All issues fixed!"
          else
            echo "⚠ Some issues may require manual intervention"
            echo "Run 'ci' to see remaining issues"
          fi
        '';

        # Smart manifest-based regeneration
        regenerate-examples = pkgs.writeShellScriptBin "regenerate-examples" ''
          set -euo pipefail

          if [ ! -f ".amalgam-manifest.toml" ]; then
            echo "❌ No .amalgam-manifest.toml found in current directory"
            echo "Please ensure you're in the amalgam project root."
            exit 1
          fi

          echo "🧠 Smart manifest-based regeneration with content tracking..."
          ${rustWithComponents}/bin/cargo run --release --bin amalgam -- generate-from-manifest

          echo ""
          echo "✅ Smart regeneration complete!"
        '';

        # Custom source filter that includes test fixtures
        src = pkgs.lib.cleanSourceWith {
          src = ./.;
          filter = path: type:
            # Include standard cargo files
            (craneLib.filterCargoSources path type) ||
            # Include test fixture files
            (builtins.match ".*/tests/fixtures/.*\\.yaml$" path != null) ||
            # Include test snapshot files
            (builtins.match ".*/tests/snapshots/.*\\.snap$" path != null) ||
            # Include any other test resources
            (builtins.match ".*/tests/.*\\.(toml|json|yaml|ncl)$" path != null);
        };

        # Build dependencies only
        cargoArtifacts = craneLib.buildDepsOnly {
          inherit src;

          # Build-time dependencies
          buildInputs = with pkgs; [
            openssl
            pkg-config
          ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [];
        };

        # Build the actual crate
        amalgam = craneLib.buildPackage {
          inherit cargoArtifacts src;

          buildInputs = with pkgs; [
            openssl
            pkg-config
          ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [];
        };

        # Docker/OCI image builders
        dockerImages = import ./nix/packages/docker-image {
          inherit pkgs amalgam;
          lib = pkgs.lib;
          nickel = nickel-with-packages;
          generated-packages = null;  # Will be populated by CI
        };

      in
      {
        packages = {
          default = amalgam;
          amalgam = amalgam;
          nickel-with-packages = nickel-with-packages;

          # Docker images
          amalgam-image = dockerImages.amalgamImage;
          packages-image = dockerImages.packagesImage;
          amalgam-layered = dockerImages.amalgamLayeredImage;

          # Helper scripts for pushing images
          push-to-registry = dockerImages.pushToRegistry;
          push-with-skopeo = dockerImages.pushWithSkopeo;
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

            # Smart commands
            ci
            fix
            release
            publish
            workspace-deps
            version-bump
            regenerate-examples
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
            cargo-insta      # snapshot testing

            # For WASM builds
            wasm-pack
            wasm-bindgen-cli
            binaryen  # WASM optimizer (includes wasm-opt)
            wasmtime  # WASM runtime for testing

            # Python for complex scripts
            python3
            python311Packages.rich
            python311Packages.click
            python311Packages.toml

            # General dev tools
            tokei      # code statistics

            # For working with schemas
            jq
            yq
            toml2json

            # For Kubernetes integration
            kubectl
            kind

            # For publishing Nickel packages (experimental)
            nickel-with-packages

          ]) ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [];

          shellHook = ''
            echo "🦀 Amalgam Development Environment"
            echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
            VERSION=$(${pkgs.toml2json}/bin/toml2json < Cargo.toml 2>/dev/null | ${pkgs.jq}/bin/jq -r '.workspace.package.version' || echo "unknown")
            echo "Version $VERSION"
            echo ""
            echo "Essential Commands:"
            echo "  ci                   - Run complete test suite (tests, clippy, fmt)"
            echo "  fix                  - Auto-fix formatting and clippy issues"
            echo "  regenerate-examples  - Rebuild and regenerate example CRDs"
            echo "  release patch        - Bump version and create release"
            echo "  publish              - Publish to crates.io"
            echo ""
            echo "Workflow:"
            echo "  1. fix                           # Auto-fix issues"
            echo "  2. ci                            # Validate everything"
            echo "  3. release [patch|minor|major]   # Create release"
            echo "  4. publish                       # Publish to crates.io"
            echo "  5. git push && git push --tags  # Push to GitHub"
            echo ""
            echo "Other Commands:"
            echo "  workspace-deps local - Switch to local development (path deps)"
            echo "  workspace-deps remote - Switch to publish mode (crates.io deps)"
            echo "  cargo watch          - Watch for changes"
            echo "  cargo insta review   - Review snapshot test changes"
            echo ""

            # Ensure we're in local dev mode by default
            workspace-deps local 2>/dev/null || true
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
            inherit cargoArtifacts src;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          };

          amalgam-fmt = craneLib.cargoFmt {
            inherit src;
          };

          amalgam-tests = craneLib.cargoTest {
            inherit cargoArtifacts src;
          };
        };
      });
}
