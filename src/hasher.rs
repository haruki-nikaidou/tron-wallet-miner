pub fn next_hash(before: [u8;16]) -> [u8;16] {
    derive_result(blake3::hash(&before).as_bytes().to_owned())
}

fn derive_result(hash: [u8;32]) -> [u8;16] {
    let mut result = [0u8; 16];
    result[..16].copy_from_slice(&hash[..16]);
    result
}

pub fn repeat(password: String, times: usize) -> [u8;16] {
    let repeated = password.repeat(times);
    let hash = blake3::hash(repeated.as_bytes());
    derive_result(hash.as_bytes().to_owned())
}
