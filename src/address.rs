use bip39::Mnemonic;
use ripemd::Ripemd160;
use secp256k1::{PublicKey, Secp256k1};
use sha2::{Digest, Sha256};
use tiny_hderive::bip32::ExtendedPrivKey;

pub fn generate_tron_address(mnemonic: &Mnemonic) -> [u8; 25] {
    let seed = mnemonic.to_seed("");

    // Derive private key using BIP44
    let path = "m/44'/195'/0'/0/0";
    let ext_priv_key = ExtendedPrivKey::derive(&seed, path).unwrap();
    let private_key = ext_priv_key.secret();

    // Generate public key
    let secp = Secp256k1::new();
    let private_key = secp256k1::SecretKey::from_slice(&private_key).unwrap();
    let public_key = PublicKey::from_secret_key(&secp, &private_key);

    // Generate TRON address
    let public_key_bytes = public_key.serialize_uncompressed();
    let hash = Sha256::digest(&public_key_bytes[1..]);
    let hash = Ripemd160::digest(&hash);

    // Add TRON prefix (0x41)
    let mut address = vec![0x41];
    address.extend_from_slice(&hash);

    // Add checksum
    let checksum = &Sha256::digest(&Sha256::digest(&address))[..4];

    // Combine address and checksum into final 25-byte array
    let mut result = [0u8; 25];
    result[..21].copy_from_slice(&address);
    result[21..].copy_from_slice(checksum);

    result
}

pub fn end_with(
    address: [u8;25],
    required_end: &str
) -> bool {
    let base58 = bs58::encode(address).into_string();
    base58.ends_with(required_end)
}