///! Domain Manifest, description field.
#[derive(Debug)]
pub struct Description(String);

impl Description {
    pub fn parse(s: String) -> Result<Description, String> {
        if !s.trim().is_empty() {
            Ok(Self(s.to_owned()))
        } else {
            Err(format!("{} is not a valid manifest url.", s))
        }
    }
}

impl AsRef<str> for Description {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Description {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
