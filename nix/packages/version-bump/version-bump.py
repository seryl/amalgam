#!/usr/bin/env python3
"""
Version bump tool for Amalgam workspace.

This tool handles semantic version bumping for the entire workspace,
ensuring all internal dependencies are updated correctly.
"""

import sys
import re
import subprocess
import tomllib
from pathlib import Path
from typing import Tuple, Optional
from enum import Enum


class BumpType(Enum):
    MAJOR = "major"
    MINOR = "minor"
    PATCH = "patch"


class SmartError:
    """Smart error handling with actionable suggestions."""

    def __init__(self, error: str, context: str, suggestions: list[str]):
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


def parse_version(version: str) -> Tuple[int, int, int]:
    """Parse a semantic version string into major, minor, patch components."""
    match = re.match(r"^(\d+)\.(\d+)\.(\d+)", version)
    if not match:
        raise ValueError(f"Invalid version format: {version}")
    return int(match.group(1)), int(match.group(2)), int(match.group(3))


def bump_version(version: str, bump_type: BumpType) -> str:
    """Bump a semantic version according to the specified type."""
    major, minor, patch = parse_version(version)

    if bump_type == BumpType.MAJOR:
        return f"{major + 1}.0.0"
    elif bump_type == BumpType.MINOR:
        return f"{major}.{minor + 1}.0"
    elif bump_type == BumpType.PATCH:
        return f"{major}.{minor}.{patch + 1}"
    else:
        raise ValueError(f"Unknown bump type: {bump_type}")


class VersionBumper:
    """Manages version bumping for the Amalgam workspace."""

    def __init__(self, root: Path = Path.cwd()):
        self.root = root
        self.workspace_toml = root / "Cargo.toml"
        self.crates_dir = root / "crates"

    def get_current_version(self) -> str:
        """Get the current workspace version."""
        with open(self.workspace_toml, "rb") as f:
            data = tomllib.load(f)
            return data["workspace"]["package"]["version"]

    def update_workspace_version(self, new_version: str) -> None:
        """Update the workspace version in Cargo.toml."""
        content = self.workspace_toml.read_text()

        # Update workspace version
        pattern = r'(\[workspace\.package\][^\[]*version = ")[^"]+(")'
        content = re.sub(pattern, rf'\g<1>{new_version}\g<2>', content)

        # Update workspace dependency versions
        workspace_deps = [
            "amalgam-core",
            "amalgam-parser",
            "amalgam-codegen",
            "amalgam-daemon",
            "amalgam",
        ]

        for dep in workspace_deps:
            # Update in [workspace.dependencies]
            pattern = rf'({dep} = {{ version = ")[^"]+(")'
            content = re.sub(pattern, rf'\g<1>{new_version}\g<2>', content)

        self.workspace_toml.write_text(content)


    def bump(self, bump_type: BumpType) -> str:
        """Bump the workspace version and update all references."""
        try:
            current_version = self.get_current_version()
            new_version = bump_version(current_version, bump_type)

            print(f"📦 Current version: {current_version}")
            print(f"🔄 Bumping to: {new_version}")

            # Update Cargo.toml
            self.update_workspace_version(new_version)
            print(f"✅ Updated Cargo.toml")

            # Update Cargo.lock
            print(f"🔄 Updating Cargo.lock...")
            try:
                subprocess.run(
                    ["cargo", "update", "--workspace"],
                    check=True,
                    capture_output=True,
                    text=True
                )
                print(f"✅ Updated Cargo.lock")
            except subprocess.CalledProcessError as e:
                print(f"⚠️  Warning: Failed to update Cargo.lock: {e.stderr}")
                print(f"   You may need to run 'cargo update --workspace' manually")

            return new_version

        except Exception as e:
            error = SmartError(
                error=f"Failed to bump version: {str(e)}",
                context="While updating workspace version",
                suggestions=[
                    "Check if Cargo.toml is valid",
                    "Ensure you have write permissions",
                    "Verify the current version format is correct",
                ],
            )
            print(error.format_for_human())
            sys.exit(1)


def main():
    """Main entry point."""
    import argparse

    parser = argparse.ArgumentParser(description="Bump Amalgam workspace version")
    parser.add_argument(
        "bump_type",
        choices=["major", "minor", "patch"],
        help="Type of version bump",
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Show what would be changed without making changes",
    )

    args = parser.parse_args()

    bumper = VersionBumper()

    if args.dry_run:
        current = bumper.get_current_version()
        new = bump_version(current, BumpType(args.bump_type))
        print(f"Would bump from {current} to {new}")
    else:
        new_version = bumper.bump(BumpType(args.bump_type))
        print(f"🎉 Version bumped to {new_version}")
        print(f"")
        print(f"Next steps:")
        print(f"  1. Commit changes: git commit -am 'release: v{new_version}'")
        print(f"  2. Tag release: git tag v{new_version}")
        print(f"  3. Push: git push && git push --tags")


if __name__ == "__main__":
    main()