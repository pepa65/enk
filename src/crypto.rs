use aes_gcm::{
	Aes256Gcm, Nonce,
	aead::{Aead, KeyInit},
};
use anyhow::{Context, Result, anyhow};
use rand::{Rng, rng};
use sha2::{Digest, Sha256};

const NONCE_SIZE: usize = 12;

pub fn encrypt(plaintext: &[u8], key: Key) -> Result<Vec<u8>> {
	let nonce_data: [u8; NONCE_SIZE] = rng().random();
	let ciphertext = key.cipher.encrypt(Nonce::from_slice(&nonce_data), plaintext).map_err(|e| anyhow!("encrypt: {e}"))?;
	let mut output_data = Vec::from(nonce_data);
	output_data.extend(ciphertext);
	Ok(output_data)
}

pub fn decrypt(ciphertext: &[u8], key: Key) -> Result<Vec<u8>> {
	if ciphertext.len() < NONCE_SIZE {
		anyhow::bail!("encrypted data too short")
	}
	let (nonce_data, ciphertext) = ciphertext.split_at(NONCE_SIZE);
	let plaintext = key.cipher.decrypt(Nonce::from_slice(nonce_data), ciphertext).map_err(|e| anyhow!("decrypt: {e}"))?;
	Ok(plaintext)
}

pub struct Key {
	cipher: Aes256Gcm,
}

impl Key {
	pub fn new(key: impl AsRef<[u8]>) -> Result<Self> {
		let key: [u8; 32] = Sha256::digest(key).into();
		let cipher = Aes256Gcm::new_from_slice(&key).context("key from slice")?;
		Ok(Self { cipher })
	}
}
