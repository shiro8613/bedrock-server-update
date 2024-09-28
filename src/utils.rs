use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use regex::Regex;
use reqwest::Url;

pub fn read_version(path :&str) -> Result<Option<String>, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut text = String::new();
    if file.read_to_string(&mut text).is_ok() {
        let re = Regex::new(r"^([0-9]|\.)+$")?;
        if re.is_match(text.as_str()) {
            Ok(Some(text))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

pub fn write_version(path: &str, version :String) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(path)?;
    Ok(file.write(version.as_bytes()).map(|_| ())?)
}

pub fn parse_version(file_name :String) -> String {
    let url = Url::parse(file_name.as_str()).unwrap();
    let path = url.path_segments().map(|c| c.collect::<Vec<_>>()).unwrap();
    if let Some(&name) = path.get(path.len() -1) {
        let version = name.replace(".zip", "").replace("bedrock-server-", "");
        version
    } else {
        String::new()
    }
}
