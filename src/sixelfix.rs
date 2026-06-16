pub fn remove_newline(sixel: &mut String) {
    // For some reason, icy-sixel is adding newline to the end of its sixels
    if sixel.ends_with("$-\x1B\\") {
        sixel.truncate(sixel.len() - 4);
        sixel.push_str("\x1B\\");
    }
}