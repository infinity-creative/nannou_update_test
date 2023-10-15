
pub fn hex_to_u32(color: &String) -> Option<u32> {
    if color.len() == 7 && color.starts_with('#') {
        // Remove the # prefix
        let hex_str = &color[1..];

        // Parse the hexadecimal string to u32
        u32::from_str_radix(hex_str, 16).ok()
    } else {
        None
    }
}