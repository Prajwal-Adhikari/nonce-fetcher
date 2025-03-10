use base64::{engine::general_purpose, Engine as _};

#[derive(Debug)]
pub struct DepositSuccessful {
    nonce: u64,
    to_twine_address: String,
    l1_token: String,
    l2_token: String,
    chain_id: u64,
    amount: String,
    slot_number: u64,
}

fn main() {
    // The program data from the log
    let encoded_data = "cIBmZdJCH7MZAAAAAAAAACoAAAAweDBEMzI1MGMzRDVGQWNiNzRBYzE1ODM0MDk2Mzk3YTNFZjc5MEFBQ0QgAAAAMTExMTExMTExMTExMTExMTExMTExMTExMTExMTExMTEqAAAAMHgwRDMyNTBjM0Q1RkFjYjc0QWMxNTgzNDA5NjM5N2EzRWY3OTBBQUNEhAMAAAAAAAALAAAANDcwMDAwMDAwMDCfnQAAAAAAAA==";

    // Step 1: Decode the base64 encoded data
    let decoded_data = match general_purpose::STANDARD.decode(encoded_data) {
        Ok(data) => data,
        Err(e) => {
            println!("Failed to decode base64: {}", e);
            return;
        }
    };

    // Skip the first 8 bytes (discriminator)
    let mut pos = 8;

    // Parse function for u64
    let parse_u64 = |data: &[u8], start: usize| -> (u64, usize) {
        let mut value_bytes = [0u8; 8];
        value_bytes.copy_from_slice(&data[start..start + 8]);
        (u64::from_le_bytes(value_bytes), start + 8)
    };

    // Parse function for strings (Borsh format - u32 length prefix followed by bytes)
    let parse_string = |data: &[u8], start: usize| -> (String, usize) {
        let len_bytes = [
            data[start],
            data[start + 1],
            data[start + 2],
            data[start + 3],
        ];
        let len = u32::from_le_bytes(len_bytes) as usize;
        let string_bytes = &data[start + 4..start + 4 + len];
        let string = String::from_utf8_lossy(string_bytes).to_string();
        (string, start + 4 + len)
    };

    // Read nonce field
    let (nonce, new_pos) = parse_u64(&decoded_data, pos);
    println!("Nonce: {}", nonce);
    pos = new_pos;

    // Read to_twine_address
    let (to_twine_address, new_pos) = parse_string(&decoded_data, pos);
    println!("To Twine Address: {}", to_twine_address);
    pos = new_pos;

    // Read l1_token
    let (l1_token, new_pos) = parse_string(&decoded_data, pos);
    println!("L1 Token: {}", l1_token);
    pos = new_pos;

    // Read l2_token
    let (l2_token, new_pos) = parse_string(&decoded_data, pos);
    println!("L2 Token: {}", l2_token);
    pos = new_pos;

    // Read chain_id
    let (chain_id, new_pos) = parse_u64(&decoded_data, pos);
    println!("Chain ID: {}", chain_id);
    pos = new_pos;

    // Read amount
    let (amount, new_pos) = parse_string(&decoded_data, pos);
    println!("Amount: {}", amount);
    pos = new_pos;

    // Read slot_number
    let (slot_number, _) = parse_u64(&decoded_data, pos);
    println!("Slot Number: {}", slot_number);

    // Create and return the struct
    let deposit = DepositSuccessful {
        nonce,
        to_twine_address,
        l1_token,
        l2_token,
        chain_id,
        amount,
        slot_number,
    };

    println!("\nFull decoded struct:\n{:#?}", deposit);
}
