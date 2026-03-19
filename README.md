Convert images (JPEG, PNG, BMP, GIF, TIFF, WebP) to AVIF format with advanced quality settings.
Supports single files, directories, and recursive processing.
Website: https://www.tems.be - (c) 2026 Philippe TEMESI

Usage: img2avif [OPTIONS] --input <PATH>

Options:
  -i, --input <PATH>
          Input file or directory

  -o, --output <PATH>
          Output file or directory (default: input.avif or input_dir/*.avif)

  -d, --directory
          Process all supported files in directory

  -r, --recursive
          Recursive directory processing

      --delete
          Delete original files after successful conversion

  -q, --quality <QUALITY>
          Compression quality (0-100, default: 80)
          
          [default: 80]

      --alpha-quality <ALPHA_QUALITY>
          Alpha channel quality (0-100, default: 80)
          
          [default: 80]

  -s, --speed <SPEED>
          Encoding speed (0-10, default: 4, 0=best compression)
          
          [default: 4]

      --color-space <COLOR_SPACE>
          Color space

          Possible values:
          - yuv420: YUV 4:2:0 (standard, good quality/size tradeoff)
          - yuv444: YUV 4:4:4 (better color quality)
          - rgb:    RGB (maximum quality, larger files)
          
          [default: yuv420]

      --bit-depth <BIT_DEPTH>
          Bit depth

          Possible values:
          - bit8:  8 bits per channel (standard)
          - bit10: 10 bits per channel (better for gradients)
          - bit12: 12 bits per channel (professional quality)
          - auto:  Automatic based on source image
          
          [default: auto]

      --lossless
          Lossless mode

      --discard-if-larger
          Don't convert if AVIF file is larger than original

      --keep-metadata
          Keep EXIF metadata (if available)

  -q, --quiet
          Quiet mode (no messages)

  -v, --verbose
          Verbose mode

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
