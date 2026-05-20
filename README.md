## **img2avif v1.1.1**

## 🖼️ Convert your photos to AVIF format with ease

Welcome to img2avif, your companion for transforming your images into AVIF, the new format that combines lightweight efficiency with exceptional quality. This manual will guide you through all the app\'s features, from basic settings to advanced parameters for demanding photographers.

## 📖 Understanding image formats

### The AVIF Format (AV1 Image File Format)

Born in 2019, AVIF is the modern successor to JPEG. Based on the AV1 video codec (developed by the Alliance for Open Media, which includes Google, Amazon, Netflix, Cisco, and Mozilla), this format offers:

-   Superior compression: up to 50% smaller file size compared to JPEG at equal quality
-   Exceptional image quality: supports 10 and 12-bit color, HDR color space
-   Transparency: unlike JPEG, AVIF handles alpha channels
-   An open standard: royalty-free, destined to become the universal web format

### Supported Input Formats

The app reads a wide range of formats, from classics to camera raw files:

JPEG (.jpg, .jpeg) - Standard - Created in 1992, the king of the web for 30 years. Lossy compression.\
PNG (.png) - Standard - 1996, lossless format, supports transparency.\
GIF (.gif) - Standard - 1987, animation and transparency, limited to 256 colors.\
BMP (.bmp) - Standard - Historic Windows format, uncompressed.\
TIFF (.tiff, .tif) - Professional - Preservation format for printing.\
WebP (.webp) - Modern - 2010 by Google, precursor to AVIF.\
ICO (.ico) - Standard - Windows icons.\
Nikon RAW (.nef, .nrw) - Raw - Proprietary format for Nikon cameras.\
Canon RAW (.cr2, .cr3, .crw) - Raw - Canon format (CR2 = version 2, CR3 = recent version).\
Sony RAW (.arw, .srf, .sr2) - Raw - Format for Sony Alpha cameras.\
Fujifilm RAW (.raf) - Raw - Format for Fujifilm X and GFX cameras.\
Olympus RAW (.orf) - Raw - Format for Olympus cameras.\
Panasonic RAW (.rw2) - Raw - Format for Panasonic Lumix cameras.\
Pentax RAW (.pef) - Raw - Format for Pentax cameras.\
Sigma RAW (.x3f) - Raw - Format for Sigma Foveon cameras.\
DNG (.dng) - Universal - Adobe\'s open format, used by Leica, DJI, and many smartphones.\
And many more (\...) - Over 30 RAW formats supported in total.

💡 A brief history of RAW: RAW files are the \"digital negatives.\" Unlike JPEG, which applies automatic processing (white balance, sharpening, saturation), RAW retains all the data captured by the sensor. It\'s the preferred format for photographers because it offers maximum freedom in post-processing.

## 🚀 Getting Started

### Basic Usage

The most basic command to convert a photo:

img2avif -i my_photo.jpg -o my_photo.avif

To convert all photos in a folder:

img2avif -d ./my_photos -o ./avif_output

To convert recursively (including subfolders):

img2avif -r ./my_collection

The app automatically creates an AVIF file with the same name as the original in the specified folder (or next to the original if no -o is provided).

### Essential Options

Compression quality (0-100, default: 80)\
img2avif -i photo.jpg -o photo.avif -q 90

Lossless mode (larger file, maximum quality)\
img2avif -i photo.png \--lossless -o photo_lossless.avif

Delete original after conversion\
img2avif -i photo.nef \--delete -o photo.avif

Quiet mode (no messages)\
img2avif -i photo.jpg -q 70 -q

Verbose mode (see what\'s happening)\
img2avif -i photo.cr2 -v

## 🎨 RAW Development: Advanced Parameters

For photographers, the app allows you to develop your RAW files just like you would in Lightroom or Darktable. All the settings below apply only to RAW files.

### Presets: Start with a Suitable Setting

img2avif -i photo.nef \--preset landscape -o landscape.avif

landscape (default): +60% exposure, +35% saturation, +20% contrast - Landscapes, outdoors, punchy colors\
portrait: +20% exposure, +5% saturation, +2% contrast - Faces, natural skin tones\
vivid: +40% exposure, +45% saturation, +15% contrast - \"Postcard\" effect, vibrant colors\
natural: +30% exposure, +10% saturation, +5% contrast - Faithful rendering, light processing\
flat: +20% exposure, -10% saturation, -5% contrast - For later editing (maximum latitude)\
night: +90% exposure, +20% saturation, +25% contrast - Night or very dark photos\
black-white: +40% exposure, 0% saturation (monochrome), +30% contrast - Black and white\
architecture: +40% exposure, +20% saturation, +25% contrast - Buildings, lines, details\
macro: +50% exposure, +30% saturation, +15% contrast - Close-ups, flowers, insects\
sports: +45% exposure, +35% saturation, +20% contrast - Action, fast movement\
sunset: +70% exposure, +40% saturation, +15% contrast - Sunsets (warm rendering)\
winter: +55% exposure, +15% saturation, +18% contrast - Snow, bright scenes\
forest: +50% exposure, +38% saturation, +22% contrast - Forests, vegetation\
street: +35% exposure, +25% saturation, +18% contrast - Street photography\
auto: detection for exposure, automatic for saturation, based on ISO for contrast - Let the app choose

### Fine Adjustments

You can customize each parameter individually, whether in addition to a preset or not:

Adjust only the exposure of a preset:\
img2avif -i photo.nef \--preset landscape \--raw-exposure 1.8 -o brighter.avif

Set all parameters manually:\
img2avif -i photo.nef \--raw-exposure 1.6 \--raw-saturation 1.35 \--raw-contrast 1.20 \--raw-gamma 1.95 \--raw-highlight 1.0 -o custom.avif

#### Parameter Details

\--raw-exposure: 0.5 - 2.0, default 1.6 - Exposure compensation. 1.0 = no change, 2.0 = twice as bright\
\--raw-saturation: 0.0 - 2.0, default 1.35 - Color intensity. 1.0 = normal, 0.0 = monochrome, 2.0 = very saturated\
\--raw-contrast: 0.0 - 2.0, default 1.20 - Overall contrast. 1.0 = normal, \>1 = more contrast\
\--raw-gamma: 1.8 - 2.6, default 1.95 - Gamma correction. Lower = more contrast in highlights\
\--raw-highlight: 0.5 - 1.0, default 1.00 - Highlight protection. Lower = less clipping\
\--raw-temperature: 0 (auto) or 2000-12000K, default 0 (auto) - White balance. 4500K = sunset, 5500K = daylight, 6500K = cloudy sky

## 🔄 Automatic Rotation

Modern cameras record orientation in EXIF metadata. By default, the app reads this information and automatically straightens your photos:

Auto-rotation (default):\
img2avif -i portrait.nef -o portrait_straightened.avif

Disable rotation:\
img2avif -i portrait.nef \--rotate none -o portrait_lying.avif

Manual rotation (90°, 180°, 270°):\
img2avif -i photo.jpg \--rotate 90 -o rotated.avif

## 📁 Batch Processing

### Convert an Entire Folder

img2avif -d ./RAW -o ./AVIF

The app processes all image files (including RAW) in the folder, in alphabetical order, with a progress bar.

### Recursive Mode (including subfolders)

img2avif -r ./my_photo_collection

Scans all subfolders and converts every image found. AVIF files are created in the same folder as each original.

### Delete Originals After Conversion

img2avif -d ./RAW \--delete

⚠️ Warning: This action is irreversible. Make sure you have a backup!

## 💡 Practical Tips

### Which Quality to Choose?

-   85-90: Exceptional quality, for archiving or printing
-   80 (default): Excellent quality/size compromise for the web
-   70-75: Good quality, significant size reduction
-   60-65: Everyday use, slight visible loss upon inspection
-   Below 60: Quick previews, bandwidth-constrained sites

### RAW: Which Preset to Choose?

Vacation photos outdoors: landscape\
Studio portrait: portrait\
Spectacular sunset: sunset\
Dark indoor photo: night\
Technical photo for editing: flat\
Not sure: landscape (default)

### Avoid Files Larger Than the Original

img2avif -i photo.jpg \--discard-if-larger

This option compares the AVIF size with the original. If the new file is larger, the conversion is skipped. Useful for already well-optimized JPEG.

## 🐛 Troubleshooting

\"Raw decode error: unsupported format\"\
Your RAW file is not yet supported by rawloader. Check the version or use another format.

\"The image is all red\"\
White balance was not applied correctly. Use a preset or manually set \--raw-temperature and \--raw-saturation.

\"Colors are dull\"\
Increase saturation: \--raw-saturation 1.4. Also check exposure: an image that\'s too dark appears less saturated.

\"The portrait is upside down\"\
Auto-rotation didn\'t work. Use \--rotate 90 or \--rotate 270 manually.

\"No images found in directory\"\
The folder contains no files with recognized extensions. Check the supported formats (see table above).

## 📜 Options Summary

img2avif \[OPTIONS\]

INPUTS/OUTPUTS:\
-i, \--input \<FILE\> - Single file\
-d, \--directory \<DIR\> - Folder (non-recursive)\
-r, \--recursive \<DIR\> - Folder (recursive)\
-o, \--output \<PATH\> - Output file or folder

COMPRESSION:\
-q, \--quality \<0-100\> - Quality (default: 80)\
\--alpha-quality \<0-100\> - Transparency quality (default: 80)\
-s, \--speed \<0-10\> - Encoding speed (default: 4)\
\--lossless - Lossless mode\
\--discard-if-larger - Skip if AVIF is larger than original

RAW DEVELOPMENT:\
\--preset \<NAME\> - Landscape, portrait, vivid, night, etc.\
\--raw-exposure \<0.5-2.0\> - Exposure compensation\
\--raw-saturation \<0-2\> - Color saturation\
\--raw-contrast \<0-2\> - Contrast\
\--raw-gamma \<1.8-2.6\> - Gamma correction\
\--raw-highlight \<0.5-1\> - Highlight protection\
\--raw-temperature \<K\> - 2000-12000 (0 = auto)

ROTATION:\
\--rotate \<auto\|none\|90\|180\|270\> - Rotation (default: auto)

MISCELLANEOUS:\
\--delete - Delete original\
\--keep-metadata - Preserve EXIF metadata (experimental)\
-q, \--quiet - Quiet mode\
-v, \--verbose - Verbose mode\
-h, \--help - Show help\
\--version - Show version

(c) 2026 Philippe TEMESI - <https://www.tems.be/>
