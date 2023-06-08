///! Domain Manifest, description field.
#[derive(Debug)]
pub struct FortunesRequired(String);

impl FortunesRequired {
    pub fn parse(s: String) -> Result<FortunesRequired, String> {
        if !s.trim().is_empty() {
            Ok(Self(s.to_owned()))
        } else {
            Err(format!("{} is not a valid manifest url.", s))
        }
    }
}

impl AsRef<str> for FortunesRequired {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for FortunesRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
