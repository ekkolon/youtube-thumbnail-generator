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

use clap::Parser;

use ytthumb::{cli::Args, run};

fn main() {
    let args = Args::parse();

    let img_result = run(&args);

    match img_result {
        Ok(_) => println!("Successfully generated thumbnail"),
        Err(err) => panic!(
            "An error occured during the thumbnail generation process: {:?}",
            err
        ),
    }
}
