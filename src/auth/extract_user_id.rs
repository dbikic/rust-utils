use anyhow::{bail, Error};
use biscuit::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Claims {
    pub mock_private_claim: Option<String>,
}

pub fn extract_user_id(user_token: String) -> Result<String, Error> {
    let token = JWT::<ClaimsSet<Claims>, biscuit::Empty>::new_encoded(&user_token);
    match token.unverified_payload() {
        Ok(claims) => match claims.registered.subject {
            None => bail!("User id can't be extracted from the token!"),
            Some(subject) => Ok(subject),
        },
        Err(_) => bail!("Invalid JWT token!"),
    }
}
