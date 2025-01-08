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


use std::time::SystemTime;
use bytes::{BufMut, Bytes, BytesMut};
use crate::amxd_fields::{build_frozen_device_field};
use crate::device_builder::{DeviceFile, DeviceFileFlag, DeviceFileType};

pub fn build_footer(files: &[DeviceFile]) -> Bytes{
    let mut buf = BytesMut::new();

    for file in files {
        buf.put(file.to_footer_field())
    }

    build_frozen_device_field("dlst", buf.freeze())
}

impl DeviceFileType {
    fn to_field_representation(&self) -> Bytes {
        let name = match self {
            DeviceFileType::Json => { "JSON" }
            DeviceFileType::Text => { "TEXT" }
            DeviceFileType::Svg => { "svg " }
            DeviceFileType::Png => { "PNG " }
        };

        Bytes::from(name)
    }
}

impl DeviceFileFlag {
    fn to_bytes(&self) -> Bytes {
        let raw_flag: u32 = match self {
            DeviceFileFlag::None => { 0 }
            DeviceFileFlag::JSFile => { 8 }
            DeviceFileFlag::MainFile => { 17 }
        };

        Bytes::from(raw_flag.to_be_bytes().to_vec())
    }
}

impl DeviceFile {
    fn to_footer_field(&self) -> Bytes {
        let hfsplus_time:u32 = to_hfsplus_time(&self.modification_date);

        let mut buf = BytesMut::new();
        buf.put(build_frozen_device_field("type", self.file_type.to_field_representation()));
        buf.put(build_frozen_device_field("fnam", Bytes::from(self.file_name.to_owned() + "\x00")));
        buf.put(build_frozen_device_field("sz32", Bytes::from(self.data_size.to_be_bytes().to_vec())));
        buf.put(build_frozen_device_field("of32", Bytes::from(self.data_offset.to_be_bytes().to_vec())));
        buf.put(build_frozen_device_field("flag", self.flag.to_bytes()));
        buf.put(build_frozen_device_field("mdat", Bytes::from(hfsplus_time.to_be_bytes().to_vec())));
        buf.put(build_frozen_device_field("vers", Bytes::from(0u32.to_be_bytes().to_vec())));

        build_frozen_device_field("dire", buf.freeze())
    }
}

fn to_hfsplus_time(system_time: &SystemTime) -> u32 {
    const HFSPLUS_OFFSET: u64 = 2082844800;

    let unix_time = system_time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    (unix_time + HFSPLUS_OFFSET) as u32
}