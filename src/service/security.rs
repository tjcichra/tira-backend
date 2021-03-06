use crypto::{digest::Digest, sha2::Sha256};

/// Service function for putting a string through sha256
pub fn sha256(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(password);
    hasher.result_str()
}
