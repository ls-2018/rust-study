#![allow(unused)]
use jwt_simple::prelude::*;

const JWT_DURATION: u64 = 60 * 60 * 24 * 7;
const JWT_ISS: &str = "chat_server";
const JWT_AUD: &str = "chat_web";


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct User {
    pub id: i64,
    pub ws_id: i64,
    pub ws_name: String,
    pub fullname: String,
    pub email: String,
}

// go run $(go env GOROOT)/src/crypto/tls/generate_cert.go -host ca -ed25519
// openssl x509 -inform PEM -in cert.pem -pubkey -noout



#[cfg(test)]
mod tests {
    use anyhow::Error;
    use jwt_simple::prelude::*;
    use crate::{KEY_PEM, PUB_CEM};
    use super::*;

    #[test]
    fn hs256key() -> Result<(), anyhow::Error> {
        let key = HS256Key::generate();
        let claims = Claims::create(Duration::from_hours(2));
        let token = key.authenticate(claims)?;
        println!("{:?}", token);
        let claims = key.verify_token::<NoCustomClaims>(&token, None)?;
        println!("{:?}", claims);
        Ok(())
    }

    #[test]
    fn pem() -> Result<(), Error> {
        let mut sp = Ed25519KeyPair::from_pem(KEY_PEM)?;
        let user = User {
            id: 0,
            ws_id: 0,
            ws_name: "".to_string(),
            fullname: "".to_string(),
            email: "".to_string(),
        };
        let claims = Claims::with_custom_claims(user, Duration::from_secs(JWT_DURATION))
            .with_issuer(JWT_ISS)
            .with_audience(JWT_AUD);
        let token = sp.sign(claims)?;
        let opts = VerificationOptions {
            allowed_issuers: Some(HashSet::from_strings(&[JWT_ISS])),
            allowed_audiences: Some(HashSet::from_strings(&[JWT_AUD])),
            ..Default::default()
        };

        let mut sp = Ed25519PublicKey::from_pem(PUB_CEM)?;
        let claims = sp.verify_token::<User>(token.as_str(), Some(opts))?;
        println!("{:?}", claims);
        Ok(())
    }
}
