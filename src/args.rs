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


use clap::{Parser, ValueEnum};
use clap::builder::PossibleValue;
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
    pub device_type: DeviceType,
}



impl ValueEnum for DeviceType {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::AudioEffect,
            Self::MidiEffect,
            Self::Instrument,
            Self::MidiToolGenerator,
            Self::MidiToolTransformer,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::AudioEffect => PossibleValue::new("audio-fx"),
            Self::MidiEffect => PossibleValue::new("midi-fx"),
            Self::Instrument => PossibleValue::new("instrument"),
            Self::MidiToolGenerator => PossibleValue::new("note-generator"),
            Self::MidiToolTransformer => PossibleValue::new("note-transformer"),
        })
    }
}