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


use crate::max_filetypes::ProjectContentSection::*;

pub enum ProjectContentSection {
    Patchers,
    Media,
    Code,
    Data,
    Externals,
    Other,
}

pub struct MaxFileType {
    pub four_character_code: String,
    pub project_file_type: String,
    pub project_content_section: ProjectContentSection,
}

impl MaxFileType {
    fn new(
        four_character_code: &str,
        project_file_type: &str,
        project_content_section: ProjectContentSection,
    ) -> MaxFileType {
        MaxFileType {
            four_character_code: four_character_code.to_string(),
            project_file_type: project_file_type.to_string(),
            project_content_section,
        }
    }
}

pub fn determine_file_type(extension: &str) -> MaxFileType {
    match extension.to_ascii_lowercase().as_str() {
        "aif" => MaxFileType::new("AIFF", "audiofile", Media),
        "aiff" => MaxFileType::new("AIFF", "audiofile", Media),
        "amp" => MaxFileType::new("ampf", "livedevice", Other),
        "amxd" => MaxFileType::new("amxd", "maxforlive", Patchers),
        "app" => MaxFileType::new("APPL", "file", Other),
        "asf" => MaxFileType::new("WMV2", "moviefile", Media),
        "auinfo" => MaxFileType::new("AUin", "file", Other),
        "avi" => MaxFileType::new("VfW", "moviefile", Media),
        "b3d" => MaxFileType::new("Jb3d", "model", Media),
        "bmp" => MaxFileType::new("BMP", "imagefile", Media),
        "bvh" => MaxFileType::new("Jbvh", "model", Media),
        "caf" => MaxFileType::new("CAF", "audiofile", Media),
        "class" => MaxFileType::new("cafe", "java", Code),
        "clct" => MaxFileType::new("maxc", "collective", Other),
        "component" => MaxFileType::new("AUpi", "file", Other),
        "css" => MaxFileType::new("css", "stylesheet", Other),
        "dae" => MaxFileType::new("Jdae", "model", Media),
        "data" => MaxFileType::new("DATA", "audiofile", Media),
        "dll" => MaxFileType::new("aPcs", "audioplugin", Other),
        "exe" => MaxFileType::new("APPL", "application", Other),
        "fbx" => MaxFileType::new("FBX", "model", Media),
        "flac" => MaxFileType::new("FLAC", "audiofile", Media),
        "folder" => MaxFileType::new("fold", "file", Other),
        "fxb" => MaxFileType::new("AFxB", "file", Other),
        "fxp" => MaxFileType::new("AFxP", "file", Other),
        "gendsp" => MaxFileType::new("gDSP", "gendsp", Code),
        "genexpr" => MaxFileType::new("GenX", "genexpr", Other),
        "genjit" => MaxFileType::new("gJIT", "genjit", Code),
        "gif" => MaxFileType::new("GIFf", "imagefile", Media),
        "glsl" => MaxFileType::new("TEXT", "shader", Code),
        "help" => MaxFileType::new("TEXT", "helpfile", Patchers),
        "hibundle" => MaxFileType::new("xQZZ", "file", Other),
        "htm" => MaxFileType::new("TEXT", "webpage", Other),
        "html" => MaxFileType::new("TEXT", "webpage", Other),
        "jar" => MaxFileType::new("jar", "java", Code),
        "java" => MaxFileType::new("TEXT", "java", Code),
        "jit" => MaxFileType::new("JiT!", 	"jitterdatafile",	Data),
        "jitmtl" => MaxFileType::new("Jmtl", "material", Media),
        "jpeg" => MaxFileType::new("JPEG", "imagefile", Media),
        "jpg" => MaxFileType::new("JPEG", "imagefile", Media),
        "js" => MaxFileType::new("TEXT", "javascript", Code),
        "json" => MaxFileType::new("JSON", "json", Data),
        "jxf" => MaxFileType::new("JiT!", "jitterdatafile",	Data),
        "jxp" => MaxFileType::new("TEXT", "pass", Code),
        "jxs" => MaxFileType::new("TEXT", "shader", Code),
        "lua" => MaxFileType::new("jlua", "lua", Code),
        "m4a" => MaxFileType::new("M4a", "audiofile", Media),
        "maxcoll" => MaxFileType::new("mQur", "queryfile", Other),
        "maxdefaults" => MaxFileType::new("JSON", "file", Other),
        "maxdefines" => MaxFileType::new("JSON", "file", Other),
        "maxdict" => MaxFileType::new("dict", "file", Other),
        "maxhelp" => MaxFileType::new("JSON", "helpfile", Patchers),
        "maxlesson" => MaxFileType::new("mLsn", "lesson", Other),
        "maxmap" => MaxFileType::new("mMap", "maxdatafile", Data),
        "maxpack" => MaxFileType::new("mPak", "file", Other),
        "maxpalette" => MaxFileType::new("mxPL", "file", Other),
        "maxpat" => MaxFileType::new("JSON", "patcher", Patchers),
        "maxpref" => MaxFileType::new("JSON", "file", Other),
        "maxpresets" => MaxFileType::new("JSON", "maxdatafile", Data),
        "maxproj" => MaxFileType::new("mPrj", "project", Other),
        "maxproto" => MaxFileType::new("JSON", "prototype", Other),
        "maxquery" => MaxFileType::new("JSON", "queryfile", Other),
        "maxrefxml" => MaxFileType::new("TEXT", "file", Other),
        "maxsnip" => MaxFileType::new("mSnp", "snippetfile", Patchers),
        "maxswatches" => MaxFileType::new("JSON", "file", Other),
        "maxtutxml" => MaxFileType::new("TEXT", "file", Other),
        "maxvigxml" => MaxFileType::new("TEXT", "file", Other),
        "maxzip" => MaxFileType::new("mZip", "project", Other),
        "meshxml" => MaxFileType::new("Jogr", "file", Other),
        "mid" => MaxFileType::new("Midi", "midifile", Other),
        "midi" => MaxFileType::new("Midi", "midifile", Other),
        "mov" => MaxFileType::new("MooV", "moviefile", Media),
        "mp3" => MaxFileType::new("Mp3", "audiofile", Media),
        "mp4" => MaxFileType::new("mpg4", "moviefile", Media),
        "mpeg" => MaxFileType::new("MPEG", "moviefile", Media),
        "mpg" => MaxFileType::new("MPEG", "moviefile", Media),
        "mxb" => MaxFileType::new("maxb", "patcher", Patchers),
        "mxc" => MaxFileType::new("maxc", "collective", Other),
        "mxd" => MaxFileType::new("iLaF", "file", Other),
        "mxe" => MaxFileType::new("iLaF", "object", Externals),
        "mxe64" => MaxFileType::new("mx64", "object", Externals),
        "mxf" => MaxFileType::new("mx@c", "collective",	Other),
        "mxo" => MaxFileType::new("iLaX", "object", Externals),
        "mxt" => MaxFileType::new("TEXT", "patcher", Patchers),
        "obj" => MaxFileType::new("Jobj", "model", Media),
        "pat" => MaxFileType::new("maxb", "patcher", Patchers),
        "pct" => MaxFileType::new("PICT", "imagefile", Media),
        "pics" => MaxFileType::new("PICS", "imagefile", Media),
        "pict" => MaxFileType::new("PICT", "imagefile", Media),
        "ply" => MaxFileType::new("Jply", "model", Media),
        "png" => MaxFileType::new("PNG", "imagefile", Media),
        "psd" => MaxFileType::new("8BPS", "imagefile", Media),
        "snd" => MaxFileType::new("ULAW", "audiofile", Media),
        "stl" => MaxFileType::new("Jstl", "model", Media),
        "svg" => MaxFileType::new("svg", "vectorimagefile", Media),
        "swf" => MaxFileType::new("SWFL", "file", Other),
        "syx" => MaxFileType::new("Midi", "midifile", Other),
        "tif" => MaxFileType::new("TIFF", "imagefile", Media),
        "tiff" => MaxFileType::new("TIFF", "imagefile", Media),
        "txt" => MaxFileType::new("TEXT", "textfile", Data),
        "vst" => MaxFileType::new("aPcs", "file", Other),
        "wav" => MaxFileType::new("WAVE", "audiofile", Media),
        "wmv" => MaxFileType::new("WMVA", "moviefile", Media),
        "xhtml" => MaxFileType::new("TEXT", "webpage", Other),
        "xml" => MaxFileType::new("TEXT", "xmlfile", Data),
        "xsl" => MaxFileType::new("XSLT", "stylesheet", Other),
        "yaml" => MaxFileType::new("YAML", "yaml", Other),
        "yml" => MaxFileType::new("YAML", "yaml", Other),
        "zip" => MaxFileType::new("ZIP", "file", Other),
        
        _ => MaxFileType::new("DATA", "file", ProjectContentSection::Other),
    }
}
