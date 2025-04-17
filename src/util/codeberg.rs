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
    let ureq: ureq::Agent = ureq::Agent::config_builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .into();
    let mut response = ureq.get(request_url).call().ok()?;
    let json = response.body_mut().read_json::<serde_json::Value>().ok()?;
    let tag_name = json["tag_name"].as_str()?;
    let browser_download_url = json["assets"][0]["browser_download_url"].as_str()?;
    Some(Release {
        version: Version::parse(tag_name).expect("expected a valid semver pattern"),
        download_url: String::from(browser_download_url),
    })
}
