///! Domain Manifest, Title field.
#[derive(Debug)]
pub struct Title(String);

impl Title {
    pub fn parse(s: String) -> Result<Title, String> {
        if !s.trim().is_empty() {
            Ok(Self(s.to_owned()))
        } else {
            Err(format!("{} is not a valid manifest url.", s))
        }
    }
}

impl AsRef<str> for Title {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Title {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
