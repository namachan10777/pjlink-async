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

extern crate pjlink;

use clap::Parser;
use pjlink::{PjlinkDevice, PowerStatus};

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

    match device.power_on().await {
        Ok(response) => match response {
            PowerStatus::Off => println!("{} is off", opts.host),
            PowerStatus::On => println!("{} is on", opts.host),
            PowerStatus::Cooling => println!("{} is cooling", opts.host),
            PowerStatus::Warmup => println!("{} is warming up", opts.host),
        },
        Err(err) => println!("An error occurred: {}", err),
    }
}
