pub fn a() -> String {
    crack_door_password_a("wtnhxymk")
}

pub fn b() -> String {
    crack_door_password_b("wtnhxymk")
}

fn crack_door_password_a(door_id: &str) -> String {
    let mut ind = 0;
    let mut password = String::new();

    for _ in 0..8 {
        let (next_char, next_ind) = next_hash_char(door_id, ind);
        password.push(next_char);
        ind = next_ind + 1;
    }

    password
}

fn crack_door_password_b(door_id: &str) -> String {
    let mut ind = 0;

    let mut seen_count = 0;
    let mut seen: [bool; 8] = Default::default();
    let mut password: [char; 8] = Default::default();

    while seen_count < 8 {
        let (pos, c, next_ind) = next_hash_two_char(door_id, ind);
        let pos = pos as usize;
        if pos < 8 && !seen[pos] {
            seen[pos] = true;
            seen_count += 1;
            password[pos] = c;
        }
        ind = next_ind + 1;
    }

    let mut out_str = String::new();

    for c in password {
        out_str.push(c);
    }

    out_str
}

fn next_hash_char(door_id: &str, start_index: usize) -> (char, usize) {
    let mut index = start_index;

    while index < usize::MAX {
        let to_hash = format!("{}{}", door_id, index);
        let hash = md5::compute(to_hash.as_bytes()).0;
        if is_valid_hash(&hash) {
            let c = byte_to_char(hash[2]);
            return (c, index);
        }
        index += 1;
    }

    panic!("Should have found a solution")
}

fn next_hash_two_char(door_id: &str, start_index: usize) -> (u8, char, usize) {
    let mut index = start_index;

    while index < usize::MAX {
        let to_hash = format!("{}{}", door_id, index);
        let hash = md5::compute(to_hash.as_bytes()).0;
        if is_valid_hash(&hash) {
            let a = hash[2];
            let b = byte_to_char(hash[3] >> 4);
            return (a, b, index);
        }
        index += 1;
    }

    panic!("Should have found a solution")
}

fn is_valid_hash(bytes: &[u8]) -> bool {
    bytes[0] == 0 && bytes[1] == 0 && bytes[2] < 16
}

// b should be in 0 ..= 15
// gives a hex char (0, 1, ..., 9, a, ..., f)
fn byte_to_char(b: u8) -> char {
    if b < 10 {
        (b + ('0' as u8)) as char
    } else if b < 16 {
        (b - 10 + ('a' as u8)) as char
    } else {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use crate::day_05::{crack_door_password_a, next_hash_char};

    #[test]
    fn sample_char_test() {
        let door = "abc";
        let (c1, ind1) = next_hash_char(door, 0);
        assert_eq!(c1, '1');
        assert_eq!(ind1, 3231929);

        let (c2, ind2) = next_hash_char(door, ind1 + 1);
        assert_eq!(c2, '8');
        assert_eq!(ind2, 5017308);

        let (c3, ind3) = next_hash_char(door, ind2 + 1);
        assert_eq!(c3, 'f');
        assert_eq!(ind3, 5278568);
    }

    #[test]
    fn sample_test_a() {
        let actual = crack_door_password_a("abc");
        assert_eq!(actual, "18f47a30");
    }
}
