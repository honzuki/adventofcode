/// Removes all whitespace that comes at the beginning
/// and returns the amount of whitespace along side a
/// new reference that doesn't contain them
pub fn split_many_whitespace_at_start(s: &str) -> (&str, usize) {
    for (idx, ch) in s.bytes().enumerate() {
        if !ch.is_ascii_whitespace() {
            return (&s[idx..s.len()], idx);
        }
    }

    return (&s[0..0], s.len());
}
