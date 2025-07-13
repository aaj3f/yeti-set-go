#!/usr/bin/env python3
"""
Download heroicons SVGs for Yeti Set Go game items
"""

import requests
from pathlib import Path
try:
    from PIL import Image
    import io
    import subprocess
    SVG_CONVERSION_AVAILABLE = True
except ImportError:
    SVG_CONVERSION_AVAILABLE = False
    print("Warning: PIL not installed. SVG to PNG conversion disabled.")
    print("Install with: pip install Pillow")

def convert_svgs_to_png(directory, size=32):
    """Convert SVG files to PNG format for macroquad compatibility"""
    svg_files = list(directory.glob("*.svg"))
    
    if not svg_files:
        print("No SVG files found to convert")
        return
    
    for svg_file in svg_files:
        try:
            # Skip if PNG already exists
            png_file = svg_file.with_suffix('.png')
            if png_file.exists():
                print(f"Skipping {svg_file.name} - PNG already exists")
                continue
            
            # Try rsvg-convert first (better quality)
            try:
                result = subprocess.run([
                    'rsvg-convert', 
                    '-w', str(size), 
                    '-h', str(size), 
                    str(svg_file),
                    '-o', str(png_file)
                ], capture_output=True, text=True)
                
                if result.returncode == 0:
                    print(f"Converted: {svg_file.name} -> {png_file.name}")
                    continue
                else:
                    raise Exception(f"rsvg-convert failed: {result.stderr}")
                    
            except FileNotFoundError:
                # rsvg-convert not available, try inkscape
                try:
                    result = subprocess.run([
                        'inkscape',
                        '--export-type=png',
                        f'--export-width={size}',
                        f'--export-height={size}',
                        f'--export-filename={png_file}',
                        str(svg_file)
                    ], capture_output=True, text=True)
                    
                    if result.returncode == 0:
                        print(f"Converted: {svg_file.name} -> {png_file.name}")
                        continue
                    else:
                        raise Exception(f"inkscape failed: {result.stderr}")
                        
                except FileNotFoundError:
                    # Neither available, create a simple colored square as fallback
                    print(f"No SVG converter available, creating colored square for {svg_file.name}")
                    
                    # Determine color based on filename
                    if any(word in svg_file.name for word in ['ci_pass', 'pr_merged', 'deploy_success', 'code_review', 'tests_pass']):
                        color = (34, 197, 94)  # Green
                    else:
                        color = (239, 68, 68)  # Red
                    
                    # Create a simple colored square
                    img = Image.new('RGBA', (size, size), color + (255,))
                    img.save(png_file)
                    print(f"Created colored square: {svg_file.name} -> {png_file.name}")
            
        except Exception as e:
            print(f"Error converting {svg_file.name}: {e}")
    
    print(f"PNG conversion complete! Files saved in {directory}")


def download_heroicons():
    """Download specific heroicons we need for the game"""
    
    # Create assets directory
    assets_dir = Path("assets/heroicons")
    assets_dir.mkdir(parents=True, exist_ok=True)
    
    # Heroicons we need (from GitHub raw URLs)
    # Using the solid versions for consistency
    base_url = "https://raw.githubusercontent.com/tailwindlabs/heroicons/master/src/24/solid"
    icons = {
        # Good items (will be colored green) - CI/CD success scenarios
        "check-circle": f"{base_url}/check-circle.svg",           # Passing CI/tests
        "shield-check": f"{base_url}/shield-check.svg",           # Security/validation passed
        "rocket-launch": f"{base_url}/rocket-launch.svg",         # Successful deployment
        "hand-thumb-up": f"{base_url}/hand-thumb-up.svg",         # Code review approval
        "bolt": f"{base_url}/bolt.svg",                           # Fast/optimized performance
        
        # Bad items (will be colored red) - CI/CD failure scenarios
        "x-circle": f"{base_url}/x-circle.svg",                   # Failed tests/CI
        "shield-exclamation": f"{base_url}/shield-exclamation.svg", # Security vulnerability
        "bug-ant": f"{base_url}/bug-ant.svg",                     # Bug detected
        "exclamation-triangle": f"{base_url}/exclamation-triangle.svg", # General warning/error
    }
    
    # Map to our game items - CI/CD themed
    game_items = {
        # Good items (green) - CI/CD success scenarios
        "item_ci_pass": "check-circle",        # Passing CI/tests
        "item_pr_merged": "hand-thumb-up",     # Code review approval  
        "item_deploy_success": "rocket-launch", # Successful deployment
        "item_code_review": "shield-check",    # Security/validation passed
        "item_tests_pass": "bolt",             # Fast/optimized performance
        
        # Bad items (red) - CI/CD failure scenarios
        "item_ci_fail": "x-circle",            # Failed tests/CI
        "item_test_fail": "bug-ant",           # Bug detected
        "item_merge_conflict": "exclamation-triangle", # General warning/error
        "item_security_vuln": "shield-exclamation",    # Security vulnerability
    }
    
    print("Downloading heroicons SVGs...")
    
    # Download base icons
    for icon_name, url in icons.items():
        try:
            response = requests.get(url)
            if response.status_code == 200:
                filepath = assets_dir / f"{icon_name}.svg"
                with open(filepath, 'w') as f:
                    f.write(response.text)
                print(f"Downloaded: {filepath}")
            else:
                print(f"Failed to download {icon_name}: {response.status_code}")
        except Exception as e:
            print(f"Error downloading {icon_name}: {e}")
    
    # Create game-specific copies
    generated_dir = Path("generated_assets")
    generated_dir.mkdir(exist_ok=True)
    
    print("\nCreating game-specific SVG files...")
    
    for game_item, icon_name in game_items.items():
        try:
            source = assets_dir / f"{icon_name}.svg"
            if source.exists():
                # Read the SVG
                with open(source, 'r') as f:
                    svg_content = f.read()
                
                # Determine color based on item type - simple green/red
                if any(word in game_item for word in ['ci_pass', 'pr_merged', 'deploy_success', 'code_review', 'tests_pass']):
                    color = '#22C55E'  # Green for success
                else:
                    color = '#EF4444'  # Red for failure
                
                # Replace fill color (solid heroicons use fill, not stroke) ... should work REGARDLESS of original fill color. Make sure we match on fill="<ANY>"
                svg_content = svg_content.replace('fill="currentColor"', f'fill="{color}"')
                svg_content = svg_content.replace('fill="black"', f'fill="{color}"')
                svg_content = svg_content.replace('fill="none"', f'fill="{color}"')
                svg_content = svg_content.replace('fill="#0F172A"', f'fill="{color}"')
                # Also handle any stroke colors that might be present
                svg_content = svg_content.replace('stroke="currentColor"', f'stroke="{color}"')
                
                # Save colored version
                output_path = generated_dir / f"{game_item}.svg"
                with open(output_path, 'w') as f:
                    f.write(svg_content)
                
                print(f"Created: {output_path}")
            else:
                print(f"Source not found: {source}")
                
        except Exception as e:
            print(f"Error processing {game_item}: {e}")
    
    print(f"\nDone! SVG files created in {generated_dir}")
    
    # Convert SVG files to PNG for macroquad compatibility
    if SVG_CONVERSION_AVAILABLE:
        print("\nConverting SVG files to PNG...")
        convert_svgs_to_png(generated_dir)
    else:
        print("\nTo use in your Rust game:")
        print("1. Add 'quad-svg' to your Cargo.toml dependencies")
        print("2. Load SVGs as textures using quad_svg::svg_to_texture()")
        print("3. Or convert to PNG if you prefer raster assets")

if __name__ == "__main__":
    download_heroicons()