// Copyright 2023 Nelson Dominguez
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::path::{Path, PathBuf};

use clap::Parser;

use crate::SamplingFilter;

/// CLI arguments parsed by *clap* to configure the thumbnail generation process.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to an image from which to generate the thumbnail.
    pub path: Box<Path>,

    /// The name for the generated thumbnail.
    /// If not specified, the input filename + '_thumb' is used.
    #[arg(short = 'n', long = "name")]
    pub out_name: Option<String>,

    /// The output directory in which to store the thumbnail.
    /// Defaults to the platform-specific user `Documents` folder if unspecified.
    #[arg(short = 'd', long = "outDir")]
    pub out_dir: Option<PathBuf>,

    /// The thumbnail's output format.
    #[arg(short, long)]
    pub format: Option<String>,

    /// Sampling algorithm to use for thumbnail generation.
    #[arg(short = 's', long = "sampling")]
    pub sampling_filter: Option<SamplingFilter>,
}
