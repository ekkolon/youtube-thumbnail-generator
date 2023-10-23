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

use std::error::Error;
use std::path::Path;

use clap::ValueEnum;
use image::DynamicImage::{self, ImageRgba8};
use image::{GenericImageView, ImageBuffer};
use strum_macros::{Display, EnumString, EnumVariantNames};

pub fn run(_args: &Args) -> Result<(), Error> {
    Ok(())
}

/// Represents an image with raw pixel data in RGBA format.
///
/// This structure is used to store image data with its width and height information.
#[derive(Clone, Debug)]
pub struct YtImage {
    /// Raw image pixels represented as RGBA pixels.
    pub raw_pixels: Vec<u8>,

    /// The image's width.
    pub width: u32,

    /// The image's height.
    pub height: u32,
}

impl YtImage {
    /// Save the image to the filesystem at a given path.
    pub fn save(&self, path: &impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
        let img_buffer =
            ImageBuffer::from_vec(self.width, self.height, self.raw_pixels.to_owned()).unwrap();

        let dyn_image = ImageRgba8(img_buffer);

        dyn_image.save(path)?;
        Ok(())
    }

    /// Convert a YtImage to a DynamicImage type (struct used by the `image` crate)
    pub fn to_dyn_image(&self) -> Result<DynamicImage, Box<dyn Error>> {
        // convert a vec of raw pixels (as u8s) to a DynamicImage type
        let _len_vec = self.raw_pixels.len() as u128;
        let raw_pixels = &self.raw_pixels;
        let img_buffer = ImageBuffer::from_vec(self.width, self.height, raw_pixels.to_vec());

        match img_buffer {
            Some(buf) => Ok(ImageRgba8(buf)),
            None => Err("Image buffer not big enough".into()),
        }
    }

    /// Return the image pixels as a byte vector.
    pub fn to_bytes(&self) -> Vec<u8> {
        let raw_pixels = &self.raw_pixels;
        let width = self.width;
        let height = self.height;

        let buf = ImageBuffer::from_vec(width, height, raw_pixels.to_owned()).unwrap();

        ImageRgba8(buf).into_bytes()
    }
}

/// Enum representing allowed formats for YouTube thumbnails.
/// These constraints are specified and taken from the
/// [Youtube Data API](https://developers.google.com/youtube/v3/docs/thumbnails/set) docs.
#[derive(ValueEnum, Display, EnumString, EnumVariantNames, Clone, Copy, Debug)]
#[strum(serialize_all = "lowercase")]
pub enum ImageFormat {
    #[strum(serialize = "png")]
    Png,

    #[strum(serialize = "jpeg", serialize = "jpg")]
    Jpeg,
}
