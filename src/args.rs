/*
 *  Copyright 2025 Mikerooni
 *  
 *  Licensed under the Apache License, Version 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *  
 *     http://www.apache.org/licenses/LICENSE-2.0
 *  
 *  Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 */


use std::process::exit;
use clap::Parser;
use crate::amxd_builder::DeviceType;

#[derive(Parser, Debug)]
#[command(version = "0.0.1")]
pub struct MaxBuildArgs {
    #[arg(short, long, required = true)]
    pub template: String,

    #[arg(short, long, required = true)]
    pub output_file: String,

    #[arg(short, long)]
    pub include: Vec<String>,
    
    #[arg(short, long, required = true)]
    pub device_type: String,
}

impl MaxBuildArgs {
    pub fn resolve_device_type(&self) -> DeviceType {
        match self.device_type.as_str() {
            "instrument" => DeviceType::Instrument,
            "audio-fx" => DeviceType::AudioEffect,
            "midi-fx" => DeviceType::MidiEffect,
            "note-generator" => DeviceType::MidiToolGenerator,
            "note-transformer" => DeviceType::MidiToolTransformer,
            unknown => {
                println!("Unknown device type: {}", unknown);
                println!("Valid device types:");
                println!("- instrument");
                println!("- audio-fx");
                println!("- midi-fx");
                println!("- note-generator");
                println!("- note-transformer");

                exit(1);
            },
        }
    }
}