#[derive(Debug)]
pub struct JobEscrowId(String);

impl JobEscrowId {
    pub fn parse(s: String) -> Result<JobEscrowId, String> {
        if !s.trim().is_empty() {
            Ok(Self(s.to_owned()))
        } else {
            Err(format!("{} is not a valid escrow ID for this Job.", s))
        }
    }
}

impl AsRef<str> for JobEscrowId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for JobEscrowId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
