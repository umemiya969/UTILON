use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;
use rand::RngCore;
use hex;

pub struct Keypair {
    pub signing: SigningKey,
    pub verifying: VerifyingKey,
}

pub fn generate_keypair() -> Keypair {
    // 1. Generate 32 random bytes (private key)
    let mut secret = [0u8; 32];
    OsRng.fill_bytes(&mut secret);

    // 2. Create signing key
    let signing = SigningKey::from_bytes(&secret);

    // 3. Derive public key
    let verifying = signing.verifying_key();

    Keypair { signing, verifying }
}

pub fn verify(pk_hex: &str, msg: &[u8], sig_hex: &str) -> bool {
    let pk_bytes = match hex::decode(pk_hex) {
        Ok(b) => b,
        Err(_) => return false,
    };

    let sig_bytes = match hex::decode(sig_hex) {
        Ok(b) => b,
        Err(_) => return false,
    };

    let pk_array: [u8; 32] = match pk_bytes.try_into() {
        Ok(a) => a,
        Err(_) => return false,
    };

    let sig_array: [u8; 64] = match sig_bytes.try_into() {
        Ok(a) => a,
        Err(_) => return false,
    };

    let pk = match VerifyingKey::from_bytes(&pk_array) {
        Ok(k) => k,
        Err(_) => return false,
    };

    let sig = Signature::from_bytes(&sig_array);

    pk.verify(msg, &sig).is_ok()
}

