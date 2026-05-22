## Welcome to img2avif

**img2avif** is a powerful command-line tool that converts your images to the modern AVIF format. Whether you\'re a professional photographer working with RAW files or just someone who wants to save disk space without sacrificing quality, img2avif gives you fine-grained control over the conversion process.

> **What is AVIF?** AVIF (AV1 Image File Format) is a modern image format that typically produces files 50% smaller than JPEG while maintaining better visual quality. It supports high dynamic range, wide color gamuts, and transparency.

## Table of Contents

1.  Quick Start
2.  Basic Photography Concepts
3.  Command Line Interface
4.  Configuration Parameters
5.  RAW Processing
6.  Image Rotation
7.  Batch Processing
8.  Examples

## Quick Start

### Basic Usage

\# Convert a single image

img2avif -i photo.jpg

\# Convert all images in a directory

img2avif -d ./my-photos

\# Convert with custom quality

img2avif -i photo.jpg -q 85

\# Process a directory recursively

img2avif -r ./photos \--delete

### What Happens

When you run img2avif, the tool:

1.  Reads your input image (supports over 30 formats including JPEG, PNG, and RAW)
2.  Applies any requested adjustments (exposure, rotation, etc.)
3.  Encodes the result as an AVIF file
4.  Saves it alongside your original (or replaces it if you choose)

## Basic Photography Concepts

Before diving into parameters, let\'s understand some fundamental concepts that img2avif uses.

### Exposure

**What it is:** Exposure controls how bright or dark your image appears. It\'s like adjusting the amount of light that reaches a camera sensor.

**In img2avif:** For RAW files, you can adjust exposure compensation (0.5× to 5×). The *auto* setting analyzes your image\'s histogram to find optimal brightness.

**Example:** An underexposed (too dark) night photo can be brightened using *\--raw-exposure 1.5*.

### Contrast

**What it is:** Contrast defines the difference between the darkest and brightest parts of your image. High contrast makes images \"pop\" but can lose detail in shadows and highlights. Low contrast looks softer and more muted.

**In img2avif:** The *\--raw-contrast* parameter (0.5 to 2.0) adjusts this. Values below 1.0 reduce contrast, above 1.0 increase it.

### Saturation

**What it is:** Saturation controls the intensity of colors. Desaturated images appear muted or black-and-white. Oversaturated images look vivid but unnatural.

**In img2avif:** The *\--raw-saturation* parameter (0.5 to 2.0) adjusts color intensity. Auto-saturation intelligently compensates when you increase exposure.

### Bit Depth

**What it is:** Bit depth determines how many distinct colors or brightness levels can be represented per channel (Red, Green, Blue).

  -------- ------- -------------------------------
  8-bit    256     Standard displays, web images
  10-bit   1,024   Smooth gradients, HDR
  12-bit   4,096   Professional photography
  -------- ------- -------------------------------

**In img2avif:** Use *\--bit-depth* to choose. Auto mode selects appropriate depth based on your source image.

### Color Space: YUV vs RGB

**RGB:** Stores images using Red, Green, and Blue channels. Excellent quality but larger file sizes.

**YUV:** Separates brightness (Y) from color information (U and V). Allows compressing color data more aggressively since human eyes are less sensitive to color detail.

  ----------- ----------- ---------- ---------------------------------
  RGB         Maximum     Largest    Graphics with text, sharp edges
  YUV 4:4:4   Excellent   Medium     Photos with fine color detail
  YUV 4:2:0   Good        Smallest   Most photos
  ----------- ----------- ---------- ---------------------------------

### Lossless vs Lossy Compression

**Lossy** (default): Removes some visual information to achieve smaller files. Quality settings from 0-100 let you trade off between size and perfection.

**Lossless:** Preserves every single pixel exactly as in the original. Creates larger files but mathematically perfect reproduction.

## Command Line Interface

### Basic Syntax

img2avif \[OPTIONS\] \--input \<FILE\> \| \--directory \<DIR\> \| \--recursive \<DIR\>

### Input Selection (Choose One)

  ---------------------------- --------------------------------------------------------
  *-i, \--input \<FILE\>*      Convert a single file
  *-d, \--directory \<DIR\>*   Convert all images in a directory (non-recursive)
  *-r, \--recursive \<DIR\>*   Convert all images in directory and all subdirectories
  ---------------------------- --------------------------------------------------------

### Output Control

  -------------------------- ------------------------------------------------------------------------------
  *-o, \--output \<PATH\>*   Output file or directory. If converting a directory, this should be a folder
  *\--delete*                Delete original files after successful conversion
  *\--discard-if-larger*     Don\'t save AVIF if it\'s larger than original
  -------------------------- ------------------------------------------------------------------------------

### Quality Settings

  ------------------------------ ------------------------------------------------ -------
  *-q, \--quality \<0-100\>*     Main compression quality                         80
  *\--alpha-quality \<0-100\>*   Quality for transparent areas                    80
  *-s, \--speed \<0-10\>*        Encoding speed (0=slowest/best compression)      4
  *\--lossless*                  Enable lossless mode (ignores quality setting)   false
  ------------------------------ ------------------------------------------------ -------

### Color Configuration

  ------------------------------------------ ------------------------ --------
  *\--color-space \<yuv420\|yuv444\|rgb\>*   Color encoding method    yuv420
  *\--bit-depth \<8\|10\|12\|auto\>*         Bits per color channel   auto
  ------------------------------------------ ------------------------ --------

### RAW Processing

  --------------------------------------- ------------------------------------------------ ------
  *\--raw-exposure \<auto\|0.5-5.0\>*     Exposure compensation                            auto
  *\--raw-percentile \<0.5-1.0\>*         Target brightness percentile for auto-exposure   0.95
  *\--raw-saturation \<auto\|0.5-2.0\>*   Color intensity adjustment                       auto
  *\--raw-contrast \<0.5-2.0\>*           Contrast adjustment                              1.0
  --------------------------------------- ------------------------------------------------ ------

### Rotation

  ------------------------------------------ ---------------- ------
  *\--rotate \<auto\|none\|90\|180\|270\>*   Image rotation   auto
  ------------------------------------------ ---------------- ------

### Metadata and Output

  -------------------- -------------------------------------- -------
  *\--keep-metadata*   Preserve EXIF metadata                 false
  *-q, \--quiet*       Suppress all messages                  false
  *-v, \--verbose*     Show detailed processing information   false
  -------------------- -------------------------------------- -------

> **Note:** Short options *-q* appears twice: for quality and quiet mode. They are distinguished by context - *-q 85* sets quality, *-q* alone (no value) enables quiet mode.

## Configuration Parameters (Detailed)

### Quality (*-q, \--quality*)

**Range:** 0 to 100\
**Default:** 80

Controls the trade-off between file size and visual quality.

  ----- ---------------------------------- -------------------
  100   Maximum quality, almost lossless   \~80% of original
  90    Excellent quality for archiving    \~40% of original
  80    Good quality, great for web        \~25% of original
  70    Acceptable quality, small files    \~15% of original
  50    Low quality, tiny files            \~8% of original
  ----- ---------------------------------- -------------------

**Tip:** Unlike JPEG, AVIF at quality 80 typically looks indistinguishable from the original while being much smaller.

### Speed (*-s, \--speed*)

**Range:** 0 to 10\
**Default:** 4

Controls encoding time vs compression efficiency.

  ------ ------------------------ ----------------
  0-1    Very slow (10× slower)   10-20% smaller
  2-3    Slow                     5-10% smaller
  4-5    Balanced (recommended)   Baseline
  6-8    Fast                     5-15% larger
  9-10   Very fast                20-30% larger
  ------ ------------------------ ----------------

**Recommendation:** Start with 4. Use slower speeds for archiving, faster speeds for previews or when converting many images.

### Color Space (*\--color-space*)

#### Yuv420 (default)

-   **Best for:** Most photographs
-   **Size saving:** Up to 50% vs RGB
-   **Quality impact:** Minimal for natural images

#### Yuv444

-   **Best for:** Images with fine colored text, computer graphics
-   **Size saving:** Moderate (15-25% vs RGB)
-   **Quality impact:** Preserves full color resolution

#### RGB

-   **Best for:** Images requiring perfect color reproduction
-   **Size saving:** Minimal
-   **Quality impact:** None (mathematically lossless within quality constraints)

### Bit Depth (*\--bit-depth*)

**Auto mode** analyzes your source image:

-   8-bit for standard JPEGs and PNGs
-   10+ bit for HDR or high-bit-depth RAW files

**Manual selection:**

-   **8-bit:** Universally compatible, smallest files
-   **10-bit:** Better gradients (reduces banding in skies)
-   **12-bit:** Maximum quality for professional work

### Discard If Larger (*\--discard-if-larger*)

Prevents AVIF files that are larger than the original. Some images (especially already-compressed JPEGs with high quality settings) may not compress further.

Example output when enabled:

Skipping: AVIF larger than original (145.2 KB \> 142.1 KB)

### Delete Original (*\--delete*)

**Warning:** This permanently removes original files after successful conversion. Only use this when you have verified the results.

The tool never deletes files if conversion fails.

## RAW Processing

img2avif supports over 50 RAW formats including:

-   **Nikon:** NEF, NRW
-   **Canon:** CR2, CR3, CRW
-   **Sony:** ARW, SRF, SR2
-   **Fujifilm:** RAF
-   **Olympus:** ORF
-   **Panasonic:** RW2
-   **Pentax:** PEF
-   **Sigma:** X3F
-   **Adobe:** DNG
-   And many more\...

### How RAW Processing Works

RAW files contain unprocessed sensor data. img2avif performs several steps:

1.  **Demosaicing:** Converts the Bayer pattern (colored pixels) into full RGB
2.  **White balance:** Applies camera\'s recorded color balance
3.  **Exposure adjustment:** Brightens or darkens the image
4.  **Saturation & contrast:** Applies your adjustments
5.  **Gamma correction:** Converts to viewable colors
6.  **AVIF encoding:** Compresses the final result

### Auto-Exposure (*\--raw-exposure auto*)

The auto-exposure algorithm:

1.  Samples pixels throughout the image
2.  Builds a brightness histogram
3.  Finds the *\--raw-percentile* brightness level (default 95th percentile)
4.  Adjusts exposure so this level becomes 95% of maximum brightness

**Example:** At 95th percentile, the brightest 5% of pixels (specular highlights, light sources) are allowed to clip, while preserving detail in the rest of the image.

### Auto-Saturation

When you increase exposure, colors can appear washed out. Auto-saturation compensates by boosting saturation by 20% of the exposure increase:

auto_saturation = 1.0 + (exposure - 1.0) × 0.2

This maintains vibrant colors when brightening dark images.

### Percentile Targeting (*\--raw-percentile*)

  ------------ ---------------------------------------------------
  0.95 (95%)   Standard, preserves most highlights
  0.99 (99%)   Very conservative, protects almost all highlights
  0.85 (85%)   Brighter image, may clip more highlights
  ------------ ---------------------------------------------------

**Use case:** Night photos with bright lights benefit from lower percentiles (0.90-0.95). Scanned slides might need higher percentiles (0.98-0.99).

### Manual Exposure Example

\# Brighten an underexposed image by 1.5 stops

img2avif -i dark_raw.nef \--raw-exposure 1.5

\# Darken an overexposed image

img2avif -i bright_raw.cr2 \--raw-exposure 0.7

## Image Rotation

img2avif handles rotation in three ways.

### Auto Rotation (*\--rotate auto*) - Default

Reads orientation information from EXIF metadata (the \"Orientation\" tag that cameras write when you take a portrait photo). The tool automatically rotates the image to view correctly.

**Supported orientations:** Normal, 90°, 180°, 270°, mirrored variations.

### Manual Rotation (*\--rotate 90*, *180*, *270*)

Forces rotation regardless of EXIF. Useful when:

-   EXIF data is missing or incorrect
-   You want to rotate images that don\'t have orientation tags
-   Batch processing images that all need the same rotation

### No Rotation (*\--rotate none*)

Disables all rotation. The image stays exactly as decoded.

### Rotation and RAW Files

RAW files often store orientation information too. img2avif reads this via rawloader and applies it when *\--rotate auto* is specified.

## Batch Processing

### Directory Mode (*-d*)

Processes all images in a single directory (non-recursive):

img2avif -d ./vacation-photos -o ./converted

Output structure:

vacation-photos/ converted/

├── beach.jpg → ├── beach.avif

├── sunset.png → ├── sunset.avif

└── family.tiff → └── family.avif

### Recursive Mode (*-r*)

Processes all images in a directory and every subdirectory:

img2avif -r ./photo-archive

Output structure preserves hierarchy:

photo-archive/ photo-archive/

├── 2023/ ├── 2023/

│ ├── spring/ │ ├── spring/

│ │ └── flowers.jpg → │ │ └── flowers.avif

│ └── summer/ │ └── summer/

│ └── beach.jpg → │ └── beach.avif

└── old/ └── old/

└── scanned.png → └── scanned.avif

### Progress Display

When processing multiple files, img2avif shows:

-   A progress bar with current file / total count
-   Estimated time remaining
-   Summary at the end (successful, failed, total size)

### Verbose Mode for Debugging

img2avif -d ./photos -v

Shows:

-   Each file being loaded
-   RAW metadata (camera model, white balance, black levels)
-   Auto-exposure calculation
-   Color mode detection
-   Rotation applied
-   Final compression details

## Examples

### Example 1: Simple Conversion

img2avif -i my-photo.jpg

Creates *my-photo.avif* in the same folder with default settings (quality 80, YUV420, speed 4).

### Example 2: High-Quality Archive

img2avif -i wedding.nef -q 95 \--color-space rgb \--bit-depth 10 -s 2

-   RAW file converted with very high quality
-   RGB color space for maximum fidelity
-   10-bit depth for smooth gradients
-   Slower encoding for better compression

### Example 3: Web-Ready Images

img2avif -d ./originals -o ./web \--quality 75 \--discard-if-larger

-   Processes all images in *originals*
-   Saves to *web* folder
-   Moderate quality (good for web)
-   Skips any conversion that would increase file size

### Example 4: Night Photography

img2avif -i night_sky.nef \--raw-exposure auto \--raw-percentile 0.92 \--raw-saturation auto

-   Auto-exposure targeting 92nd percentile (preserves star highlights)
-   Auto-saturation compensates for brightening
-   Result: Stars remain visible, sky brightens naturally

### Example 5: Batch with Deletion

img2avif -r ./raw-imports \--delete \--quality 85 \--keep-metadata

-   Recursively processes all images in *raw-imports*
-   Deletes originals after successful conversion
-   High quality for archival
-   Preserves EXIF metadata

### Example 6: Correcting Wrong Orientation

img2avif -i sideways.jpg \--rotate 90

Forces 90° clockwise rotation regardless of EXIF.

### Example 7: Lossless PNG to AVIF

img2avif -i diagram.png \--lossless \--color-space rgb

Mathematically lossless conversion preserving all details - ideal for diagrams, screenshots, and graphics with text.

### Example 8: Smallest Possible Files

img2avif -d ./for-email -q 60 \--color-space yuv420 -s 8

Smallest files possible, suitable for email attachments (though quality will be noticeably reduced).

## Understanding Output Messages

### Normal Operation

🖼️ img2avif v1.3.0

© 2026 Philippe TEMESI - https://www.tems.be

Supported formats: JPEG, PNG, BMP, GIF, TIFF, WebP, ICO, RAW\...

📁 Found 24 image(s) in ./photos

Recursive mode: enabled

✓ Converted: DSC_0001.nef -\> DSC_0001.avif (850.2 KB, 18.3% of original)

### Verbose Mode Additional Output

📷 RAW Metadata:

Camera: NIKON CORPORATION NIKON D850

Resolution: 8256 x 5504 pixels

CFA Pattern: RGGB

White Balance: R=2.145, G=1.000, B=1.432

Auto-exposure: 1.24x (target percentile: 95%)

Auto-saturation: 1.05x (based on exposure)

Processing 8256x5504 image\...

### Error Messages

  ----------------------------- ----------------------------------- -------------------------------------
  *Input path does not exist*   File or directory not found         Check the path spelling
  *Unsupported format*          File type not recognized            Check file extension
  *Failed to decode RAW*        RAW file corrupted or unsupported   Try different software, verify file
  *No valid images found*       Directory has no image files        Check file types in directory
  ----------------------------- ----------------------------------- -------------------------------------

## Tips and Best Practices

### For Photographers

1.  **Keep originals** until you verify AVIF quality. Use *\--delete* only after inspection.
2.  **Use auto-exposure** for batches of similar shots under consistent lighting.
3.  **Set ***\--raw-percentile 0.92***** for night or high-contrast scenes.
4.  **10-bit depth** reduces banding in skies and gradients significantly.

### For Web Developers

1.  **Quality 75-80** is usually indistinguishable from original.
2.  **YUV420** offers the best size/quality tradeoff.
3.  **Speed 6-8** is fine for production since you encode once.
4.  **AVIF support** is now in all modern browsers.

### For Archivists

1.  **Use ***\--lossless***** or **quality 100** for master copies.
2.  **Consider ***\--color-space rgb***** for maximum fidelity.
3.  **Keep metadata** with *\--keep-metadata* for copyright and camera info.
4.  **Test a few files** before batch processing entire archives.

### General

-   **Higher quality doesn\'t always mean visibly better** - test at 80, 85, 90 to find your sweet spot.
-   **RAW files take longer** due to demosaicing (but results are worth it).
-   **Use verbose mode ***-v***** the first time you process a new camera\'s RAW files to verify settings.

## Supported Input Formats

### Standard Formats

JPEG, PNG, BMP, GIF, TIFF, WebP, ICO

### RAW Formats (over 50 types)

Nikon (.nef, .nrw), Canon (.cr2, .cr3, .crw), Sony (.arw, .srf, .sr2), Fujifilm (.raf), Olympus (.orf), Panasonic (.rw2), Pentax (.pef), Sigma (.x3f), Adobe (.dng), and many more including .raw, .rwl, .mrw, .dcr, .kdc, .3fr, .ari, .bay, .cap, .data, .dcs, .drf, .erf, .fff, .iiq, .mef, .mos, .mdc, .obm, .ptx, .pxn, .qtk, .rdc, .srw, .sti, .rwz

## Version Information

This manual covers **img2avif version 1.**2**.0**

**Author:** Philippe TEMESI philippe@tems.be\
**Website:** [https://www.tems.be](https://www.tems.be/)\
