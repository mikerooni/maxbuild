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


use crate::amxd_fields::{build_frozen_device_field_padless, build_header_field};
use bytes::{BufMut, Bytes, BytesMut};

#[derive(Debug, Clone)]
pub enum DeviceType {
    AudioEffect,
    MidiEffect,
    Instrument,
    MidiToolGenerator,
    MidiToolTransformer,
}

impl DeviceType {
    fn to_header_representation(&self) -> Bytes {
        let name = match self {
            DeviceType::AudioEffect => { "aaaa" }
            DeviceType::MidiEffect => { "mmmm" }
            DeviceType::Instrument => { "iiii" }
            DeviceType::MidiToolGenerator => { "nagg" }
            DeviceType::MidiToolTransformer => { "natt" }
        };

        Bytes::from(name)
    }
}

pub fn build_frozen_amxd(device_type: DeviceType, meta: u32, data: Bytes, footer: Bytes) -> Bytes {
    let mut buf = BytesMut::new();

    buf.put(build_header_field("ampf", device_type.to_header_representation()));
    buf.put(build_header_field("meta", Bytes::from(meta.to_le_bytes().to_vec())));
    buf.put(build_header_field("ptch", build_frozen_device_body(data, footer)));

    buf.freeze()
}

fn build_frozen_device_body(data: Bytes, footer: Bytes) -> Bytes {
    let mut buf = BytesMut::new();

    buf.put(build_frozen_header(data.len() as u32 + 16));  // 16 = frozen device header
    buf.put(data);
    buf.put(footer);

    buf.freeze()
}

fn build_frozen_header(footer_location: u32) -> Bytes {
    let mut buf = BytesMut::new();

    buf.put(build_frozen_device_field_padless(
        "mx@c",
        Bytes::from((footer_location as u64).to_be_bytes().to_vec()),
    ));

    buf.freeze()
}

