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

pub fn have_bad_digraphs(a: Vec<u8>) -> bool {
    let digraphs: [&str; 22] = [
        "cj", "fq", "gx", "hx", "jf", "jq", "jx", "jz", "qb", "qc", "qj", "qk", "qx", "qz", "sx",
        "vf", "vj", "vq", "vx", "wx", "xj", "zx",
    ];

    let s: String = a.into_iter().map(|c| c as char).collect();

    for i in &digraphs {
        if s.contains(i) {
            return true;
        }
    }

    false
}
