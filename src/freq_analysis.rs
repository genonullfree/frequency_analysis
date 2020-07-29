
pub fn printable_ascii(a: Vec<u8>) -> bool {
    let s: String = a.into_iter().map(|c| c as char).collect();

    s.is_ascii()
}

pub fn have_vowels(a: Vec<u8>) -> bool {
    let vowels: [char; 6] = ['a', 'e', 'i', 'o', 'u', 'y'];

    for i in &vowels {
        for b in &a {
            if *i == *b as char {
                return true;
            }
        }
    }

    false
}
