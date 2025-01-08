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


use crate::amxd_builder::{build_frozen_amxd, DeviceType};
use crate::amxd_footer::build_footer;
use crate::args::MaxBuildArgs;
use crate::device_builder::build_device;
use clap::Parser;
use std::path::Path;
use std::{fs, io};
use std::process::exit;
use bytes::{Buf, Bytes};
use crate::maxpat_project::preprocess_template_file;

mod amxd_builder;
mod amxd_fields;
mod amxd_footer;
mod args;
mod device_builder;
mod maxpat_project;

fn main() {
    let args = MaxBuildArgs::parse();
    let device_type = args.resolve_device_type();

    let mut includes: Vec<String> = Vec::new();
    for included_dir in args.include {
        add_files_recursive(&included_dir, &mut includes).unwrap();
    }

    let meta = Bytes::from(fs::read(&args.template).unwrap()[20..24].to_vec()).get_u32_le();
    let preprocessed_template = match preprocess_template_file(&args.template, &includes) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Cannot process template file:{}\n{}", e, e.backtrace());
            exit(1);
        }
    };

    let device_data = build_device(&preprocessed_template, &includes);
    let frozen_device = build_frozen_amxd(
        device_type,
        meta,
        device_data.data,
        build_footer(device_data.files),
    );

    fs::write(Path::new(&args.output_file), frozen_device.to_vec()).unwrap();
    fs::remove_dir_all(Path::new(&preprocessed_template).parent().unwrap()).unwrap();
}

fn add_files_recursive(path: &str, includes: &mut Vec<String>) -> io::Result<()> {
    if fs::metadata(path).unwrap().is_file() {
        includes.push(path.to_string());
        return Ok(());
    }

    let dir = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    for entry in dir {
        if entry.is_dir() {
            add_files_recursive(entry.as_path().to_str().unwrap(), includes)?
        } else {
            includes.push(entry.as_path().to_str().unwrap().to_string());
        }
    }

    return Ok(());
}
