mod downloader;
mod utils;
mod extractor;

use std::error::Error;
use std::process::exit;
use crate::downloader::Downloader;
use crate::extractor::Extractor;

pub static VERSION_FILE :&str = "./autoupdate_version";
pub static DOWNLOAD_LINK :&str = "https://minecraft.net/en-us/download/server/bedrock/";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let version = match utils::read_version(VERSION_FILE)? {
        Some(v ) => v,
        None => {
            eprintln!("version file parse error");
            exit(1);
        },
    };

    let extractor = Extractor::new("./minecraft", vec!["allowlist.json", "permissions.json", "server.properties", "worlds"]);
    let mut client = Downloader::new(DOWNLOAD_LINK)?;
    client.fetch().await?;
    println!("{}", client.is_update(version));

    let data = client.download().await?;
    extractor.extract(data)?;

    utils::write_version(VERSION_FILE, client.get_online_version().unwrap())?;

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let  v = vec!["allowlist.json", "behavior_packs/chemistry/", "behavior_packs/chemistry/entities/", "behavior_packs/chemistry/entities/balloon.json", "behavior_packs/chemistry/manifest.json"];

        println!("{:?}", v);
    }
}