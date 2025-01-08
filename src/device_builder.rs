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


use std::{fs, path};
use std::path::Path;
use std::time::SystemTime;
use bytes::{BufMut, Bytes, BytesMut};


pub enum DeviceFileType {
    JSON, // only used for max patchers (?)
    Text,
    SVG,
    PNG,
}

pub enum DeviceFileFlag {
    None,
    JSFile,
    MainFile
}

pub struct DeviceFile {
    pub file_type: DeviceFileType,
    pub file_name: String,
    pub data_size: u32,
    pub data_offset: u32,
    pub flag: DeviceFileFlag,
    pub modification_date: SystemTime,
}

pub struct DeviceData {
    pub data: Bytes,
    pub files: Vec<DeviceFile>
}

pub fn build_device(main_file_path: &String, paths: &Vec<String>) -> DeviceData {
    let mut data_buf = BytesMut::new();
    let mut files: Vec<DeviceFile> = Vec::new();

    files.push(add_file(main_file_path, DeviceFileFlag::MainFile, &mut data_buf).unwrap());
    for path in paths {
        let flag = if path.ends_with(".js") {
            DeviceFileFlag::JSFile
        } else {
            DeviceFileFlag::None
        };

        files.push(add_file(&path, flag, &mut data_buf).unwrap());
    }

    return DeviceData { data: data_buf.freeze(), files, }
}

fn add_file(file_path: &String, flag: DeviceFileFlag, data_buf: &mut BytesMut) -> Option<DeviceFile> {
    let data_offset = data_buf.len();
    let path = Path::new(file_path);

    let bytes = fs::read(path).unwrap();
    let length = bytes.len();

    println!("Packing file: {}", file_path);
    data_buf.put(Bytes::from(bytes));

    return Some(DeviceFile {
        file_type: determine_file_type(path.extension()?.to_str()?),
        file_name: path.file_name()?.to_str()?.to_owned(),
        data_size: length as u32,
        data_offset: data_offset as u32 + 16, // +16 to account for frozen device header
        flag: flag,
        modification_date: SystemTime::now()
    })
}

fn determine_file_type(extension: &str) -> DeviceFileType {
    match extension {
        "amxd" => DeviceFileType::JSON,
        "maxpat" => DeviceFileType::JSON,
        "json" => DeviceFileType::JSON,
        "svg" => DeviceFileType::SVG,
        "png" => DeviceFileType::PNG,
        _ => DeviceFileType::Text
    }
}