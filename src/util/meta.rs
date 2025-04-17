use crate::util::codeberg;
use semver::Version;

#[derive(Debug)]
pub struct Meta {
    pub current_version: Version,
    pub latest_release: codeberg::Release,
    pub newer_version_available: bool,
}

impl Default for Meta {
    fn default() -> Self {
        let current_version = Version::parse(env!("CARGO_PKG_VERSION")).unwrap();
        let latest_release = codeberg::get_latest_release().unwrap_or_default();
        let newer_version_available = matches!(
            &current_version.cmp_precedence(&latest_release.version),
            std::cmp::Ordering::Less
        );
        Self {
            current_version,
            latest_release,
            newer_version_available,
        }
    }
}
