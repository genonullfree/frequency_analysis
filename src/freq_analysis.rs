
pub fn printable_ascii(a: Vec<u8>) -> bool {
    let s: String = a.into_iter().map(|c| c as char).collect();

    s.is_ascii()
}
