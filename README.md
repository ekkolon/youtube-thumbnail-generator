# YouTube Thumbnail Generator

> This project is a work in progress.

Fast, cross-platform CLI tool for generating high-quality YouTube thumbnails.

## Table of Contents

- [CLI Reference](#cli-reference)
- [Authors](#authors)
- [License](#license)
- [Roadmap](#roadmap)
- [Feedback](#feedback)

## CLI Reference

### <ins>Usage</ins>

**ytthumb** [OPTIONS] \<PATH>

### <ins>Arguments</ins>

\<PATH> Local image file path for generating the thumbnail

### <ins>Options</ins>

**-n**, **--name** \<OUT_NAME>

Generated thumbnail name. If not specified, the input filename is used with '\_thumb' appended

**-d**, **--outDir** \<OUT_DIR>

Specifies the output directory for the thumbnail. If unspecified, it defaults to the user's platform-specific _Pictures_ folder

**-f**, **--format** \<FORMAT>

The thumbnail's output format

<ins>Possible values</ins>:

- png
- jpg / jpeg

**-s**, **--sampling** \<SAMPLING_FILTER>

Sampling algorithm to use for thumbnail generation

<ins>Possible values</ins>:

- nearest: Nearest Neighbor
- triangle: Linear Filter
- catmull-rom: Cubic Filter
- gaussian: Gaussian Filter
- lanczos3: Lanczos with window 3

**-h**, **--help**

Print help (see a summary with '-h')

**-V**, **--version**

Print version

## Authors

- Nelson Dominguez ([@ekkolon](https://www.github.com/ekkolon))

## License

This project is licensed under the Apache License, Version 2.0 - see the [LICENSE](LICENSE) file for details.

## Roadmap

This project's roadmap will be announced soon. Stay tuned for updates!

## Feedback

If you have any feedback, please reach out at [ekkolon@proton.me](mailto:ekkolon@proton.me)
