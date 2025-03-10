use base64::{engine::general_purpose, Engine as _};

#[derive(Debug)]
pub struct DepositMessageInfo {
    pub nonce: u64,
    pub chain_id: u64,
    pub slot_number: u64,
    pub from_l1_pubkey: String,
    pub to_twine_address: String,
    pub l1_token: String,
    pub l2_token: String,
    pub amount: String,
}

fn main() {
    // The program data from the log
    let encoded_data = "cIBmZdJCH7MPAAAAAAAAACoAAAAweDBEMzI1MGMzRDVGQWNiNzRBYzE1ODM0MDk2Mzk3YTNFZjc5MEFBQUEgAAAAMTExMTExMTExMTExMTExMTExMTExMTExMTExMTExMTEqAAAAMHgwRDMyNTBjM0Q1RkFjYjc0QWMxNTgzNDA5NjM5N2EzRWY3OTBBQUFBhAMAAAAAAAAKAAAANjAwMDAwMDAwMJt1AAAAAAAA";

    // Step 1: Decode the base64 encoded data
    let decoded_data = match general_purpose::STANDARD.decode(encoded_data) {
        Ok(data) => data,
        Err(e) => {
            println!("Failed to decode base64: {}", e);
            return;
        }
    };

    // Skip the first 8 bytes (discriminator)
    let pos = 8;

    // Parse function for u64
    let parse_u64 = |data: &[u8], start: usize| -> (u64, usize) {
        let mut value_bytes = [0u8; 8];
        value_bytes.copy_from_slice(&data[start..start + 8]);
        (u64::from_le_bytes(value_bytes), start + 8)
    };

    // Read nonce field
    let (nonce, pos) = parse_u64(&decoded_data, pos);
    println!(
        "Nonce: {} (bytes {:02x?})",
        nonce,
        &decoded_data[pos - 8..pos]
    );
}
