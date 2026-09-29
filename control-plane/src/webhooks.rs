use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub fn verify_sha256_signature(secret: &[u8], payload: &[u8], signature: &str) -> bool {
    let Some(hex_value) = signature.strip_prefix("sha256=") else {
        return false;
    };
    let Ok(expected) = hex::decode(hex_value) else {
        return false;
    };
    let Ok(mut mac) = HmacSha256::new_from_slice(secret) else {
        return false;
    };
    mac.update(payload);
    mac.verify_slice(&expected).is_ok()
}

pub fn delivery_key(provider_connection_id: &str, delivery_id: &str) -> String {
    format!("{provider_connection_id}:{delivery_id}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signature_validation_is_fail_closed() {
        let secret = b"secret";
        let payload = b"{\"event\":\"push\"}";
        let mut mac = HmacSha256::new_from_slice(secret).expect("HMAC key");
        mac.update(payload);
        let signature = format!("sha256={}", hex::encode(mac.finalize().into_bytes()));

        assert!(verify_sha256_signature(secret, payload, &signature));
        assert!(!verify_sha256_signature(secret, b"changed", &signature));
        assert!(!verify_sha256_signature(secret, payload, "invalid"));
    }

    #[test]
    fn delivery_idempotency_key_is_connection_scoped() {
        assert_ne!(delivery_key("a", "1"), delivery_key("b", "1"));
    }
}
