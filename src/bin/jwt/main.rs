use std::{env, fs::File, io::Read, time::SystemTime};

use hmac::{Mac};
use jwt::{PKeyWithDigest, SignWithKey};
use openssl::{hash::MessageDigest, pkey::PKey};
use serde_json::json;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let mut file = File::open("/Users/z/Downloads/emoji-to-do-dev.2022-08-07.private-key.pem")?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;

    let private_key = PKey::private_key_from_pem(s.as_bytes())?;
    let github_app_id = env::var("GITHUB_APP_ID")?;
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();

    let body = json!({
        "iat": now - 60,
        "exp": now + (10 * 60),
        "iss": github_app_id
    });
    let key = PKeyWithDigest {
        digest: MessageDigest::sha256(),
        key: private_key,
    };
    let jwt = body.sign_with_key(&key)?;
    println!("{}", jwt);

    Ok(())
}
