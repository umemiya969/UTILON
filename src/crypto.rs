use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;
use hex;

pub struct Keypair {
    pub signing: SigningKey,
    pub verifying: VerifyingKey,
}

pub fn generate_keypair() -> Keypair {
    let signing = SigningKey::generate(&mut OsRng);
    let verifying = signing.verifying_key();
    Keypair { signing, verifying }
}

pub fn sign(sk: &SigningKey, msg: &[u8]) -> String {
    let sig: Signature = sk.sign(msg);
    hex::encode(sig.to_bytes())
}

pub fn verify(pk_hex: &str, msg: &[u8], sig_hex: &str) -> bool {
    let pk_bytes = hex::decode(pk_hex).ok()?;
    let sig_bytes = hex::decode(sig_hex).ok()?;

    let pk = VerifyingKey::from_bytes(&pk_bytes.try_into().ok()?).ok()?;
    let sig = Signature::from_bytes(&sig_bytes.try_into().ok()?);

    pk.verify(msg, &sig).is_ok()
}
