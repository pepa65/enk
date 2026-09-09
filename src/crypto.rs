use aes_gcm::{
	Aes256Gcm, Nonce,
	aead::{Aead, KeyInit, Payload},
};
use anyhow::{Context, Result, anyhow};
use argon2::{Algorithm, Argon2, Params, Version};
use rand::{RngExt, rng};
use zeroize::Zeroizing;

// File format enk1:
// - 4 bytes magic ("enk1")
// - 16 bytes Argon2id salt
// - 12 bytes AES-256-GCM nonce
// - N bytes AES-256-GCM ciphertext
// - 16 bytes authentication tag
//
// Key derivation: AES-256 key = Argon2id(secret, salt)
//
// Fixed parameters for enk1:
// - Algorithm:   Argon2id
// - Version:     1.3 (0x13)
// - Memory:      256 MiB (262144 KiB)
// - Time cost:   13
// - Parallelism: 1
// - Output:      32 bytes
//
// The magic & salt are authenticated as AES-GCM AAD, the nonce by AES-GCM itself

const MAGIC: &[u8; 4] = b"enk1";
const SALT_SIZE: usize = 16; // 128-bit Argon2 salt
const NONCE_SIZE: usize = 12; // 96-bit GCM nonce
const KEY_SIZE: usize = 32; // 256-bit AES key
const TAG_SIZE: usize = 16; // 128-bit GCM authentication tag
const HEADER_SIZE: usize = MAGIC.len() + SALT_SIZE + NONCE_SIZE;
const ARGON2_M_COST: u32 = 256 * 1024; // Memory cost
const ARGON2_T_COST: u32 = 13; // Time cost
const ARGON2_P_COST: u32 = 1; // Parallelism

pub fn encrypt(plaintext: &[u8], secret: &[u8]) -> Result<Vec<u8>> {
	if secret.is_empty() {
		anyhow::bail!("secret is empty");
	}

	let salt: [u8; SALT_SIZE] = rng().random();
	let nonce_data: [u8; NONCE_SIZE] = rng().random();
	let key = Key::from_secret(secret, &salt)?;
	let nonce = Nonce::try_from(nonce_data.as_slice()).expect("nonce size is fixed");
	let aad = make_aad(&salt);
	let ciphertext = key.cipher.encrypt(&nonce, Payload { msg: plaintext, aad: &aad }).map_err(|e| anyhow!("encryption failed: {e}"))?;
	let mut output = Vec::with_capacity(HEADER_SIZE + ciphertext.len());
	output.extend_from_slice(MAGIC);
	output.extend_from_slice(&salt);
	output.extend_from_slice(&nonce_data);
	output.extend_from_slice(&ciphertext);
	Ok(output)
}

pub fn decrypt(ciphertext: &[u8], secret: &[u8]) -> Result<Vec<u8>> {
	if secret.is_empty() {
		anyhow::bail!("secret is empty");
	}

	if ciphertext.len() < HEADER_SIZE + TAG_SIZE {
		anyhow::bail!("encrypted data too short");
	}

	let (magic, rest) = ciphertext.split_at(MAGIC.len());
	if magic != MAGIC {
		anyhow::bail!("unsupported encrypted data format");
	}

	let (salt_data, rest) = rest.split_at(SALT_SIZE);
	let (nonce_data, encrypted_data) = rest.split_at(NONCE_SIZE);
	let salt: &[u8; SALT_SIZE] = salt_data.try_into().expect("salt size is fixed");
	let nonce = Nonce::try_from(nonce_data).expect("nonce size is fixed");
	let key = Key::from_secret(secret, salt)?;
	let aad = make_aad(salt);
	key.cipher
		.decrypt(&nonce, Payload { msg: encrypted_data, aad: &aad })
		.map_err(|_| anyhow!("decryption failed: wrong password/key or corrupted data"))
}

fn make_aad(salt: &[u8; SALT_SIZE]) -> [u8; MAGIC.len() + SALT_SIZE] {
	let mut aad = [0u8; MAGIC.len() + SALT_SIZE];
	aad[..MAGIC.len()].copy_from_slice(MAGIC);
	aad[MAGIC.len()..].copy_from_slice(salt);
	aad
}

struct Key {
	cipher: Aes256Gcm,
}

impl Key {
	fn from_secret(secret: &[u8], salt: &[u8; SALT_SIZE]) -> Result<Self> {
		let params = Params::new(ARGON2_M_COST, ARGON2_T_COST, ARGON2_P_COST, Some(KEY_SIZE)).context("invalid Argon2 parameters")?;
		let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
		let mut key = Zeroizing::new([0u8; KEY_SIZE]);
		argon2.hash_password_into(secret, salt, key.as_mut()).context("key derivation failed")?;
		let cipher = Aes256Gcm::new_from_slice(key.as_ref()).context("invalid AES-256 key")?;
		Ok(Self { cipher })
	}
}
