use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use serde_json::json;

pub fn generate(user_id: i64) -> Result<String, Box<dyn std::error::Error>> {
    let master_key = std::env::var("MASTER_KEY").expect("MASTER_KEY is expected");
    let key: Hmac<sha2::Sha256> = Hmac::new_from_slice(master_key.as_bytes())?;
    let body = json!({ "user_id": user_id });
    Ok(body.sign_with_key(&key).unwrap())
}
