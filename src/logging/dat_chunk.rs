use chrono::Local;

/// To dump data in xxd-like hex format and return as a string
pub fn dump_data(data: &[u8], type_str: &str) -> String {
    const BYTES_PER_LINE: usize = 16;
    let mut result = String::new();

    let timestamp = Local::now().format("%Y_%m_%d_%H_%M_%S").to_string();
    let begin = format!("-------[ {} [{}] ]----------\n", type_str, timestamp);

    result.push_str(begin.as_str());
    for (i, chunk) in data.chunks(BYTES_PER_LINE).enumerate() {
        // Write the offset/address in hex format
        result.push_str(&format!("{:08X}: ", i * BYTES_PER_LINE));

        // Write the hex values of the bytes in the chunk
        for byte in chunk {
            result.push_str(&format!("{:02X} ", byte));
        }

        // Add spacing if the last line is not full
        for _ in 0..(BYTES_PER_LINE - chunk.len()) {
            result.push_str("   ");
        }

        // Write the ASCII representation
        result.push('|');
        for byte in chunk {
            if byte.is_ascii_graphic() || byte.is_ascii_whitespace() {
                result.push(*byte as char);
            } else {
                result.push('.');
            }
        }
        result.push('|');
        result.push('\n');
    }

    result
}
