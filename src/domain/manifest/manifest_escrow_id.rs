#[derive(Debug)]
pub struct ManifestEscrowId(String);

impl ManifestEscrowId {
    pub fn parse(s: String) -> Result<ManifestEscrowId, String> {
        if !s.trim().is_empty() {
            Ok(Self(s.to_owned()))
        } else {
            Err(format!("{} is not a valid escrow ID for this Job.", s))
        }
    }
}

impl AsRef<str> for ManifestEscrowId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ManifestEscrowId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
