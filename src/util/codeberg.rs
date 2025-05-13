use nyquest::{ClientBuilder, blocking::Request};
use semver::Version;
use serde::Deserialize;

const APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
const CODEBERG_ENDPOINT_ROOT: &str = "https://codeberg.org/api/v1";

#[derive(Debug)]
pub struct Release {
    pub version: Version,
    pub download_url: String,
}

impl Default for Release {
    fn default() -> Self {
        Self {
            version: Version::new(0, 0, 0),
            download_url: String::new(),
        }
    }
}

#[derive(Deserialize)]
struct LatestRelease {
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(Deserialize)]
struct Asset {
    browser_download_url: String,
}

pub fn get_latest_release() -> Result<Release, Box<dyn std::error::Error>> {
    let request_url = format!("{CODEBERG_ENDPOINT_ROOT}/repos/futile/gta-tools/releases/latest");
    let client = ClientBuilder::default()
        .user_agent(APP_USER_AGENT)
        .build_blocking()?;
    let response = client.request(Request::get(request_url))?;
    let json = response.json::<LatestRelease>()?;
    let tag_name = json.tag_name;
    let browser_download_url = &json.assets[0].browser_download_url;
    Ok(Release {
        version: Version::parse(&tag_name)?,
        download_url: browser_download_url.to_owned(),
    })
}
