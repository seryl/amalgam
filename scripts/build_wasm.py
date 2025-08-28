#!/usr/bin/env python3
"""
Build script for amalgam-wasm with multiple targets and optimization levels.
"""

import os
import sys
import subprocess
import shutil
import json
import toml
from pathlib import Path
from typing import Optional, List, Dict, Any
from dataclasses import dataclass
from enum import Enum

try:
    from rich.console import Console
    from rich.progress import Progress, SpinnerColumn, TextColumn
    from rich.table import Table
    import click
except ImportError:
    print("Error: Required packages not found. Please run in nix develop shell.")
    sys.exit(1)

console = Console()

class Target(Enum):
    """WASM build targets."""
    WEB = "web"
    NODEJS = "nodejs"
    BUNDLER = "bundler"
    NO_MODULES = "no-modules"
    DENO = "deno"

@dataclass
class BuildConfig:
    """Build configuration."""
    target: Target
    optimize: bool
    debug: bool
    profile: str
    features: List[str]
    output_dir: Path

class WasmBuilder:
    """WASM build orchestrator."""
    
    def __init__(self, project_root: Path):
        self.project_root = project_root
        self.wasm_crate = project_root / "crates" / "amalgam-wasm"
        self.cargo_toml = self.wasm_crate / "Cargo.toml"
        
        if not self.cargo_toml.exists():
            console.print(f"[red]Error: amalgam-wasm crate not found at {self.wasm_crate}[/red]")
            sys.exit(1)
    
    def check_dependencies(self) -> bool:
        """Check if required tools are installed."""
        tools = {
            "wasm-pack": "wasm-pack --version",
            "wasm-opt": "wasm-opt --version",
            "cargo": "cargo --version",
        }
        
        missing = []
        for tool, cmd in tools.items():
            try:
                subprocess.run(cmd.split(), capture_output=True, check=True)
            except (subprocess.CalledProcessError, FileNotFoundError):
                missing.append(tool)
        
        if missing:
            console.print(f"[red]Missing tools: {', '.join(missing)}[/red]")
            console.print("[yellow]Please run this script in the nix develop shell[/yellow]")
            return False
        return True
    
    def get_package_info(self) -> Dict[str, Any]:
        """Get package information from Cargo.toml."""
        with open(self.cargo_toml, 'r') as f:
            data = toml.load(f)
        return data.get('package', {})
    
    def build(self, config: BuildConfig) -> bool:
        """Build WASM module with given configuration."""
        pkg_info = self.get_package_info()
        version = pkg_info.get('version', 'unknown')
        
        console.print(f"\n[bold blue]Building amalgam-wasm v{version}[/bold blue]")
        console.print(f"Target: {config.target.value}")
        console.print(f"Profile: {config.profile}")
        console.print(f"Optimize: {config.optimize}")
        
        # Prepare wasm-pack command
        cmd = [
            "wasm-pack", "build",
            "--target", config.target.value,
            "--out-dir", str(config.output_dir),
        ]
        
        if config.profile == "release":
            cmd.append("--release")
        elif config.profile == "dev":
            cmd.append("--dev")
        
        if config.features:
            cmd.extend(["--features", ",".join(config.features)])
        
        # Build with progress indicator
        with Progress(
            SpinnerColumn(),
            TextColumn("[progress.description]{task.description}"),
            transient=True,
        ) as progress:
            task = progress.add_task("Building WASM module...", total=None)
            
            try:
                result = subprocess.run(
                    cmd,
                    cwd=self.wasm_crate,
                    capture_output=True,
                    text=True,
                )
                
                if result.returncode != 0:
                    console.print(f"[red]Build failed:[/red]\n{result.stderr}")
                    return False
                
                progress.update(task, completed=True)
                
            except Exception as e:
                console.print(f"[red]Build error: {e}[/red]")
                return False
        
        # Optimize if requested
        if config.optimize:
            self.optimize_wasm(config.output_dir)
        
        # Generate additional files based on target
        self.generate_target_files(config)
        
        console.print(f"[green]✓ Build complete![/green]")
        self.print_output_info(config)
        
        return True
    
    def optimize_wasm(self, output_dir: Path) -> None:
        """Optimize WASM binary with wasm-opt."""
        wasm_file = output_dir / "amalgam_wasm_bg.wasm"
        if not wasm_file.exists():
            return
        
        console.print("[yellow]Optimizing WASM binary...[/yellow]")
        
        # Create backup
        backup = wasm_file.with_suffix('.wasm.bak')
        shutil.copy2(wasm_file, backup)
        
        cmd = [
            "wasm-opt",
            "-O3",  # Aggressive optimization
            "--enable-simd",  # Enable SIMD if available
            str(wasm_file),
            "-o", str(wasm_file),
        ]
        
        try:
            result = subprocess.run(cmd, capture_output=True, text=True)
            if result.returncode == 0:
                original_size = backup.stat().st_size
                optimized_size = wasm_file.stat().st_size
                reduction = (1 - optimized_size / original_size) * 100
                console.print(f"[green]✓ Optimized: {reduction:.1f}% size reduction[/green]")
                backup.unlink()  # Remove backup
            else:
                # Restore backup on failure
                shutil.move(backup, wasm_file)
                console.print(f"[red]Optimization failed: {result.stderr}[/red]")
        except Exception as e:
            console.print(f"[red]Optimization error: {e}[/red]")
    
    def generate_target_files(self, config: BuildConfig) -> None:
        """Generate additional files based on target."""
        if config.target == Target.WEB:
            self.generate_web_example(config.output_dir)
        elif config.target == Target.NODEJS:
            self.generate_node_example(config.output_dir)
    
    def generate_web_example(self, output_dir: Path) -> None:
        """Generate example HTML file for web target."""
        html_content = """<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Amalgam WASM Demo</title>
    <style>
        body { font-family: system-ui; max-width: 800px; margin: 40px auto; padding: 20px; }
        textarea { width: 100%; height: 200px; font-family: monospace; }
        button { padding: 10px 20px; margin: 10px 0; }
        pre { background: #f4f4f4; padding: 15px; overflow-x: auto; }
        .error { color: red; }
        .success { color: green; }
    </style>
</head>
<body>
    <h1>Amalgam WASM Demo</h1>
    
    <h2>Parse CRD</h2>
    <textarea id="crd-input" placeholder="Paste your CRD YAML here..."></textarea>
    <button onclick="parseCRD()">Parse CRD</button>
    
    <h2>Generate Nickel</h2>
    <button onclick="generateNickel()">Generate Nickel Code</button>
    
    <h2>Output</h2>
    <pre id="output"></pre>
    
    <script type="module">
        import init, { AmalgamWasm } from './amalgam_wasm.js';
        
        let wasm;
        
        async function initialize() {
            await init();
            window.parseCRD = async () => {
                const input = document.getElementById('crd-input').value;
                try {
                    wasm = await AmalgamWasm.parse_crd(input);
                    document.getElementById('output').innerHTML = 
                        '<span class="success">✓ CRD parsed successfully</span>';
                } catch (e) {
                    document.getElementById('output').innerHTML = 
                        '<span class="error">Error: ' + e + '</span>';
                }
            };
            
            window.generateNickel = () => {
                if (!wasm) {
                    document.getElementById('output').innerHTML = 
                        '<span class="error">Please parse a CRD first</span>';
                    return;
                }
                try {
                    const nickel = wasm.generate_nickel();
                    document.getElementById('output').textContent = nickel;
                } catch (e) {
                    document.getElementById('output').innerHTML = 
                        '<span class="error">Error: ' + e + '</span>';
                }
            };
        }
        
        initialize();
    </script>
</body>
</html>"""
        
        html_file = output_dir / "demo.html"
        html_file.write_text(html_content)
        console.print(f"[green]Generated demo at: {html_file}[/green]")
    
    def generate_node_example(self, output_dir: Path) -> None:
        """Generate example Node.js file."""
        js_content = """#!/usr/bin/env node

const { AmalgamWasm } = require('./amalgam_wasm.js');

async function main() {
    // Example CRD
    const crd = `
apiVersion: apiextensions.k8s.io/v1
kind: CustomResourceDefinition
metadata:
  name: examples.test.io
spec:
  group: test.io
  versions:
  - name: v1
    served: true
    storage: true
    schema:
      openAPIV3Schema:
        type: object
        properties:
          spec:
            type: object
            properties:
              replicas:
                type: integer
              message:
                type: string
`;
    
    try {
        // Parse CRD
        const amalgam = await AmalgamWasm.parse_crd(crd);
        console.log('✓ CRD parsed successfully');
        
        // Generate Nickel code
        const nickel = amalgam.generate_nickel();
        console.log('\\nGenerated Nickel code:');
        console.log(nickel);
        
    } catch (error) {
        console.error('Error:', error);
    }
}

main().catch(console.error);
"""
        
        js_file = output_dir / "example.js"
        js_file.write_text(js_content)
        js_file.chmod(0o755)
        console.print(f"[green]Generated example at: {js_file}[/green]")
    
    def print_output_info(self, config: BuildConfig) -> None:
        """Print information about generated files."""
        table = Table(title="Generated Files")
        table.add_column("File", style="cyan")
        table.add_column("Description", style="white")
        
        files = {
            "amalgam_wasm.js": "JavaScript bindings",
            "amalgam_wasm_bg.wasm": "WASM binary",
            "amalgam_wasm.d.ts": "TypeScript definitions",
            "package.json": "NPM package manifest",
        }
        
        for file, desc in files.items():
            path = config.output_dir / file
            if path.exists():
                size = path.stat().st_size
                size_str = f"{size / 1024:.1f} KB" if size > 1024 else f"{size} B"
                table.add_row(file, f"{desc} ({size_str})")
        
        console.print(table)
    
    def build_all_targets(self, optimize: bool = False) -> None:
        """Build for all targets."""
        for target in Target:
            output_dir = self.project_root / "wasm-dist" / target.value
            output_dir.mkdir(parents=True, exist_ok=True)
            
            config = BuildConfig(
                target=target,
                optimize=optimize,
                debug=False,
                profile="release",
                features=[],
                output_dir=output_dir,
            )
            
            self.build(config)

@click.command()
@click.option('--target', '-t', 
              type=click.Choice([t.value for t in Target]),
              default=Target.WEB.value,
              help='Build target')
@click.option('--optimize', '-O', is_flag=True, help='Optimize WASM with wasm-opt')
@click.option('--debug', '-d', is_flag=True, help='Build in debug mode')
@click.option('--all-targets', is_flag=True, help='Build for all targets')
@click.option('--output', '-o', type=click.Path(path_type=Path), 
              help='Output directory')
@click.option('--features', '-f', multiple=True, help='Enable features')
def main(target: str, optimize: bool, debug: bool, all_targets: bool, 
         output: Optional[Path], features: tuple):
    """Build amalgam-wasm with various configurations."""
    
    project_root = Path(__file__).parent.parent
    builder = WasmBuilder(project_root)
    
    if not builder.check_dependencies():
        sys.exit(1)
    
    if all_targets:
        builder.build_all_targets(optimize=optimize)
    else:
        if output is None:
            output = project_root / "wasm-dist" / target
        
        output.mkdir(parents=True, exist_ok=True)
        
        config = BuildConfig(
            target=Target(target),
            optimize=optimize,
            debug=debug,
            profile="dev" if debug else "release",
            features=list(features),
            output_dir=output,
        )
        
        if not builder.build(config):
            sys.exit(1)

if __name__ == '__main__':
    main()