use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use sha2::{Digest, Sha256};

/// Verifiziert PKCE gemäß RFC 7636.
///
/// Der Server prüft: BASE64URL(SHA256(code_verifier)) == gespeicherter code_challenge
///
/// # Argumente
/// * `code_verifier`   – vom Client beim Token-Request gesendet
/// * `code_challenge`  – beim Authorization-Request gespeicherter Wert
///
/// # Rückgabe
/// `true` wenn der Verifier zur Challenge passt, sonst `false`
pub fn verify_pkce(code_verifier: &str, code_challenge: &str) -> bool {
    if code_verifier.is_empty() || code_challenge.is_empty() {
        return false;
    }

    let hash = Sha256::digest(code_verifier.as_bytes());
    let computed = URL_SAFE_NO_PAD.encode(hash);

    // Timing-sicherer Vergleich (verhindert Timing-Angriffe)
    constant_time_eq(&computed, code_challenge)
}

/// Timing-sicherer String-Vergleich (verhindert Timing-Angriffe)
fn constant_time_eq(a: &str, b: &str) -> bool {
    let a = a.as_bytes();
    let b = b.as_bytes();
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b.iter()).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_pkce() {
        // Beispiel aus RFC 7636 Appendix B
        let verifier = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk";
        let challenge = URL_SAFE_NO_PAD.encode(Sha256::digest(verifier.as_bytes()));
        assert!(verify_pkce(verifier, &challenge));
    }

    #[test]
    fn test_invalid_pkce() {
        let verifier = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk";
        let wrong_challenge = "falsche-challenge";
        assert!(!verify_pkce(verifier, wrong_challenge));
    }

    #[test]
    fn test_empty_values() {
        assert!(!verify_pkce("", "some-challenge"));
        assert!(!verify_pkce("some-verifier", ""));
    }
}

