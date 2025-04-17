use semver::Version;

const APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

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
    let request_url = "https://codeberg.org/api/v1/repos/futile/gta-tools/releases/latest";
    let ureq_config = ureq::Agent::config_builder()
        .user_agent(APP_USER_AGENT)
        .build();
    let ureq = ureq::Agent::new_with_config(ureq_config);
    let Ok(mut response) = ureq.get(request_url).call() else {
        return None;
    };
    let Ok(json) = response.body_mut().read_json::<serde_json::Value>() else {
        return None;
    };
    let version: Version;
    if let Some(tag_name) = &json["tag_name"].as_str() {
        version = Version::parse(tag_name).unwrap();
    } else {
        return None;
    }
    let download_url: String;
    if let Some(browser_download_url) = &json["assets"][0]["browser_download_url"].as_str() {
        download_url = String::from(*browser_download_url);
    } else {
        return None;
    }
    Some(Release {
        version,
        download_url,
    })
}
