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


use anyhow::Result;
use bytes::{BufMut, Bytes, BytesMut};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;
use std::time::SystemTime;
use std::{env, fs};


#[derive(Serialize, Deserialize)]
struct ProjectFile {
    kind: String,
    local: u8,
}

impl ProjectFile {
    pub fn new(kind: &str) -> Self {
        Self { kind: kind.to_string(), local: 1 }
    }
}

#[derive(Serialize, Deserialize, Default)]
struct ProjectContents {
    patchers: HashMap<String, ProjectFile>,
    media: HashMap<String, ProjectFile>,
    code: HashMap<String, ProjectFile>,
    data: HashMap<String, ProjectFile>,
}

pub fn preprocess_template_file(template_path: &str, files: &Vec<String>) -> Result<String> {
    let mut maxpat_json = parse_maxpat_json(&template_path)?;
    let project = maxpat_json["patcher"]["project"].as_object_mut().unwrap();

    let project_contents = build_prject_contents(files);
    project.insert("contents".to_string(), serde_json::to_value(project_contents)?, );

    return write_template(&template_path, maxpat_json);
}

fn build_prject_contents(files: &Vec<String>) -> ProjectContents {
    let mut contents = ProjectContents::default();

    for file in files {
        let path = Path::new(file);
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        match path.extension().map(|e| e.to_ascii_lowercase()) {
            Some(x) if x == "maxpat" => {
                contents.patchers.insert(file_name, ProjectFile::new("patcher"));
            }
            Some(x) if x == "js" => {
                contents.code.insert(file_name, ProjectFile::new("javascript"));
            }
            Some(x) if x == "svg" => {
                contents.media.insert(file_name, ProjectFile::new("vectorimagefile"));
            }
            Some(x) if x == "png" || x == "jpg" => {
                contents.media.insert(file_name, ProjectFile::new("imagefile"));
            }
            None | Some(_) => {
                contents.data.insert(file_name, ProjectFile::new("textfile"));
            },
        };
    }

    return contents;
}

fn write_template(template_path: &str, template: Value) -> Result<String> {
    let mut output_file = env::temp_dir();
    output_file.push("maxbuild");
    output_file.push(SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_nanos()
        .to_string());
    fs::create_dir_all(&output_file)?;

    let mut buf = BytesMut::new();
    buf.put(Bytes::from(serde_json::to_vec_pretty(&template)?));
    buf.put_u8(0);

    output_file.push(Path::new(template_path).file_name().unwrap());
    fs::write(&output_file, buf.to_vec())?;

    return Ok(output_file.to_str().unwrap().to_string());
}

fn parse_maxpat_json(template_path: &str) -> Result<Value> {
    let file_contents = fs::read(template_path)?;
    // Skip header, as well as the null byte at the end:
    let maxpat_contents = &file_contents[32..file_contents.len() - 1];
    let json_contents = serde_json::from_slice(maxpat_contents)?;

    return Ok(json_contents);
}
