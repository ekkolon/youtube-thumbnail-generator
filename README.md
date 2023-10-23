# Youtube Thumbnail Generator

Lightning-fast command-line tool to generate high-quality and optimized YouTube thumbnails.

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
