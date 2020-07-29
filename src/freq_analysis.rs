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

pub fn have_freq_chars(a: Vec<u8>, pct: f64) -> bool {
    let frequent: [char; 6] = ['e', 't', 'a', 'o', 'i', 'n'];
    let mut count = 0;

    for i in &frequent {
        for b in &a {
            if *i == *b as char {
                count += 1;
            }
        }
    }

    let p_calc: f64 = ((count * 100) / a.len()) as f64;

    p_calc > pct
}

pub fn have_freq_punctuation(a: Vec<u8>, pct: f64) -> bool {
    let mut count = 0;

    for b in &a {
        if b.is_ascii_punctuation() {
            count += 1;
        }
    }

    let p_calc: f64 = ((count * 100) / a.len()) as f64;

    p_calc < pct
}
