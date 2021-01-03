use std::io::BufRead;
fn main() {
    std::io::BufReader::new(std::fs::File::open("/usr/share/dict/words").unwrap())
        .lines()
        .for_each(|word| check(word.unwrap()));
}

fn check(word: String) {
    let lower = word.to_lowercase();
    let mut chars = lower.chars();
    let mut first;
    match chars.next() {
        Some(c) => first = c,
        None => return,
    }
    while let Some(second) = chars.next() {
        if !valid(&first, &second) {
            return;
        }
        first = second;
    }
    println!("{}", word)
}

fn valid(a: &char, b: &char) -> bool {
    match a {
        'b' => match b {
            'r' | 'l' | 'o' | 't' | 'y' => true,
            _ => false,
        },
        'c' => match b {
            'r' | 'o' | 'e' | 'g' | 'h' => true,
            _ => false,
        },
        'e' => match b {
            'r' | 'o' | 'l' | 'g' | 'c' => true,
            _ => false,
        },
        'g' => match b {
            'e' | 'o' | 'h' | 'w' | 'c' => true,
            _ => false,
        },
        'h' => match b {
            's' | 'o' | 'g' | 'w' | 'c' => true,
            _ => false,
        },
        'l' => match b {
            'r' | 'o' | 'b' | 't' | 'e' => true,
            _ => false,
        },
        'n' => match b {
            'w' | 'o' | 's' | 't' | 'y' => true,
            _ => false,
        },
        'o' => match b {
            'b' | 'c' | 'e' | 'g' | 'h' | 'l' | 'n' | 'r' | 's' | 't' | 'w' | 'y' => true,
            _ => false,
        },
        'r' => match b {
            'c' | 'o' | 'l' | 'b' | 'e' => true,
            _ => false,
        },
        's' => match b {
            'w' | 'o' | 'h' | 'n' | 'y' => true,
            _ => false,
        },
        't' => match b {
            'b' | 'o' | 'l' | 'n' | 'y' => true,
            _ => false,
        },
        'w' => match b {
            'n' | 'o' | 's' | 'g' | 'h' => true,
            _ => false,
        },
        'y' => match b {
            'b' | 'o' | 's' | 't' | 'n' => true,
            _ => false,
        },
        _ => false
    }
}
