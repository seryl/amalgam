#!/usr/bin/env python3
"""
Workspace dependency manager for Amalgam.

This tool manages the workspace dependencies in Cargo.toml files,
automatically handling the switch between local path dependencies
(for development) and version-only dependencies (for publishing).

It dynamically reads workspace dependencies from Cargo.toml, so it
automatically handles any workspace crates without hardcoding.
"""

import subprocess
import sys
import tomllib
from pathlib import Path
from typing import Dict, List, Optional, Set
from enum import Enum


class DependencyMode(Enum):
    LOCAL = "local"
    REMOTE = "remote"
    MIXED = "mixed"  # Some local, some remote
    UNKNOWN = "unknown"


class SmartError(Exception):
    """Smart error handling with actionable suggestions."""

    def __init__(self, error: str, context: str, suggestions: List[str]):
        super().__init__(error)
        self.error = error
        self.context = context
        self.suggestions = suggestions

    def format_for_human(self) -> str:
        return f"""
❌ {self.error}
📋 Context: {self.context}
💡 Suggestions:
{chr(10).join(f"  • {s}" for s in self.suggestions)}
        """


class WorkspaceDepsManager:
    """Manages Cargo workspace dependencies intelligently."""

    def __init__(self, root: Path = Path.cwd()):
        self.root = root
        self.workspace_toml = root / "Cargo.toml"
        self.crates_dir = root / "crates"
        self._workspace_deps = None
        self._workspace_version = None

    @property
    def workspace_deps(self) -> Set[str]:
        """Dynamically discover workspace dependencies."""
        if self._workspace_deps is None:
            self._workspace_deps = self.discover_workspace_deps()
        return self._workspace_deps

    @property
    def workspace_version(self) -> str:
        """Get the current workspace version."""
        if self._workspace_version is None:
            with open(self.workspace_toml, "rb") as f:
                data = tomllib.load(f)
                self._workspace_version = data["workspace"]["package"]["version"]
        return self._workspace_version

    def discover_workspace_deps(self) -> Set[str]:
        """Discover all workspace dependencies from Cargo.toml."""
        workspace_deps = set()

        with open(self.workspace_toml, "rb") as f:
            data = tomllib.load(f)

            # Get dependencies from [workspace.dependencies]
            if "workspace" in data and "dependencies" in data["workspace"]:
                for dep_name, dep_info in data["workspace"]["dependencies"].items():
                    # Check if this is an internal workspace dependency
                    # by looking for path references to crates/
                    if isinstance(dep_info, dict) and "path" in dep_info:
                        if dep_info["path"].startswith("crates/"):
                            workspace_deps.add(dep_name)
                    # Also check if the dependency name matches our workspace pattern
                    elif dep_name.startswith("amalgam-"):
                        workspace_deps.add(dep_name)

        # Also discover by looking at what's actually in crates/
        for crate_dir in self.crates_dir.iterdir():
            if not crate_dir.is_dir():
                continue

            cargo_toml = crate_dir / "Cargo.toml"
            if cargo_toml.exists():
                with open(cargo_toml, "rb") as f:
                    try:
                        crate_data = tomllib.load(f)
                        if "package" in crate_data and "name" in crate_data["package"]:
                            crate_name = crate_data["package"]["name"]
                            if crate_name.startswith("amalgam"):
                                workspace_deps.add(crate_name)
                    except Exception:
                        # Skip malformed Cargo.toml files
                        continue

        return workspace_deps

    def get_crate_dir_for_dep(self, dep_name: str) -> str:
        """Get the crate directory name for a dependency."""
        # Handle special cases
        if dep_name == "amalgam":
            return "amalgam-cli"
        # Standard pattern: amalgam-core -> amalgam-core
        return dep_name

    def detect_current_mode(self) -> DependencyMode:
        """Detect the current dependency mode by checking Cargo.toml files."""
        has_local = False
        has_remote = False

        for crate_dir in self.crates_dir.iterdir():
            if not crate_dir.is_dir():
                continue

            cargo_toml = crate_dir / "Cargo.toml"
            if not cargo_toml.exists():
                continue

            content = cargo_toml.read_text()

            # Check for path dependencies
            for dep in self.workspace_deps:
                # Look for various patterns
                if f'{dep} = {{ version = ' in content and 'path = ' in content:
                    has_local = True
                elif f'{dep}.workspace = true' in content:
                    # Check workspace definition
                    pass
                elif f'{dep} = "' in content:
                    has_remote = True

        # Also check workspace dependencies
        workspace_content = self.workspace_toml.read_text()
        for dep in self.workspace_deps:
            if f'{dep} = {{ version = ' in workspace_content and 'path = ' in workspace_content:
                has_local = True
            elif f'{dep} = {{ version = ' in workspace_content and 'path = ' not in workspace_content:
                has_remote = True

        if has_local and has_remote:
            return DependencyMode.MIXED
        elif has_local:
            return DependencyMode.LOCAL
        elif has_remote:
            return DependencyMode.REMOTE
        else:
            return DependencyMode.UNKNOWN

    def update_workspace_dependencies(self, mode: DependencyMode) -> None:
        """Update all workspace dependencies to the specified mode."""
        version = self.workspace_version

        # Read the workspace Cargo.toml
        with open(self.workspace_toml, "rb") as f:
            workspace_data = tomllib.load(f)

        # Update [workspace.dependencies] section
        if "workspace" in workspace_data and "dependencies" in workspace_data["workspace"]:
            deps_to_update = {}

            for dep_name in self.workspace_deps:
                if dep_name in workspace_data["workspace"]["dependencies"]:
                    crate_dir = self.get_crate_dir_for_dep(dep_name)

                    if mode == DependencyMode.LOCAL:
                        # Add path for local development
                        deps_to_update[dep_name] = {
                            "version": version,
                            "path": f"crates/{crate_dir}"
                        }
                    else:
                        # Remove path for publishing
                        deps_to_update[dep_name] = {
                            "version": version
                        }

            # Now update the actual file
            self.update_cargo_toml_deps(self.workspace_toml, deps_to_update, is_workspace=True)

        # Update all crate dependencies
        updated_crates = []
        for crate_dir in self.crates_dir.iterdir():
            if not crate_dir.is_dir():
                continue

            cargo_toml = crate_dir / "Cargo.toml"
            if not cargo_toml.exists():
                continue

            # Check if this crate uses any workspace dependencies
            with open(cargo_toml, "rb") as f:
                try:
                    crate_data = tomllib.load(f)
                except Exception:
                    continue

            deps_to_update = {}

            # Check regular dependencies
            if "dependencies" in crate_data:
                for dep_name in self.workspace_deps:
                    if dep_name in crate_data["dependencies"]:
                        dep_value = crate_data["dependencies"][dep_name]
                        # Only update if it's not using workspace = true
                        if not (isinstance(dep_value, dict) and dep_value.get("workspace") is True):
                            if mode == DependencyMode.LOCAL:
                                crate_dir_name = self.get_crate_dir_for_dep(dep_name)
                                deps_to_update[dep_name] = {
                                    "version": version,
                                    "path": f"../{crate_dir_name}"
                                }
                            else:
                                deps_to_update[dep_name] = version

            # Check dev-dependencies too
            if "dev-dependencies" in crate_data:
                for dep_name in self.workspace_deps:
                    if dep_name in crate_data["dev-dependencies"]:
                        dep_value = crate_data["dev-dependencies"][dep_name]
                        if not (isinstance(dep_value, dict) and dep_value.get("workspace") == True):
                            if mode == DependencyMode.LOCAL:
                                crate_dir_name = self.get_crate_dir_for_dep(dep_name)
                                deps_to_update[f"dev.{dep_name}"] = {
                                    "version": version,
                                    "path": f"../{crate_dir_name}"
                                }
                            else:
                                deps_to_update[f"dev.{dep_name}"] = version

            if deps_to_update:
                self.update_cargo_toml_deps(cargo_toml, deps_to_update, is_workspace=False)
                updated_crates.append(crate_dir.name)

        if updated_crates:
            print(f"✅ Updated dependencies in: {', '.join(updated_crates)}")

    def update_cargo_toml_deps(self, toml_path: Path, deps_to_update: Dict, is_workspace: bool) -> None:
        """Update specific dependencies in a Cargo.toml file."""
        content = toml_path.read_text()
        lines = content.splitlines(keepends=True)

        updated_lines = []
        current_section = None

        for line in lines:
            # Track which section we're in
            if line.strip().startswith("["):
                if "[workspace.dependencies]" in line:
                    current_section = "workspace.dependencies"
                elif "[dependencies]" in line:
                    current_section = "dependencies"
                elif "[dev-dependencies]" in line:
                    current_section = "dev-dependencies"
                else:
                    current_section = None

            # Check if we need to update this line
            line_updated = False
            if current_section:
                for dep_name, dep_value in deps_to_update.items():
                    # Handle dev dependencies special case
                    actual_dep = dep_name.replace("dev.", "") if dep_name.startswith("dev.") else dep_name

                    # Only update if we're in the right section
                    if dep_name.startswith("dev.") and current_section != "dev-dependencies":
                        continue
                    if not dep_name.startswith("dev.") and current_section == "dev-dependencies":
                        continue

                    if actual_dep in line and "workspace = true" not in line:
                        # Update the line
                        if isinstance(dep_value, dict):
                            # Check if it has a path (local mode) or just version (remote mode)
                            if "path" in dep_value:
                                # Local mode with path
                                new_line = f'{actual_dep} = {{ version = "{dep_value["version"]}", path = "{dep_value["path"]}" }}\n'
                            else:
                                # Remote mode with just version
                                new_line = f'{actual_dep} = "{dep_value["version"]}"\n'
                        else:
                            # Remote mode (simple string version)
                            new_line = f'{actual_dep} = "{dep_value}"\n'

                        updated_lines.append(new_line)
                        line_updated = True
                        break

            if not line_updated:
                updated_lines.append(line)

        toml_path.write_text("".join(updated_lines))

    def run_cargo_update(self) -> None:
        """Run cargo update to ensure lock file is in sync."""
        try:
            subprocess.run(["cargo", "update"], check=True, capture_output=True)
            print("✅ Cargo.lock updated")
        except subprocess.CalledProcessError as e:
            raise SmartError(
                error="Failed to update Cargo.lock",
                context="After changing dependency mode",
                suggestions=[
                    "Check if all dependencies are valid",
                    "Try running 'cargo check' to see detailed errors",
                    "Ensure you're in the project root directory"
                ]
            )

    def switch_mode(self, target_mode: Optional[DependencyMode] = None) -> None:
        """Switch to the specified dependency mode, or toggle if not specified."""
        current_mode = self.detect_current_mode()

        if target_mode is None:
            # Toggle between local and remote
            if current_mode == DependencyMode.LOCAL:
                target_mode = DependencyMode.REMOTE
            else:
                target_mode = DependencyMode.LOCAL

        print(f"📦 Workspace version: {self.workspace_version}")
        print(f"📦 Current mode: {current_mode.value}")
        print(f"📦 Workspace deps: {', '.join(sorted(self.workspace_deps))}")

        if current_mode == target_mode:
            print(f"✅ Already in {target_mode.value} mode")
            return

        print(f"🔄 Switching to {target_mode.value} mode...")

        try:
            self.update_workspace_dependencies(target_mode)
            self.run_cargo_update()

            print(f"✅ Successfully switched to {target_mode.value} mode")

            if target_mode == DependencyMode.LOCAL:
                print("📝 Using local path dependencies (for development)")
            else:
                print("📝 Using crates.io dependencies (for publishing)")
                print("⚠️  Note: Ensure all dependencies are published before testing")

        except Exception as e:
            if isinstance(e, SmartError):
                print(e.format_for_human())
                sys.exit(1)
            else:
                error = SmartError(
                    error=f"Unexpected error: {str(e)}",
                    context="While switching dependency modes",
                    suggestions=[
                        "Check if all Cargo.toml files are valid",
                        "Ensure you have write permissions",
                        "Try running with --debug for more details"
                    ]
                )
                print(error.format_for_human())
                sys.exit(1)


def main():
    """Main entry point."""
    import argparse

    parser = argparse.ArgumentParser(
        description="Manage Amalgam workspace dependencies intelligently"
    )
    parser.add_argument(
        "mode",
        nargs="?",
        choices=["local", "remote", "status"],
        help="Target mode (local/remote) or status to check current mode"
    )
    parser.add_argument(
        "--debug",
        action="store_true",
        help="Enable debug output"
    )

    args = parser.parse_args()

    manager = WorkspaceDepsManager()

    if args.mode == "status" or args.mode is None:
        current = manager.detect_current_mode()
        print(f"📦 Workspace version: {manager.workspace_version}")
        print(f"📦 Dependency mode: {current.value}")
        deps_list = ', '.join(sorted(manager.workspace_deps))
        print(f"📦 Workspace deps found: {deps_list}")

        if current == DependencyMode.LOCAL:
            print("📝 Using local path dependencies (for development)")
        elif current == DependencyMode.REMOTE:
            print("📝 Using crates.io dependencies (for publishing)")
        elif current == DependencyMode.MIXED:
            print("⚠️  Mixed mode detected - some deps are local, some remote")
            print("💡 Run 'workspace-deps local' or 'workspace-deps remote'")
    else:
        if args.mode == "local":
            target = DependencyMode.LOCAL
        else:
            target = DependencyMode.REMOTE
        manager.switch_mode(target)


if __name__ == "__main__":
    main()
