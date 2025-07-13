#!/usr/bin/env python3
"""
Yeti Set Go - AI Asset Generation Tool
Generates consistent game assets using Black Forest Labs Flux API
"""

import os
import requests
import time
import base64
from typing import Dict, List, Optional, Tuple
from pathlib import Path
from dataclasses import dataclass
from enum import Enum
import argparse
from PIL import Image
import io
import logging
import random


class FluxModel(Enum):
    KONTEXT_PRO = "flux-kontext-pro"
    KONTEXT_MAX = "flux-kontext-max"
    PRO_1_1 = "flux-pro-1.1"
    PRO = "flux-pro"
    DEV = "flux-dev"


class AssetType(Enum):
    YETI_SPRITE = "yeti_sprite"
    ITEM_SPRITE = "item_sprite"
    ENVIRONMENT = "environment"
    STYLE_GUIDE = "style_guide"


@dataclass
class AssetSpec:
    name: str
    type: AssetType
    size: Tuple[int, int]
    description: str
    variations: List[str] = None
    batch_group: str = None
    model: FluxModel = FluxModel.KONTEXT_PRO
    aspect_ratio: str = "1:1"
    reference_image_path: Optional[str] = None


class FluxAssetGenerator:
    def __init__(self, api_key: str, region: str = "global"):
        self.api_key = api_key

        # API endpoints
        self.base_urls = {
            "global": "https://api.bfl.ai",
            "eu": "https://api.eu.bfl.ai",
            "us": "https://api.us.bfl.ai",
        }
        self.base_url = self.base_urls.get(region, self.base_urls["global"])

        self.output_dir = Path("generated_assets")
        self.output_dir.mkdir(exist_ok=True)

        # Rate limiting
        self.max_concurrent_requests = 24
        self.max_concurrent_kontext_max = 6
        self.active_requests = 0

        # Setup logging
        logging.basicConfig(level=logging.INFO)
        self.logger = logging.getLogger(__name__)

        # Fluree brand colors from spec
        self.brand_colors = {
            "ice_blue": "#CEF1FF",
            "vibrant_blue": "#13C6FF",
            "deep": "#091133",
            "fluree_safe_blue": "#00A0D1",
            "peak": "#C6D4FF",
            "violet": "#B775D6",
            "purple": "#4B56A5",
            "plum": "#171F69",
            "grey": "#979797",
            "metal": "#5D6970",
            "teal": "#18CFDB",
            "ember": "#FF4C13",
        }

        # Separate style bases for different asset types
        self.yeti_style = f"""
        clean minimal pixel art style, 8-bit retro gaming aesthetic, 
        for small casual endless runner game window,
        using Fluree brand colors: ice blue {self.brand_colors['ice_blue']}, 
        vibrant blue {self.brand_colors['vibrant_blue']}, 
        deep navy {self.brand_colors['deep']}, metal {self.brand_colors['metal']},
        completely transparent background, PNG with alpha channel, isolated on transparent background, 
        no background elements, cut out style, no ground, no shadows, no scenery, 
        floating character on pure transparency, remove all background completely,
        consistent lighting and perspective, crisp pixel edges, no blur, isolated subject,
        optimized for CI/CD pipeline themed endless runner game
        """

        self.icon_style = "simple flat design, solid colors, transparent background, clean minimal icon, no gradients, no shadows"

        self.environment_style = f"""
        clean pixel art style, using metal {self.brand_colors['metal']} and grey {self.brand_colors['grey']} tones,
        Fluree SafeBlue {self.brand_colors['fluree_safe_blue']} accents, minimal aesthetic for small game window,
        transparent background, PNG with alpha channel
        """

        self.asset_specs = self._load_asset_specs()

    def _load_asset_specs(self) -> List[AssetSpec]:
        """Load asset specifications with improved prompting"""
        specs = []

        # Style guide - simplified and focused
        style_guide = AssetSpec(
            name="style_reference",
            type=AssetType.STYLE_GUIDE,
            size=(512, 512),
            description="Cute yeti character for 2D endless runner game, white and blue fur, friendly cartoon face, simple clean art style, facing right, standing neutral pose",
            model=FluxModel.PRO_1_1,  # Use text-to-image for initial style guide
            aspect_ratio="1:1",
        )
        specs.append(style_guide)

        # SIMPLIFIED YETI ANIMATION - 2 distinctly different frames
        yeti_specs = [
            AssetSpec(
                "yeti_run_frame1_left_foot_forward_no_bg",
                AssetType.YETI_SPRITE,
                (60, 60),
                "A single cute yeti character running, left foot forward, right arm extended back, left arm forward, animated sprite style, transparent colorless png background, matching the exact art style of the reference image, 2D cartoon illustration, simple shading, friendly expression, running pose frame 1",
                batch_group="yeti",
                model=FluxModel.KONTEXT_PRO,
            ),
            AssetSpec(
                "yeti_run_frame3_both_feet_contact_no_bg",
                AssetType.YETI_SPRITE,
                (60, 60),
                "A single cute yeti character running, right foot forward, left arm extended back, right arm forward, animated sprite style, transparent colorless png background, matching the exact art style of the reference image, 2D cartoon illustration, simple shading, friendly expression, running pose frame 2",
                batch_group="yeti",
                model=FluxModel.KONTEXT_PRO,
            ),
            # Other yeti actions - simplified prompts
            AssetSpec(
                "yeti_jump_no_bg",
                AssetType.YETI_SPRITE,
                (60, 60),
                "Same yeti character, jumping pose, both legs tucked up",
                batch_group="yeti",
                model=FluxModel.KONTEXT_PRO,
            ),
            AssetSpec(
                "yeti_cheer_no_bg",
                AssetType.YETI_SPRITE,
                (60, 60),
                "Same yeti character, celebration pose, both arms raised high",
                batch_group="yeti",
                model=FluxModel.KONTEXT_PRO,
            ),
            AssetSpec(
                "yeti_stumble_no_bg",
                AssetType.YETI_SPRITE,
                (60, 60),
                "Same yeti character, single yeti, transform from happy facial expression to sad, a little dazed, wobbly pose, stars circling head",
                batch_group="yeti",
                model=FluxModel.KONTEXT_PRO,
            ),
        ]
        specs.extend(yeti_specs)

        # Item icons are now generated using actual heroicons (see generate_heroicons_items method)
        # No AI generation needed for simple, consistent icons

        # Environment assets with simplified prompts
        env_specs = [
            AssetSpec(
                "pipeline_track",
                AssetType.ENVIRONMENT,
                (128, 32),
                "horizontal CI/CD pipeline conveyor belt track, pixel art style, industrial metal texture with subtle repeating pattern",
                aspect_ratio="4:1",
            ),
            AssetSpec(
                "background",
                AssetType.ENVIRONMENT,
                (480, 320),
                "minimal cloud infrastructure background, soft gradient, simple design",
                aspect_ratio="3:2",
            ),
            AssetSpec(
                "ui_frame",
                AssetType.ENVIRONMENT,
                (160, 32),
                "clean UI frame for progress counter, rounded corners, minimal design",
                aspect_ratio="4:1",
            ),
        ]
        specs.extend(env_specs)

        return specs

    def _encode_image_to_base64(self, image_path: str) -> str:
        """Encode image to base64 for Kontext models"""
        with open(image_path, "rb") as img_file:
            return base64.b64encode(img_file.read()).decode("utf-8")

    def _wait_for_rate_limit(self, model: FluxModel):
        """Wait if we've hit rate limits"""
        max_requests = (
            self.max_concurrent_kontext_max
            if model == FluxModel.KONTEXT_MAX
            else self.max_concurrent_requests
        )
        while self.active_requests >= max_requests:
            time.sleep(1)

    def _submit_generation_request(
        self,
        prompt: str,
        model: FluxModel,
        aspect_ratio: str = "1:1",
        reference_image: Optional[str] = None,
    ) -> Optional[Dict]:
        """Submit generation request and return task info"""
        self._wait_for_rate_limit(model)
        self.active_requests += 1

        try:
            headers = {"Content-Type": "application/json", "x-key": self.api_key}

            # Build request data
            data = {
                "prompt": prompt,
                "aspect_ratio": aspect_ratio,
                "safety_tolerance": 2,
                "output_format": "png",
            }

            # Add reference image for Kontext models
            if reference_image and model in [
                FluxModel.KONTEXT_PRO,
                FluxModel.KONTEXT_MAX,
            ]:
                data["input_image"] = reference_image

            # Use correct endpoint format
            endpoint = f"{self.base_url}/v1/{model.value}"

            self.logger.info(f"Submitting request to {endpoint}")
            response = requests.post(endpoint, headers=headers, json=data)

            if response.status_code == 200:
                result = response.json()
                self.logger.info(f"Task submitted successfully. ID: {result['id']}")
                # Use the polling URL from the response if available, otherwise construct it
                polling_url = result.get("polling_url")
                if not polling_url:
                    polling_url = f"{self.base_url}/v1/get_result?id={result['id']}"
                self.logger.info(f"Polling URL: {polling_url}")
                return {"id": result["id"], "polling_url": polling_url}
            elif response.status_code == 429:
                self.logger.warning("Rate limited, waiting...")
                time.sleep(random.uniform(5, 15))
                return self._submit_generation_request(
                    prompt, model, aspect_ratio, reference_image
                )
            elif response.status_code == 402:
                self.logger.error("Insufficient credits")
                return None
            else:
                self.logger.error(
                    f"API Error: {response.status_code} - {response.text}"
                )
                return None

        except Exception as e:
            self.logger.error(f"Request failed: {e}")
            return None
        finally:
            self.active_requests -= 1

    def _poll_for_result(self, task_info: Dict, timeout: int = 300) -> Optional[bytes]:
        """Poll for generation result with exponential backoff"""
        start_time = time.time()
        wait_time = 2

        while time.time() - start_time < timeout:
            try:
                headers = {"x-key": self.api_key}
                self.logger.debug(f"Polling: {task_info['polling_url']}")
                response = requests.get(task_info["polling_url"], headers=headers)

                if response.status_code == 200:
                    result = response.json()
                    status = result.get("status")

                    if status == "Ready":
                        # Download the image immediately (expires in 10 minutes)
                        image_url = result["result"]["sample"]
                        img_response = requests.get(image_url)
                        if img_response.status_code == 200:
                            return img_response.content
                        else:
                            self.logger.error(
                                f"Failed to download image: {img_response.status_code}"
                            )
                            return None
                    elif status in ["Error", "Content Moderated"]:
                        self.logger.error(f"Generation failed: {status}")
                        if "details" in result:
                            self.logger.error(f"Details: {result['details']}")
                        return None
                    elif status == "Pending":
                        progress = result.get("progress", 0)
                        self.logger.info(
                            f"Generation in progress... {progress}% (waited {int(time.time() - start_time)}s)"
                        )
                        time.sleep(wait_time)
                        wait_time = min(wait_time * 1.2, 15)  # Exponential backoff
                        continue
                else:
                    self.logger.error(
                        f"Polling error: {response.status_code} - {response.text}"
                    )
                    self.logger.error(f"Polling URL: {task_info['polling_url']}")
                    return None

            except Exception as e:
                self.logger.error(f"Polling failed: {e}")
                return None

        self.logger.error(f"Generation timed out after {timeout} seconds")
        return None

    def _build_prompt(self, spec: AssetSpec, variation: str = None) -> str:
        """Build optimized prompts based on BFL best practices"""

        if spec.type == AssetType.ITEM_SPRITE:
            # Ultra-minimal prompts for icons
            prompt = spec.description
            if variation:
                prompt += f", {variation}"
            return prompt

        elif spec.type == AssetType.YETI_SPRITE:
            # Simple prompts that rely on Kontext for consistency
            if spec.reference_image_path and os.path.exists(spec.reference_image_path):
                # Use minimal prompt when we have reference
                prompt = spec.description
            else:
                # Only add style details for first generation
                prompt = f"{spec.description}, white and blue fur, friendly cartoon style, clean pixel art"

            if variation:
                prompt += f", {variation}"
            return prompt

        elif spec.type == AssetType.STYLE_GUIDE:
            # Detailed but focused style guide prompt
            return spec.description

        else:
            # Environment assets
            return f"{spec.description}, pixel art style, clean minimal design"

    def _build_item_prompt(self, spec: AssetSpec, variation: str = None) -> str:
        """Build simple, clean prompts for item sprites"""
        prompt = f"{spec.description}, {self.icon_style}"

        if variation:
            prompt += f", {variation}"

        # Add size optimization
        prompt += f", optimized for {spec.size[0]}x{spec.size[1]} pixel display"

        return prompt

    def _build_yeti_prompt(self, spec: AssetSpec, variation: str = None) -> str:
        """Build complex prompts for yeti sprites with style consistency"""
        has_style_reference = (
            spec.reference_image_path
            and os.path.exists(spec.reference_image_path)
            and spec.type != AssetType.STYLE_GUIDE
        )

        if has_style_reference:
            base_prompt = f"Using the same art style and character design as the reference image, create: {spec.description}"
            base_prompt += (
                f", maintain the exact same visual style, colors, and artistic approach"
            )
            base_prompt += (
                f", but generate a completely new image showing: {spec.description}"
            )
        else:
            base_prompt = f"{spec.description}, {self.yeti_style}"

        if variation:
            base_prompt += f", {variation}"

        # Add size constraints
        base_prompt += f", optimized for {spec.size[0]}x{spec.size[1]} pixel display"

        # Add yeti-specific styling
        if has_style_reference:
            base_prompt += f", same character as reference but in new pose/action, keep identical visual style and character design, maintain right-facing direction"
            base_prompt += f", completely transparent background, PNG with alpha channel, isolated on transparent background, no background elements, cut out style"
        else:
            base_prompt += f", cute friendly yeti character, white and blue fur using ice blue {self.brand_colors['ice_blue']} and vibrant blue {self.brand_colors['vibrant_blue']} as primary colors, deep {self.brand_colors['deep']} for outlines and shadows, consistent character design"

        # Force transparency for ALL yeti sprites
        base_prompt += f", completely transparent background, PNG with alpha channel, isolated on transparent background, no background elements, cut out style, no ground, no shadows, no scenery, floating character on pure transparency, remove all background completely"

        return base_prompt

    def _build_environment_prompt(self, spec: AssetSpec, variation: str = None) -> str:
        """Build prompts for environment sprites"""
        has_style_reference = spec.reference_image_path and os.path.exists(
            spec.reference_image_path
        )

        if has_style_reference:
            base_prompt = f"Using the same art style and color palette as the reference image, create: {spec.description}"
        else:
            base_prompt = f"{spec.description}, {self.environment_style}"

        if variation:
            base_prompt += f", {variation}"

        # Add size constraints
        base_prompt += f", optimized for {spec.size[0]}x{spec.size[1]} pixel display"

        return base_prompt

    def generate_image(
        self,
        prompt: str,
        model: FluxModel,
        aspect_ratio: str = "1:1",
        reference_image: Optional[str] = None,
    ) -> Optional[bytes]:
        """Generate a single image using Flux API"""
        task_info = self._submit_generation_request(
            prompt, model, aspect_ratio, reference_image
        )
        if not task_info:
            return None

        return self._poll_for_result(task_info)

    def post_process_image(
        self, image_data: bytes, target_size: Tuple[int, int]
    ) -> Optional[bytes]:
        """Basic post-processing (resize and optimize only)"""
        try:
            img = Image.open(io.BytesIO(image_data))

            if img.mode != "RGBA":
                img = img.convert("RGBA")

            # High-quality resize
            img = img.resize(target_size, Image.LANCZOS)

            # Optimize for pixel art
            if target_size[0] <= 64 and target_size[1] <= 64:
                # Apply slight sharpening for small pixel art
                from PIL import ImageFilter

                img = img.filter(ImageFilter.SHARPEN)

            output = io.BytesIO()
            img.save(output, format="PNG", optimize=True)
            return output.getvalue()

        except Exception as e:
            self.logger.error(f"Post-processing failed: {e}")
            return None

    def generate_asset(self, spec: AssetSpec, variation: str = None) -> bool:
        """Generate a single asset with enhanced features"""
        prompt = self._build_prompt(spec, variation)

        # Use reference image for consistency if available
        reference_image = None
        if spec.reference_image_path and os.path.exists(spec.reference_image_path):
            reference_image = self._encode_image_to_base64(spec.reference_image_path)

        filename = f"{spec.name}{f'_{variation}' if variation else ''}.png"
        self.logger.info(f"Generating: {filename}")
        self.logger.info(f"Model: {spec.model.value}")
        self.logger.info(f"Prompt: {prompt}")

        # Generate image
        image_data = self.generate_image(
            prompt, spec.model, spec.aspect_ratio, reference_image
        )
        if not image_data:
            self.logger.error("Failed to generate image")
            return False

        # Post-process (basic resize and optimize)
        processed_data = self.post_process_image(image_data, spec.size)
        if not processed_data:
            self.logger.error("Failed to post-process image")
            return False

        # Save to file
        filepath = self.output_dir / filename
        with open(filepath, "wb") as f:
            f.write(processed_data)

        self.logger.info(f"Saved: {filepath}")
        return True

    def generate_style_guide_first(self, reference_image_path: str = None) -> bool:
        """Generate style guide first as reference for other assets"""
        style_specs = [
            spec for spec in self.asset_specs if spec.type == AssetType.STYLE_GUIDE
        ]

        for spec in style_specs:
            # Use reference image if provided
            if reference_image_path and os.path.exists(reference_image_path):
                spec.reference_image_path = reference_image_path
                spec.model = FluxModel.KONTEXT_PRO  # Use Kontext for image-to-image
                self.logger.info(f"Using reference image: {reference_image_path}")

            if self.generate_asset(spec):
                # Set this as reference for ALL other assets to ensure consistent style
                style_path = str(self.output_dir / f"{spec.name}.png")
                self.logger.info(f"Style guide generated successfully: {style_path}")
                self.logger.info(
                    "Setting style guide as reference for all future asset generations..."
                )

                for asset_spec in self.asset_specs:
                    # Only set style guide reference for YETI sprites, not item sprites
                    if asset_spec.type == AssetType.YETI_SPRITE:
                        asset_spec.reference_image_path = style_path
                        asset_spec.model = (
                            FluxModel.KONTEXT_PRO
                        )  # Use Kontext for style transfer
                        self.logger.info(
                            f"Set style reference for {asset_spec.name} -> {style_path}"
                        )
                    elif asset_spec.type == AssetType.ITEM_SPRITE:
                        # Keep item sprites using text-only generation for clean, simple icons
                        asset_spec.reference_image_path = None
                        asset_spec.model = (
                            FluxModel.PRO_1_1
                        )  # Use regular text-to-image
                        self.logger.info(
                            f"Item sprite {asset_spec.name} will use simplified text-only generation"
                        )

                return True
        return False

    def generate_batch(self, batch_group: str) -> bool:
        """Generate a batch of related assets for consistency"""
        batch_specs = [
            spec for spec in self.asset_specs if spec.batch_group == batch_group
        ]

        if not batch_specs:
            self.logger.error(f"No assets found for batch group: {batch_group}")
            return False

        self.logger.info(f"Generating batch: {batch_group}")
        success_count = 0

        # Special handling for animation sequences like yeti_run
        if batch_group == "yeti_run":
            return self._generate_animation_sequence(batch_specs[0])

        # Regular batch processing for other groups
        for spec in batch_specs:
            if spec.variations:
                for variation in spec.variations:
                    if self.generate_asset(spec, variation):
                        success_count += 1
                    time.sleep(3)  # Rate limiting
            else:
                if self.generate_asset(spec):
                    success_count += 1
                time.sleep(3)  # Rate limiting

        self.logger.info(f"Batch complete: {success_count} assets generated")
        return success_count > 0

    def _generate_animation_sequence_simplified(self) -> bool:
        """Generate 2-frame running animation using Kontext chaining"""

        # Step 1: Generate Frame 1 using style guide as reference
        frame1_spec = AssetSpec(
            name="yeti_run_frame1_left_foot_forward_no_bg",
            type=AssetType.YETI_SPRITE,
            size=(60, 60),
            description="A single cute yeti character running, left foot forward, right arm extended back, left arm forward, animated sprite style, transparent colorless png background, matching the exact art style of the reference image, 2D cartoon illustration, simple shading, friendly expression, running pose frame 1",
            model=FluxModel.KONTEXT_PRO,
            reference_image_path=self.output_dir / "style_reference.png",
        )

        self.logger.info("Generating Frame 1 (extended leap)...")
        if not self.generate_asset(frame1_spec):
            self.logger.error("Failed to generate frame 1")
            return False

        time.sleep(3)  # Rate limiting

        # Step 2: Generate Frame 2 using Frame 1 as reference
        frame2_spec = AssetSpec(
            name="yeti_run_frame3_both_feet_contact_no_bg",
            type=AssetType.YETI_SPRITE,
            size=(60, 60),
            description="A single cute yeti character running, right foot forward, left arm extended back, right arm forward, animated sprite style, transparent colorless png background, matching the exact art style of the reference image, 2D cartoon illustration, simple shading, friendly expression, running pose frame 2",
            model=FluxModel.KONTEXT_PRO,
            reference_image_path=self.output_dir / "style_reference.png",
        )

        self.logger.info("Generating Frame 2 (compressed crouch)...")
        if not self.generate_asset(frame2_spec):
            self.logger.error("Failed to generate frame 2")
            return False

        self.logger.info("2-frame running animation complete!")
        return True

    def generate_all_improved(self) -> None:
        """Improved workflow for consistent asset generation"""

        self.logger.info("Starting improved asset generation workflow...")

        # Step 1: Generate style guide (if not already exists)
        style_path = self.output_dir / "style_reference.png"
        if style_path.exists():
            self.logger.info(f"Step 1: Using existing style guide: {style_path}")
        else:
            self.logger.info("Step 1: Generating style guide...")
            style_spec = next(
                spec for spec in self.asset_specs if spec.type == AssetType.STYLE_GUIDE
            )
            if not self.generate_asset(style_spec):
                self.logger.error("Style guide generation failed - aborting")
                return
            self.logger.info(f"Style guide complete: {style_path}")
            time.sleep(5)

        # Step 2: Generate yeti animation frames in sequence
        self.logger.info("Step 2: Generating yeti animation frames...")
        if not self._generate_animation_sequence_simplified():
            self.logger.error("Animation generation failed")
        time.sleep(5)

        # Step 3: Generate other yeti poses using style guide
        self.logger.info("Step 3: Generating other yeti poses...")
        other_yeti_specs = [
            spec
            for spec in self.asset_specs
            if spec.type == AssetType.YETI_SPRITE and "frame" not in spec.name
        ]

        for spec in other_yeti_specs:
            spec.reference_image_path = str(style_path)
            self.generate_asset(spec)
            time.sleep(3)

        # Step 4: Item icons are now handled by download_heroicons.py script
        self.logger.info(
            "Step 4: Skipping item icons (use download_heroicons.py script instead)"
        )

        # Step 5: Environment assets
        self.logger.info("Step 5: Generating environment assets...")
        env_specs = [
            spec for spec in self.asset_specs if spec.type == AssetType.ENVIRONMENT
        ]
        for spec in env_specs:
            self.generate_asset(spec)
            time.sleep(3)

        self.logger.info("Improved asset generation complete!")

    def generate_yeti_animation_only(self) -> bool:
        """Generate just the yeti animation frames with improved prompting"""

        # Ensure we have a style guide
        style_path = self.output_dir / "style_reference.png"
        if not style_path.exists():
            self.logger.error(
                "Style guide not found. Generate it first with --style-guide-first"
            )
            return False

        return self._generate_animation_sequence_simplified()

    def generate_item_icons_only(self) -> bool:
        """Item icons are now handled by download_heroicons.py script"""
        self.logger.info("Item icons are now handled by download_heroicons.py script")
        self.logger.info("Run: python download_heroicons.py")
        return True

    def setup_style_guide_references(self) -> bool:
        """Set up style guide as reference for all assets if it exists"""
        style_guide_path = self.output_dir / "style_reference.png"

        if style_guide_path.exists():
            self.logger.info(f"Found existing style guide: {style_guide_path}")
            self.logger.info("Setting up style guide as reference for all assets...")

            for asset_spec in self.asset_specs:
                # Only set style guide reference for YETI sprites, not item sprites
                if asset_spec.type == AssetType.YETI_SPRITE:
                    asset_spec.reference_image_path = str(style_guide_path)
                    asset_spec.model = FluxModel.KONTEXT_PRO
                    self.logger.info(f"Set style reference for {asset_spec.name}")
                elif asset_spec.type == AssetType.ITEM_SPRITE:
                    # Keep item sprites using text-only generation for clean, simple icons
                    asset_spec.reference_image_path = None
                    asset_spec.model = FluxModel.PRO_1_1
                    self.logger.info(
                        f"Item sprite {asset_spec.name} will use simplified text-only generation"
                    )

            return True
        else:
            self.logger.warning(
                "No style guide found. Generate one first with --style-guide-first"
            )
            return False

    def edit_image(
        self,
        input_image_path: str,
        edit_prompt: str,
        output_name: str = None,
        model: FluxModel = FluxModel.KONTEXT_PRO,
    ) -> bool:
        """Edit an existing image using Flux Kontext image-to-image"""
        if not os.path.exists(input_image_path):
            self.logger.error(f"Input image not found: {input_image_path}")
            return False

        # Encode the input image
        input_image_b64 = self._encode_image_to_base64(input_image_path)

        # Build the edit prompt
        full_prompt = f"{edit_prompt}, {self.style_base}"

        if output_name is None:
            base_name = Path(input_image_path).stem
            output_name = f"{base_name}_edited"

        self.logger.info(f"Editing image: {input_image_path}")
        self.logger.info(f"Edit prompt: {full_prompt}")
        self.logger.info(f"Output name: {output_name}")

        # Generate the edited image
        image_data = self.generate_image(full_prompt, model, "1:1", input_image_b64)
        if not image_data:
            self.logger.error("Failed to edit image")
            return False

        # Save the edited image
        output_path = self.output_dir / f"{output_name}.png"
        with open(output_path, "wb") as f:
            f.write(image_data)

        self.logger.info(f"Edited image saved: {output_path}")
        return True

    def remove_background(self, image_path: str, output_path: str = None) -> bool:
        """Remove background from an existing image using BFL API"""
        if not os.path.exists(image_path):
            self.logger.error(f"Image not found: {image_path}")
            return False

        try:
            # Encode the input image
            input_image_b64 = self._encode_image_to_base64(image_path)

            # Background removal prompt
            bg_removal_prompt = "Remove the background completely, make it transparent, keep only the main subject, PNG format with alpha channel, isolated subject on transparent background"

            # Get original image size for output
            img = Image.open(image_path)
            original_size = img.size

            # Determine output path
            if output_path is None:
                path_obj = Path(image_path)
                output_path = (
                    path_obj.parent / f"{path_obj.stem}_no_bg{path_obj.suffix}"
                )

            self.logger.info(f"Removing background using BFL API: {image_path}")
            self.logger.info(f"Background removal prompt: {bg_removal_prompt}")

            # Use BFL API to remove background
            processed_image_data = self.generate_image(
                bg_removal_prompt, FluxModel.KONTEXT_PRO, "1:1", input_image_b64
            )

            if not processed_image_data:
                self.logger.error("Failed to remove background via BFL API")
                return False

            # Basic post-processing (resize to original size)
            final_data = self.post_process_image(processed_image_data, original_size)
            if not final_data:
                self.logger.error("Failed to post-process background-removed image")
                return False

            # Save processed image
            with open(output_path, "wb") as f:
                f.write(final_data)

            self.logger.info(
                f"Background removed successfully: {image_path} -> {output_path}"
            )
            return True

        except Exception as e:
            self.logger.error(f"Background removal failed: {e}")
            return False

    def batch_remove_backgrounds(self) -> int:
        """Remove backgrounds from all images in generated_assets/ using BFL API"""
        if not self.output_dir.exists():
            self.logger.error(f"Output directory not found: {self.output_dir}")
            return 0

        png_files = list(self.output_dir.glob("*.png"))
        if not png_files:
            self.logger.warning("No PNG files found in output directory")
            return 0

        # Filter out files that already have "_no_bg" suffix
        files_to_process = [f for f in png_files if "_no_bg" not in f.stem]

        if not files_to_process:
            self.logger.warning("No files to process (all already have _no_bg suffix)")
            return 0

        success_count = 0
        self.logger.info(
            f"Processing {len(files_to_process)} images for AI background removal..."
        )

        for png_file in files_to_process:
            output_path = png_file.parent / f"{png_file.stem}_no_bg{png_file.suffix}"
            if self.remove_background(str(png_file), str(output_path)):
                success_count += 1
            # Rate limiting between API calls
            time.sleep(3)

        self.logger.info(
            f"AI background removal complete: {success_count}/{len(files_to_process)} images processed"
        )
        return success_count

    def interactive_edit_session(self, input_image_path: str) -> None:
        """Interactive session for editing an image with feedback loop"""
        if not os.path.exists(input_image_path):
            self.logger.error(f"Input image not found: {input_image_path}")
            return

        base_name = Path(input_image_path).stem
        current_image = input_image_path
        edit_count = 0

        print(f"\n=== Interactive Image Editing Session ===")
        print(f"Starting with: {input_image_path}")
        print(f"Images will be saved to: {self.output_dir}")
        print(
            f"Commands: 'edit', 'style', 'color', 'pose', 'detail', 'undo', 'save', 'quit'"
        )

        while True:
            print(f"\nCurrent image: {current_image}")
            print("What would you like to do?")
            print("1. Edit - Custom edit instruction")
            print("2. Style - Change art style")
            print("3. Color - Adjust colors")
            print("4. Pose - Modify character pose")
            print("5. Detail - Add/remove details")
            print("6. Undo - Go back to previous version")
            print("7. Save - Save current version with custom name")
            print("8. Quit - Exit editing session")

            choice = input("Choose option (1-8): ").strip()

            if choice == "1" or choice.lower() == "edit":
                edit_instruction = input("Describe what you want to change: ")
                if edit_instruction:
                    edit_count += 1
                    output_name = f"{base_name}_edit_{edit_count}"
                    if self.edit_image(current_image, edit_instruction, output_name):
                        current_image = str(self.output_dir / f"{output_name}.png")

            elif choice == "2" or choice.lower() == "style":
                print("Style options:")
                print("- 'more pixel art' / 'less pixel art'")
                print("- 'cartoon style' / 'realistic style'")
                print("- 'clean lines' / 'sketchy style'")
                print("- 'bright colors' / 'muted colors'")
                style_instruction = input("Style change: ")
                if style_instruction:
                    edit_count += 1
                    output_name = f"{base_name}_style_{edit_count}"
                    full_instruction = f"Change the style: {style_instruction}"
                    if self.edit_image(current_image, full_instruction, output_name):
                        current_image = str(self.output_dir / f"{output_name}.png")

            elif choice == "3" or choice.lower() == "color":
                print("Color options:")
                print("- 'make brighter' / 'make darker'")
                print("- 'more blue tones' / 'more warm tones'")
                print("- 'increase contrast' / 'softer colors'")
                print("- 'change [color] to [color]'")
                color_instruction = input("Color change: ")
                if color_instruction:
                    edit_count += 1
                    output_name = f"{base_name}_color_{edit_count}"
                    full_instruction = f"Adjust colors: {color_instruction}"
                    if self.edit_image(current_image, full_instruction, output_name):
                        current_image = str(self.output_dir / f"{output_name}.png")

            elif choice == "4" or choice.lower() == "pose":
                print("Pose options:")
                print("- 'arms raised higher' / 'arms lower'")
                print("- 'more dynamic pose' / 'calmer pose'")
                print("- 'facing left' / 'facing right'")
                print("- 'jumping higher' / 'closer to ground'")
                pose_instruction = input("Pose change: ")
                if pose_instruction:
                    edit_count += 1
                    output_name = f"{base_name}_pose_{edit_count}"
                    full_instruction = f"Change the pose: {pose_instruction}, keep the same character and style"
                    if self.edit_image(current_image, full_instruction, output_name):
                        current_image = str(self.output_dir / f"{output_name}.png")

            elif choice == "5" or choice.lower() == "detail":
                print("Detail options:")
                print("- 'add more details' / 'simplify'")
                print("- 'sharper edges' / 'softer edges'")
                print("- 'add shadows' / 'remove shadows'")
                print("- 'add sparkles' / 'remove effects'")
                detail_instruction = input("Detail change: ")
                if detail_instruction:
                    edit_count += 1
                    output_name = f"{base_name}_detail_{edit_count}"
                    full_instruction = f"Adjust details: {detail_instruction}"
                    if self.edit_image(current_image, full_instruction, output_name):
                        current_image = str(self.output_dir / f"{output_name}.png")

            elif choice == "6" or choice.lower() == "undo":
                if edit_count > 0:
                    # Go back to previous version
                    edit_count -= 1
                    if edit_count == 0:
                        current_image = input_image_path
                    else:
                        # Find the previous edit
                        prev_files = list(
                            self.output_dir.glob(f"{base_name}_*_{edit_count}.png")
                        )
                        if prev_files:
                            current_image = str(prev_files[0])
                        else:
                            current_image = input_image_path
                    print(f"Reverted to: {current_image}")
                else:
                    print("Already at original image")

            elif choice == "7" or choice.lower() == "save":
                save_name = input("Enter name for saved version: ")
                if save_name:
                    save_path = self.output_dir / f"{save_name}.png"
                    import shutil

                    shutil.copy2(current_image, save_path)
                    print(f"Saved as: {save_path}")

            elif choice == "8" or choice.lower() == "quit":
                print("Exiting editing session")
                print(f"Final image: {current_image}")
                break

            else:
                print("Invalid choice")

    def interactive_review(self, asset_name: str) -> None:
        """Enhanced interactive review with more options"""
        spec = next((s for s in self.asset_specs if s.name == asset_name), None)
        if not spec:
            self.logger.error(f"Asset not found: {asset_name}")
            return

        while True:
            print(f"\n=== Reviewing: {asset_name} ===")
            print("1. Generate new version")
            print("2. Adjust prompt")
            print("3. Change model")
            print("4. Set reference image")
            print("5. Edit current version (image-to-image)")
            print("6. Accept current version")
            print("7. Skip this asset")

            choice = input("Choose option: ").strip()

            if choice == "1":
                self.generate_asset(spec)
            elif choice == "2":
                new_desc = input(
                    f"Current description: {spec.description}\nNew description: "
                )
                if new_desc:
                    spec.description = new_desc
                    self.generate_asset(spec)
            elif choice == "3":
                print("Available models:")
                for i, model in enumerate(FluxModel, 1):
                    print(f"{i}. {model.value}")
                try:
                    model_choice = int(input("Choose model: ")) - 1
                    spec.model = list(FluxModel)[model_choice]
                    self.generate_asset(spec)
                except (ValueError, IndexError):
                    print("Invalid choice")
            elif choice == "4":
                ref_path = input("Reference image path: ")
                if os.path.exists(ref_path):
                    spec.reference_image_path = ref_path
                    self.generate_asset(spec)
                else:
                    print("File not found")
            elif choice == "5":
                # Image-to-image editing
                current_asset_path = self.output_dir / f"{asset_name}.png"
                if os.path.exists(current_asset_path):
                    self.interactive_edit_session(str(current_asset_path))
                else:
                    print(f"Asset not found: {current_asset_path}")
                    print("Generate the asset first (option 1)")
            elif choice == "6":
                print("Asset accepted")
                break
            elif choice == "7":
                print("Asset skipped")
                break
            else:
                print("Invalid choice")


def main():
    parser = argparse.ArgumentParser(
        description="Generate game assets using Flux AI (V2)"
    )
    parser.add_argument(
        "--api-key", help="Black Forest Labs API key (or set BFL_API_KEY env var)"
    )
    parser.add_argument(
        "--region", choices=["global", "eu", "us"], default="global", help="API region"
    )
    parser.add_argument("--asset", help="Generate specific asset by name")
    parser.add_argument("--batch", help="Generate batch group")
    parser.add_argument("--all", action="store_true", help="Generate all assets")
    parser.add_argument(
        "--interactive", action="store_true", help="Interactive review mode"
    )
    parser.add_argument(
        "--style-guide-first", action="store_true", help="Generate style guide first"
    )
    parser.add_argument(
        "--reference-image", help="Reference image for style guide generation"
    )
    parser.add_argument("--edit", help="Edit an existing image (provide image path)")
    parser.add_argument("--edit-prompt", help="Edit instruction for --edit command")
    parser.add_argument(
        "--edit-session", help="Start interactive editing session (provide image path)"
    )
    parser.add_argument(
        "--remove-bg", help="Remove background from existing image (provide image path)"
    )
    parser.add_argument(
        "--batch-remove-bg",
        action="store_true",
        help="Remove backgrounds from all images in generated_assets/",
    )
    parser.add_argument(
        "--all-improved",
        action="store_true",
        help="Generate all assets using improved workflow",
    )
    parser.add_argument(
        "--yeti-animation-only",
        action="store_true",
        help="Generate just the yeti animation frames",
    )
    parser.add_argument(
        "--item-icons-only",
        action="store_true",
        help="Generate just the item icons with ultra-simple prompts",
    )

    args = parser.parse_args()

    # Get API key
    api_key = args.api_key or os.environ.get("BFL_API_KEY")
    if not api_key:
        print(
            "Error: API key required. Use --api-key or set BFL_API_KEY environment variable"
        )
        return

    generator = FluxAssetGenerator(api_key, args.region)

    if args.edit_session:
        # Interactive editing session
        generator.interactive_edit_session(args.edit_session)
    elif args.edit:
        # Single edit command
        if not args.edit_prompt:
            print("Error: --edit requires --edit-prompt")
            return
        generator.edit_image(args.edit, args.edit_prompt)
    elif args.remove_bg:
        # Remove background from single image
        generator.remove_background(args.remove_bg)
    elif args.batch_remove_bg:
        # Remove backgrounds from all images
        generator.batch_remove_backgrounds()
    elif args.style_guide_first:
        generator.generate_style_guide_first(args.reference_image)
    elif args.all:
        generator.generate_all_improved()
    elif args.all_improved:
        generator.generate_all_improved()
    elif args.yeti_animation_only:
        generator.generate_yeti_animation_only()
    elif args.item_icons_only:
        generator.generate_item_icons_only()
    elif args.batch:
        # Set up style guide reference if available
        generator.setup_style_guide_references()
        generator.generate_batch(args.batch)
    elif args.asset:
        spec = next((s for s in generator.asset_specs if s.name == args.asset), None)
        if spec:
            # Set up style guide reference if available
            generator.setup_style_guide_references()

            if args.interactive:
                generator.interactive_review(args.asset)
            else:
                generator.generate_asset(spec)
        else:
            print(f"Asset not found: {args.asset}")
    else:
        print("Usage examples:")
        print("  Generate style guide first: --style-guide-first")
        print(
            "  Generate style guide with reference: --style-guide-first --reference-image fluree_logo.png"
        )
        print("  Generate all assets (improved workflow): --all")
        print("  Test yeti animation only: --yeti-animation-only")
        print("  Generate specific asset: --asset yeti_run")
        print("  Generate batch: --batch yeti_run")
        print("  Interactive mode: --asset yeti_run --interactive")
        print(
            "  Edit single image: --edit path/to/image.png --edit-prompt 'make it brighter'"
        )
        print("  Interactive editing: --edit-session path/to/image.png")
        print("  Remove background: --remove-bg path/to/image.png")
        print("  Batch background removal: --batch-remove-bg")
        print("")
        print("For item icons:")
        print("  Run: python download_heroicons.py")

        print("\nAvailable assets:")
        for spec in generator.asset_specs:
            print(f"  {spec.name} ({spec.type.value}, {spec.model.value})")

        print("\nAvailable batch groups:")
        batch_groups = set(
            spec.batch_group for spec in generator.asset_specs if spec.batch_group
        )
        for group in sorted(batch_groups):
            print(f"  {group}")

        print("\nImproved workflow commands:")
        print("  --all                    Generate all assets using improved workflow")
        print("  --yeti-animation-only    Test just the 2-frame yeti animation")
        print("  --item-icons-only        Test just the simplified item icons")

        print("\nImage editing commands:")
        print("  --edit IMAGE_PATH --edit-prompt 'INSTRUCTION'")
        print("  --edit-session IMAGE_PATH  (interactive editing)")
        print("  Interactive review includes editing: --asset NAME --interactive")


if __name__ == "__main__":
    main()
