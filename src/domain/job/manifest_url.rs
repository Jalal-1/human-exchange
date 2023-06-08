///! Domain Job, manifest url field.
#[derive(Debug)]
pub struct ManifestUrl(String);

impl ManifestUrl {
    pub fn parse(s: String) -> Result<ManifestUrl, String> {
        if !s.trim().is_empty() {
            Ok(Self(s.to_owned()))
        } else {
            Err(format!("{} is not a valid manifest url.", s))
        }
    }
}

impl AsRef<str> for ManifestUrl {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ManifestUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
