#!/usr/bin/env python3
"""
Package testing and publishing tools for Amalgam-generated Nickel packages.

This tool helps with:
1. Testing packages with different dependency modes (Git, Path, Index)
2. Preparing packages for publishing to nickel-mine
3. Validating package structures
4. Creating test applications
"""

import argparse
import json
import os
import subprocess
import sys
import tempfile
from dataclasses import dataclass
from pathlib import Path
from typing import Dict, List, Optional, Tuple
import shutil
import re


@dataclass
class PackageInfo:
    """Information about a Nickel package."""
    name: str
    path: Path
    version: str
    dependencies: Dict[str, str]
    

@dataclass
class TestResult:
    """Result of a package test."""
    success: bool
    package: str
    test_type: str
    output: str
    error: Optional[str] = None


class PackageManager:
    """Manages Amalgam-generated Nickel packages."""
    
    def __init__(self, workspace_root: Path):
        self.workspace_root = workspace_root
        self.packages_dir = workspace_root / "examples" / "pkgs"
        self.nickel_bin = self._find_nickel()
        
    def _find_nickel(self) -> str:
        """Find the nickel binary."""
        # Try local build first
        local_nickel = self.workspace_root / "nickel" / "target" / "release" / "nickel"
        if local_nickel.exists():
            return str(local_nickel)
        
        # Try system nickel
        result = subprocess.run(["which", "nickel"], capture_output=True, text=True)
        if result.returncode == 0:
            return result.stdout.strip()
        
        raise RuntimeError("Nickel binary not found. Please install Nickel or build it locally.")
    
    def list_packages(self) -> List[PackageInfo]:
        """List all available packages."""
        packages = []
        
        if not self.packages_dir.exists():
            print(f"Warning: Packages directory {self.packages_dir} does not exist")
            return packages
            
        for pkg_dir in self.packages_dir.iterdir():
            if pkg_dir.is_dir():
                manifest_path = pkg_dir / "Nickel-pkg.ncl"
                if manifest_path.exists():
                    info = self._read_package_info(pkg_dir)
                    if info:
                        packages.append(info)
        
        return packages
    
    def _read_package_info(self, pkg_dir: Path) -> Optional[PackageInfo]:
        """Read package information from manifest."""
        manifest_path = pkg_dir / "Nickel-pkg.ncl"
        
        try:
            # Use nickel to evaluate the manifest
            result = subprocess.run(
                [self.nickel_bin, "export", str(manifest_path)],
                capture_output=True,
                text=True,
                cwd=str(pkg_dir)
            )
            
            if result.returncode != 0:
                print(f"Warning: Failed to read manifest for {pkg_dir.name}: {result.stderr}")
                return None
            
            manifest = json.loads(result.stdout)
            
            return PackageInfo(
                name=manifest.get("name", pkg_dir.name),
                path=pkg_dir,
                version=manifest.get("version", "0.1.0"),
                dependencies=manifest.get("dependencies", {})
            )
        except Exception as e:
            print(f"Warning: Error reading manifest for {pkg_dir.name}: {e}")
            return None
    
    def test_package(self, package: str, mode: str = "path") -> TestResult:
        """Test a package with specified dependency mode."""
        pkg_info = None
        for info in self.list_packages():
            if info.name == package or info.path.name == package:
                pkg_info = info
                break
        
        if not pkg_info:
            return TestResult(
                success=False,
                package=package,
                test_type=mode,
                output="",
                error=f"Package {package} not found"
            )
        
        # Create a test directory
        with tempfile.TemporaryDirectory() as tmpdir:
            test_dir = Path(tmpdir)
            
            # Create test manifest based on mode
            if mode == "path":
                manifest = self._create_path_manifest(pkg_info)
            elif mode == "git":
                manifest = self._create_git_manifest(pkg_info)
            else:
                manifest = self._create_index_manifest(pkg_info)
            
            manifest_path = test_dir / "Nickel-pkg.ncl"
            manifest_path.write_text(manifest)
            
            # Create simple test file
            test_file = test_dir / "test.ncl"
            test_file.write_text(f'''
let pkg = import "{pkg_info.name}" in
{{
    package_loaded = pkg != null,
    package_name = "{pkg_info.name}",
}}
''')
            
            # Run the test
            result = subprocess.run(
                [self.nickel_bin, "eval", str(test_file)],
                capture_output=True,
                text=True,
                cwd=str(test_dir)
            )
            
            return TestResult(
                success=result.returncode == 0,
                package=package,
                test_type=mode,
                output=result.stdout,
                error=result.stderr if result.returncode != 0 else None
            )
    
    def _create_path_manifest(self, pkg_info: PackageInfo) -> str:
        """Create a manifest with Path dependencies."""
        deps = {}
        for dep_name, dep_spec in pkg_info.dependencies.items():
            deps[dep_name] = f'\'Path "{pkg_info.path}/../{dep_name}",'
        
        return f'''{{
    name = "test-{pkg_info.name}",
    version = "0.1.0",
    dependencies = {{
        {pkg_info.name} = 'Path "{pkg_info.path}",
        {self._format_deps(deps)}
    }},
}} | std.package.Manifest'''
    
    def _create_git_manifest(self, pkg_info: PackageInfo) -> str:
        """Create a manifest with Git dependencies."""
        # This assumes packages are in a git repo
        # You'll need to adjust the URL to your actual repo
        git_url = "https://github.com/seryl/amalgam"  # Replace with your repo
        
        deps = {}
        for dep_name, dep_spec in pkg_info.dependencies.items():
            deps[dep_name] = f'''\'Git {{
                url = "{git_url}",
                ref = "main",
                path = "examples/pkgs/{dep_name}"
            }},'''
        
        return f'''{{
    name = "test-{pkg_info.name}",
    version = "0.1.0",
    dependencies = {{
        {pkg_info.name} = 'Git {{
            url = "{git_url}",
            ref = "main",
            path = "examples/pkgs/{pkg_info.name}"
        }},
        {self._format_deps(deps)}
    }},
}} | std.package.Manifest'''
    
    def _create_index_manifest(self, pkg_info: PackageInfo) -> str:
        """Create a manifest with Index dependencies."""
        # This assumes packages are published to nickel-mine
        deps = {}
        for dep_name, dep_spec in pkg_info.dependencies.items():
            deps[dep_name] = f'''\'Index {{
                package = "github:amalgam/{dep_name}",
                version = "0.1.0"
            }},'''
        
        return f'''{{
    name = "test-{pkg_info.name}",
    version = "0.1.0", 
    dependencies = {{
        {pkg_info.name} = 'Index {{
            package = "github:amalgam/{pkg_info.name}",
            version = "{pkg_info.version}"
        }},
        {self._format_deps(deps)}
    }},
}} | std.package.Manifest'''
    
    def _format_deps(self, deps: Dict[str, str]) -> str:
        """Format dependencies for manifest."""
        return "\n        ".join(f"{name} = {spec}" for name, spec in deps.items())
    
    def validate_package(self, package: str) -> TestResult:
        """Validate a package structure and syntax."""
        pkg_info = None
        for info in self.list_packages():
            if info.name == package or info.path.name == package:
                pkg_info = info
                break
        
        if not pkg_info:
            return TestResult(
                success=False,
                package=package,
                test_type="validate",
                output="",
                error=f"Package {package} not found"
            )
        
        # Check manifest
        manifest_result = subprocess.run(
            [self.nickel_bin, "typecheck", str(pkg_info.path / "Nickel-pkg.ncl")],
            capture_output=True,
            text=True
        )
        
        if manifest_result.returncode != 0:
            return TestResult(
                success=False,
                package=package,
                test_type="validate",
                output="",
                error=f"Manifest validation failed: {manifest_result.stderr}"
            )
        
        # Check all .ncl files
        errors = []
        for ncl_file in pkg_info.path.rglob("*.ncl"):
            result = subprocess.run(
                [self.nickel_bin, "typecheck", str(ncl_file)],
                capture_output=True,
                text=True
            )
            if result.returncode != 0:
                errors.append(f"{ncl_file.relative_to(pkg_info.path)}: {result.stderr}")
        
        if errors:
            return TestResult(
                success=False,
                package=package,
                test_type="validate",
                output="",
                error="\n".join(errors)
            )
        
        return TestResult(
            success=True,
            package=package,
            test_type="validate",
            output=f"Package {package} is valid",
            error=None
        )
    
    def prepare_for_publishing(self, package: str, target_dir: Path) -> bool:
        """Prepare a package for publishing to nickel-mine."""
        pkg_info = None
        for info in self.list_packages():
            if info.name == package or info.path.name == package:
                pkg_info = info
                break
        
        if not pkg_info:
            print(f"Error: Package {package} not found")
            return False
        
        # Create target directory
        target_dir.mkdir(parents=True, exist_ok=True)
        pkg_target = target_dir / pkg_info.name
        
        # Copy package files
        if pkg_target.exists():
            shutil.rmtree(pkg_target)
        shutil.copytree(pkg_info.path, pkg_target)
        
        # Update manifest for publishing
        manifest_path = pkg_target / "Nickel-pkg.ncl"
        manifest_content = manifest_path.read_text()
        
        # Replace Path dependencies with Index dependencies
        # This is a simple regex replacement - might need refinement
        manifest_content = re.sub(
            r"'Path\s+\"[^\"]+\"",
            lambda m: self._path_to_index(m.group(0)),
            manifest_content
        )
        
        manifest_path.write_text(manifest_content)
        
        print(f"Package {package} prepared for publishing at {pkg_target}")
        return True
    
    def _path_to_index(self, path_dep: str) -> str:
        """Convert a Path dependency to an Index dependency."""
        # Extract package name from path
        match = re.search(r'"([^/]+)"', path_dep)
        if match:
            pkg_name = match.group(1)
            # For now, assume all packages are under amalgam org
            return f"'Index {{ package = \"github:amalgam/{pkg_name}\", version = \"0.1.0\" }}"
        return path_dep


def main():
    """Main entry point."""
    parser = argparse.ArgumentParser(description="Amalgam Nickel package management tools")
    
    # Find workspace root
    current_dir = Path.cwd()
    workspace_root = current_dir
    while workspace_root.parent != workspace_root:
        if (workspace_root / "Cargo.toml").exists() and "amalgam" in (workspace_root / "Cargo.toml").read_text():
            break
        workspace_root = workspace_root.parent
    
    if not (workspace_root / "Cargo.toml").exists():
        print("Error: Could not find Amalgam workspace root")
        sys.exit(1)
    
    manager = PackageManager(workspace_root)
    
    subparsers = parser.add_subparsers(dest="command", help="Available commands")
    
    # List packages
    subparsers.add_parser("list", help="List all available packages")
    
    # Test package
    test_parser = subparsers.add_parser("test", help="Test a package")
    test_parser.add_argument("package", help="Package name to test")
    test_parser.add_argument(
        "--mode",
        choices=["path", "git", "index"],
        default="path",
        help="Dependency mode to test"
    )
    
    # Validate package
    validate_parser = subparsers.add_parser("validate", help="Validate a package")
    validate_parser.add_argument("package", help="Package name to validate")
    
    # Prepare for publishing
    publish_parser = subparsers.add_parser("prepare", help="Prepare package for publishing")
    publish_parser.add_argument("package", help="Package name to prepare")
    publish_parser.add_argument(
        "--target",
        type=Path,
        default=Path("./publish"),
        help="Target directory for prepared package"
    )
    
    # Test all packages
    subparsers.add_parser("test-all", help="Test all packages")
    
    args = parser.parse_args()
    
    if args.command == "list":
        packages = manager.list_packages()
        if packages:
            print("Available packages:")
            for pkg in packages:
                deps_str = ", ".join(pkg.dependencies.keys()) if pkg.dependencies else "none"
                print(f"  - {pkg.name} (v{pkg.version}) - Dependencies: {deps_str}")
        else:
            print("No packages found")
    
    elif args.command == "test":
        result = manager.test_package(args.package, args.mode)
        if result.success:
            print(f"✅ Package {args.package} test passed ({args.mode} mode)")
            if result.output:
                print(f"Output: {result.output}")
        else:
            print(f"❌ Package {args.package} test failed ({args.mode} mode)")
            if result.error:
                print(f"Error: {result.error}")
            sys.exit(1)
    
    elif args.command == "validate":
        result = manager.validate_package(args.package)
        if result.success:
            print(f"✅ {result.output}")
        else:
            print(f"❌ Package {args.package} validation failed")
            if result.error:
                print(f"Errors:\n{result.error}")
            sys.exit(1)
    
    elif args.command == "prepare":
        if manager.prepare_for_publishing(args.package, args.target):
            print(f"✅ Package prepared successfully")
        else:
            print(f"❌ Failed to prepare package")
            sys.exit(1)
    
    elif args.command == "test-all":
        packages = manager.list_packages()
        failed = []
        
        for pkg in packages:
            print(f"Testing {pkg.name}...")
            for mode in ["path"]:  # Start with path mode only
                result = manager.test_package(pkg.name, mode)
                if result.success:
                    print(f"  ✅ {mode} mode: passed")
                else:
                    print(f"  ❌ {mode} mode: failed")
                    failed.append((pkg.name, mode))
        
        if failed:
            print(f"\n❌ {len(failed)} tests failed:")
            for pkg, mode in failed:
                print(f"  - {pkg} ({mode} mode)")
            sys.exit(1)
        else:
            print(f"\n✅ All tests passed!")
    
    else:
        parser.print_help()


if __name__ == "__main__":
    main()