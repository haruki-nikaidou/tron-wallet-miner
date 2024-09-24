use bip39::Mnemonic;
use hmac::{Hmac, Mac};
use ripemd::Ripemd160;
use secp256k1::{PublicKey, Secp256k1};
use sha2::{Digest, Sha256, Sha512};

fn derive_private_key(seed: &[u8], path: &str) -> ([u8; 32], [u8; 32]) {
    let mut key = [0u8; 32];
    let mut chain_code = [0u8; 32];

    let mut hmac: Hmac<Sha512> = Hmac::new_from_slice(b"Bitcoin seed").unwrap();
    hmac.update(seed);
    let result = hmac.finalize().into_bytes();
    key.copy_from_slice(&result[..32]);
    chain_code.copy_from_slice(&result[32..]);

    for part in path.split('/').skip(1) {
        let hardened = part.ends_with('\'');
        let index = if hardened {
            0x80000000 + part[..part.len() - 1].parse::<u32>().unwrap()
        } else {
            part.parse::<u32>().unwrap()
        };

        let mut data = Vec::with_capacity(37);
        if hardened {
            data.push(0);
            data.extend_from_slice(&key);
        } else {
            let secp = Secp256k1::new();
            let public_key = PublicKey::from_secret_key(
                &secp,
                &secp256k1::SecretKey::from_slice(&key).unwrap());
            data.extend_from_slice(&public_key.serialize());
        }
        data.extend_from_slice(&index.to_be_bytes());

        let mut hmac: Hmac<Sha512> = Hmac::new_from_slice(&chain_code).unwrap();
        hmac.update(&data);
        let result = hmac.finalize().into_bytes();

        for (i, byte) in key.iter_mut().enumerate() {
            *byte ^= result[i];
        }
        chain_code.copy_from_slice(&result[32..]);
    }

    (key, chain_code)
}

pub fn generate_tron_address(mnemonic: &Mnemonic) -> [u8; 25] {
    let seed = mnemonic.to_seed("");

    // Derive private key using BIP44
    let path = "m/44'/195'/0'/0/0";
    let (private_key, _) = derive_private_key(&seed, path);
    let private_key = secp256k1::SecretKey::from_slice(&private_key).unwrap();

    // Generate public key
    let secp = Secp256k1::new();
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
    required_end: &[u8]
) -> bool {
    let mut result = true;
    let req_iter = required_end.iter().rev();
    let addr_iter = address.iter().rev();
    for (req, addr) in req_iter.zip(addr_iter) {
        if req != addr {
            result = false;
            break;
        }
    }
    result
}