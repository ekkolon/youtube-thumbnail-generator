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
use std::{env::current_dir, error::Error};

use clap::Parser;
use directories::UserDirs;
use path_clean::PathClean;

use crate::{sampling::SamplingFilter, ImageFormat};

/// CLI arguments parsed by *clap* to configure the thumbnail generation process.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Local image file path for generating the thumbnail
    pub path: Box<Path>,

    /// Generated thumbnail name.
    ///
    /// If not specified, the input filename is used with '_thumb' appended.
    #[arg(short = 'n', long = "name")]
    pub out_name: Option<String>,

    /// Specifies the output directory for the thumbnail.
    ///
    /// If unspecified, it defaults to the user's platform-specific 'Pictures' folder
    #[arg(short = 'd', long = "outDir")]
    pub out_dir: Option<PathBuf>,

    /// The thumbnail's output format.
    #[arg(short, long, default_value_t = ImageFormat::Png)]
    pub format: ImageFormat,

    /// Sampling algorithm to use for thumbnail generation.
    #[arg(short = 's', long = "sampling", default_value_t = SamplingFilter::Lanczos3)]
    pub sampling_filter: SamplingFilter,
}

/// Represents CLI arguments parsed by `clap` with reasonable
/// defaults for arguments whose parsed values are *None*.
#[derive(Debug)]
pub struct NormalizedArgs {
    /// Local image file path for generating the thumbnail
    pub path: Box<Path>,

    /// Generated thumbnail name.
    pub out_name: String,

    /// Specifies the output directory for the thumbnail.
    pub out_dir: PathBuf,

    /// The thumbnail's output format.
    pub format: ImageFormat,

    /// Sampling algorithm to use for thumbnail generation.
    pub sampling_filter: SamplingFilter,
}

impl Args {
    /// Returns parsed arguments with reasonable defaults.
    pub fn normalize(self) -> NormalizedArgs {
        let out_name = self.out_name.unwrap_or_else(|| {
            let p_without_ext = self.path.with_extension("");
            let original_filename = p_without_ext.file_name().unwrap();

            if original_filename.is_empty() {
                panic!("Invalid empty path to image file")
            }

            format!(
                "{}_thumb.{}",
                &original_filename.to_str().unwrap(),
                &self.format
            )
        });

        let path = abs_path_to_img(&self.path).unwrap();

        NormalizedArgs {
            path,
            out_name,
            out_dir,
            format: self.format,
            sampling_filter: self.sampling_filter,
        }
    }
}

impl NormalizedArgs {
    pub fn get_final_output_path(&self) -> PathBuf {
        self.out_dir.join(&self.out_name)
    }
}

fn default_output_dir() -> PathBuf {
    UserDirs::new()
        .unwrap_or_else(|| {
            panic!("Unable to find user-facing standard directories in the operating sytem")
        })
        .picture_dir()
        .unwrap_or_else(|| {
            panic!("Unable to find user 'Pictures' directory in the operating sytem")
        })
        .to_path_buf()
}

fn abs_path_to_img(path: &impl AsRef<Path>) -> Result<Box<Path>, Box<dyn Error>> {
    let path = path.as_ref();

    let absolute_path = if path.is_absolute() {
        path.to_path_buf().into_boxed_path()
    } else {
        current_dir()?.join(path).into_boxed_path()
    }
    .clean();

    Ok(absolute_path.into_boxed_path())
}
