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


use bytes::{BufMut, Bytes, BytesMut};

pub fn build_frozen_device_field(field_type: &str, data: Bytes) -> Bytes {
    build_field(field_type, data, true, true, true)
}
pub fn build_frozen_device_field_padless(field_type: &str, data: Bytes) -> Bytes {
    build_field(field_type, data, true, true, false)
}

pub fn build_header_field(field_type: &str, data: Bytes) -> Bytes {
    build_field(field_type, data, false, false, true)
}

fn build_field(
    field_type: &str,
    data: Bytes,
    big_endian: bool,
    include_desc_length: bool,
    pad_end: bool,
) -> Bytes {
    let mut buf = BytesMut::new();

    let add_length = if include_desc_length { 8 } else { 0 };
    let raw_padding_length = (data.len() as u32 + add_length) % 4;
    let padding_length = if raw_padding_length == 0 || !pad_end {
        0
    } else {
        4 - raw_padding_length
    };      
    
    let data_length = data.len() as u32 + padding_length + add_length;

    buf.put(Bytes::from(field_type.to_owned()));
    if big_endian {
        buf.put_u32(data_length);
    } else {
        buf.put_u32_le(data_length);
    }
    buf.put(data);
    buf.put_bytes(0, padding_length as usize);

    buf.freeze()
}
