use base64::Engine;
use sha2::Digest;
use sha2::Sha256;

pub fn hash_sha256(bytes: &[u8]) -> String {
    let result = Sha256::digest(bytes);
    base64::prelude::BASE64_STANDARD.encode(result).to_string()
}
