#!/usr/bin/env python3
"""
Quick script to convert white backgrounds to transparent in PNG files
"""

from PIL import Image
import os
import glob

def make_transparent(input_path, output_path=None):
    """Convert white background to transparent"""
    if output_path is None:
        name, ext = os.path.splitext(input_path)
        output_path = f"{name}_transparent{ext}"
    
    # Open image and convert to RGBA
    img = Image.open(input_path).convert("RGBA")
    
    # Get image data
    data = img.getdata()
    
    # Create new data list with transparency
    new_data = []
    for item in data:
        # If pixel is white or very close to white, make it transparent
        if item[0] > 240 and item[1] > 240 and item[2] > 240:
            new_data.append((255, 255, 255, 0))  # Transparent
        else:
            new_data.append(item)  # Keep original
    
    # Update image data
    img.putdata(new_data)
    
    # Save with transparency
    img.save(output_path, "PNG")
    print(f"Converted: {input_path} -> {output_path}")

def main():
    # Process all yeti PNG files
    yeti_files = glob.glob("generated_assets/yeti_*.png")
    
    if not yeti_files:
        print("No yeti PNG files found in generated_assets/")
        return
    
    print(f"Found {len(yeti_files)} yeti files to process...")
    
    for file_path in yeti_files:
        if "_transparent" not in file_path:  # Skip already processed files
            make_transparent(file_path)
    
    print("Done! Transparent versions created with '_transparent' suffix.")
    print("You can now update the Rust code to use these transparent versions.")

if __name__ == "__main__":
    main()