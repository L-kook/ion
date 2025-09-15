use base64::Engine;
use sha2::{Digest, Sha256};

pub fn hash_sha256(bytes: &[u8]) -> String {
  let result = Sha256::digest(bytes);
  format!("{}", base64::prelude::BASE64_STANDARD.encode(result))
}
