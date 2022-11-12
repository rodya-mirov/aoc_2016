use std::collections::HashMap;

pub fn a() -> String {
    let contents = std::fs::read_to_string("src/input_4.txt").unwrap();
    format!("{}", a_with_input(&contents))
}

fn a_with_input(contents: &str) -> i32 {
    contents
        .lines()
        .filter(|line| is_valid(line))
        .map(|line| sector(line))
        .sum()
}

fn is_valid(line: &str) -> bool {
    checksum(line) == actual_hash(line)
}

fn checksum(line: &str) -> String {
    let mut out = String::new();

    let mut seen_open = false;
    for c in line.chars() {
        if c == '[' {
            seen_open = true;
        } else if c == ']' {
            seen_open = false;
        } else if seen_open {
            out.push(c);
        }
    }

    out
}

fn actual_hash(line: &str) -> String {
    let front = line.split("[").next().unwrap();
    let mut mult_map: HashMap<char, usize> = HashMap::new();
    let mut seen: Vec<char> = Vec::new();

    for room_char in front.chars().filter(|c| c.is_ascii_lowercase()) {
        if !mult_map.contains_key(&room_char) {
            seen.push(room_char);
        }

        *mult_map.entry(room_char).or_insert(0) += 1;
    }

    seen.sort_by(|a, b| {
        mult_map
            .get(&b)
            .unwrap()
            .cmp(mult_map.get(&a).unwrap())
            .then(a.cmp(b))
    });

    seen.into_iter().take(5).collect()
}

fn sector(line: &str) -> i32 {
    // i feel like there's a better way to do this
    let token = line
        .trim()
        .split("-")
        .last()
        .expect("String should be nonempty");
    token
        .split("[")
        .next()
        .expect("Should should be nonempty")
        .parse::<i32>()
        .unwrap()
}

pub fn b() -> String {
    let contents = std::fs::read_to_string("src/input_4.txt").unwrap();

    let sector = contents
        .lines()
        // bit janky but grep returned one result and the problem is intentionally ambiguous
        // as to what the right answer looks like, so idk how to make it less manual
        .filter(|line| decrypt(line).contains("pole"))
        .map(|line| sector(line))
        .next()
        .expect("Should find something");

    sector.to_string()
}

fn decrypt(line: &str) -> String {
    let rotations = sector(line);

    let mut chars_iter = line
        .split('[')
        .next()
        .unwrap()
        .chars()
        .filter(|&c| c == '-' || c.is_ascii_lowercase())
        .map(|c| decrypt_char(c, rotations));

    let mut out = String::new();

    while let Some(next_char) = chars_iter.next() {
        out.push(next_char);
    }

    out.pop();

    out
}

fn decrypt_char(c: char, rotations: i32) -> char {
    if c == '-' {
        ' '
    } else if c.is_ascii_lowercase() {
        let mut offset: i32 = (c as i32) - ('a' as i32);
        offset += rotations % 26;
        if offset >= 26 {
            offset -= 26;
        }
        (offset + ('a' as i32)) as u8 as char
    } else {
        panic!("Bad input char {}", c)
    }
}

#[cfg(test)]
mod tests {
    use super::{actual_hash, checksum, decrypt, is_valid, sector};

    #[test]
    fn sample_a_valid() {
        assert!(is_valid("aaaaa-bbb-z-y-x-123[abxyz]"));
        assert!(is_valid("a-b-c-d-e-f-g-h-987[abcde]"));
        assert!(is_valid("not-a-real-room-404[oarel]"));
        assert!(!is_valid("totally-real-room-200[decoy]"));
    }

    #[test]
    fn sample_a_sector() {
        assert_eq!(sector("aaaaa-bbb-z-y-x-123[abxyz]"), 123);
        assert_eq!(sector("a-b-c-d-e-f-g-h-987[abcde]"), 987);
        assert_eq!(sector("not-a-real-room-404[oarel]"), 404);
        assert_eq!(sector("totally-real-room-200[decoy]"), 200);
    }

    #[test]
    fn sample_a_actual_hash() {
        assert_eq!(
            actual_hash("aaaaa-bbb-z-y-x-123[abxyz]"),
            "abxyz".to_string()
        );
        assert_eq!(
            actual_hash("a-b-c-d-e-f-g-h-987[abcde]"),
            "abcde".to_string()
        );
        assert_eq!(
            actual_hash("not-a-real-room-404[oarel]"),
            "oarel".to_string()
        );
        assert_eq!(
            actual_hash("totally-real-room-200[decoy]"),
            "loart".to_string()
        );
    }

    #[test]
    fn sample_a_checksum() {
        assert_eq!(checksum("aaaaa-bbb-z-y-x-123[abxyz]"), "abxyz".to_string());
        assert_eq!(checksum("a-b-c-d-e-f-g-h-987[abcde]"), "abcde".to_string());
        assert_eq!(checksum("not-a-real-room-404[oarel]"), "oarel".to_string());
        assert_eq!(
            checksum("totally-real-room-200[decoy]"),
            "decoy".to_string()
        );
    }

    #[test]
    fn sample_b() {
        assert_eq!(
            decrypt("qzmt-zixmtkozy-ivhz-343"),
            "very encrypted name".to_string()
        );
    }
}
