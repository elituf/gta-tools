use nyquest::{ClientBuilder, blocking::Request};
use semver::Version;

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
            version: Version::parse(env!("CARGO_PKG_VERSION")).unwrap(),
            download_url: String::new(),
        }
    }
}

pub fn get_latest_release() -> Option<Release> {
    let request_url = format!("{CODEBERG_ENDPOINT_ROOT}/repos/futile/gta-tools/releases/latest");
    let client = ClientBuilder::default()
        .user_agent(APP_USER_AGENT)
        .build_blocking()
        .unwrap();
    let response = client.request(Request::get(request_url)).unwrap();
    let json = response.json::<serde_json::Value>().ok()?;
    let tag_name = json["tag_name"].as_str()?;
    let browser_download_url = json["assets"][0]["browser_download_url"].as_str()?;
    Some(Release {
        version: Version::parse(tag_name).expect("expected a valid semver pattern"),
        download_url: String::from(browser_download_url),
    })
}
