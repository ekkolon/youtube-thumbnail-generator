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

\<PATH> Path to an image from which to generate the thumbnail

### <ins>Options</ins>

**-n**, **--name** \<OUT_NAME>

The name for the generated thumbnail. If not specified, the input filename + '\_thumb' is used

**-d**, **--outDir** \<OUT_DIR>

The output directory in which to store the thumbnail. Defaults to the platform-specific user `Documents` folder if unspecified

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
