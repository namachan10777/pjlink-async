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
use pjlink::{ErrorType, InputType, PjlinkDevice, PowerStatus};

#[derive(clap::Parser)]
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

    match device.get_device_name().await {
        Ok(response) => println!("{} Device Name: {}", opts.host, response),
        Err(err) => println!("{} Device: error occurred: {}", opts.host, err),
    }

    match device.get_manufacturer().await {
        Ok(response) => println!("{} Manufacturer: {}", opts.host, response),
        Err(err) => println!("{} Manugacturer: error occurred: {}", opts.host, err),
    }

    match device.get_product_name().await {
        Ok(response) => println!("{} Product: {}", opts.host, response),
        Err(err) => println!("{} Product: error occurred: {}", opts.host, err),
    }

    match device.get_info().await {
        Ok(response) => println!("{} Infomation: {}", opts.host, response),
        Err(err) => println!("{} Info: error occurred: {}", opts.host, err),
    }

    match device.get_class().await {
        Ok(response) => println!("{} Class: {}", opts.host, response),
        Err(err) => println!("{} Class: error occurred: {}", opts.host, err),
    }

    match device.get_power_status().await {
        Ok(response) => match response {
            PowerStatus::Off => println!("{} Power: off", opts.host),
            PowerStatus::On => println!("{} Power: on", opts.host),
            PowerStatus::Cooling => println!("{} Power: cooling", opts.host),
            PowerStatus::Warmup => println!("{} Power: warming up", opts.host),
        },
        Err(err) => println!("{} Power: error occurred: {}", opts.host, err),
    }

    match device.get_input().await {
        Ok(input) => match input {
            InputType::RGB(input_number) => println!("{} Input: RGB {}", opts.host, input_number),
            InputType::Video(input_number) => {
                println!("{} Input: Video {}", opts.host, input_number)
            }
            InputType::Digital(input_number) => {
                println!("{} Input: Digital {}", opts.host, input_number)
            }
            InputType::Storage(input_number) => {
                println!("{} Input: Storage {}", opts.host, input_number)
            }
            InputType::Network(input_number) => {
                println!("{} Input: Network {}", opts.host, input_number)
            }
        },
        Err(err) => println!("{} Input: error occurred: {}", opts.host, err),
    }

    match device.get_avmute().await {
        Ok(response) => println!(
            "{} Video Mute: {} Audio Mute: {}",
            opts.host, response.video, response.audio
        ),
        Err(err) => println!("{} AvMute: error occurred: {}", opts.host, err),
    }

    match device.get_lamp().await {
        Ok(response) => {
            let mut lamp_count = 1;
            for lamp in response.iter() {
                println!(
                    "{} Lamp {}: Hours: {} On: {}",
                    opts.host, lamp_count, lamp.hours, lamp.on
                );
                lamp_count += 1;
            }
        }
        Err(err) => println!("{} Lamp: error occurred: {}", opts.host, err),
    }

    match device.get_error_status().await {
        Ok(error_status) => {
            match error_status.fan_error {
                ErrorType::Warning => println!("{} Error Status: Fan Warning", opts.host),
                ErrorType::Error => println!("{} Error Status: Fan Error", opts.host),
                _ => (),
            }
            match error_status.lamp_error {
                ErrorType::Warning => println!("{} Error Status: Lamp Warning", opts.host),
                ErrorType::Error => println!("{} Error Status: Lamp Error", opts.host),
                _ => (),
            }
            match error_status.temperature_error {
                ErrorType::Warning => println!("{} Error Status: Temperature Warning", opts.host),
                ErrorType::Error => println!("{} Error Status: Temperature Error", opts.host),
                _ => (),
            }
            match error_status.cover_open_error {
                ErrorType::Warning => println!("{} Error Status: Cover Open Warning", opts.host),
                ErrorType::Error => println!("{} Error Status: Cover Open Error", opts.host),
                _ => (),
            }
            match error_status.filter_error {
                ErrorType::Warning => println!("{} Error Status: Filter Warning", opts.host),
                ErrorType::Error => println!("{} Error Status: Filter Error", opts.host),
                _ => (),
            }
            match error_status.other_error {
                ErrorType::Warning => println!("{} Error Status: Other Warning", opts.host),
                ErrorType::Error => println!("{} Error Status: Other Error", opts.host),
                _ => (),
            }
        }
        Err(err) => println!("{} Error Status: error occurred: {}", opts.host, err),
    }
}
