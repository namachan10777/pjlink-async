// Copyright 2018 Rick Russell
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use clap::Parser;
use pjlink::{InputType, PjlinkDevice};

#[derive(Parser)]
struct Opts {
    host: String,
    #[clap(short, long)]
    password: Option<String>,
}

#[tokio::main]
async fn main() {
    let opts = Opts::parse();

    let device: PjlinkDevice = if let Some(password) = opts.password {
        PjlinkDevice::new_with_password(&opts.host, &password).unwrap()
    } else {
        PjlinkDevice::new(&opts.host).unwrap()
    };

    match device.set_input(InputType::Digital(4)).await {
        Ok(input) => match input {
            InputType::RGB(input_number) => println!("Input: RGB {}", input_number),
            InputType::Video(input_number) => println!("Input: Video {}", input_number),
            InputType::Digital(input_number) => println!("Input: Digital {}", input_number),
            InputType::Storage(input_number) => println!("Input: Storage {}", input_number),
            InputType::Network(input_number) => println!("Input: Network {}", input_number),
        },
        Err(err) => println!("An error occurred: {}", err),
    }
}
