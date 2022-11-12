pub fn day_2a() -> String {
    let contents = std::fs::read_to_string("./src/input_2.txt").unwrap();
    let actual = day_2a_with_input(&contents);

    format!("{}", actual)
}

fn day_2a_with_input(input: &str) -> String {
    let mut out: String = "".to_string();

    let mut x = 1;
    let mut y = 1;

    for line in input.trim().lines() {
        for step in line.trim().chars() {
            match step {
                'U' => {
                    y = (y - 1).max(0);
                }
                'D' => {
                    y = (y + 1).min(2);
                }
                'L' => {
                    x = (x - 1).max(0);
                }
                'R' => {
                    x = (x + 1).min(2);
                }
                _ => panic!("Trash"),
            }
        }

        let key = (y * 3) + x + 1;
        out.push_str(key.to_string().as_str());
    }

    out
}

pub fn day_2b() -> String {
    let contents = std::fs::read_to_string("./src/input_2.txt").unwrap();
    let actual = day_2b_with_input(&contents);

    format!("{}", actual)
}

fn day_2b_with_input(input: &str) -> String {
    let mut out: String = "".to_string();

    let mut x = 0;
    let mut y = 2;

    fn is_valid(x: i32, y: i32) -> bool {
        if y == 2 {
            x >= 0 && x <= 4
        } else if y == 1 || y == 3 {
            x >= 1 && x <= 3
        } else if y == 0 || y == 4 {
            x == 2
        } else {
            false
        }
    }

    fn location_char(x: i32, y: i32) -> char {
        match (x, y) {
            (2, 0) => '1',
            (1, 1) => '2',
            (2, 1) => '3',
            (3, 1) => '4',
            (0, 2) => '5',
            (1, 2) => '6',
            (2, 2) => '7',
            (3, 2) => '8',
            (4, 2) => '9',
            (1, 3) => 'A',
            (2, 3) => 'B',
            (3, 3) => 'C',
            (2, 4) => 'D',
            _ => panic!("Unknown location ({}, {})", x, y),
        }
    }

    for line in input.trim().lines() {
        for step in line.trim().chars() {
            let (next_x, next_y) = match step {
                'U' => (x, y - 1),
                'D' => (x, y + 1),
                'L' => (x - 1, y),
                'R' => (x + 1, y),
                _ => panic!("Trash"),
            };

            if is_valid(next_x, next_y) {
                x = next_x;
                y = next_y;
            }
        }

        out.push(location_char(x, y));
    }

    out
}

#[cfg(test)]
mod tests {
    fn test_2a(input: &str, expected: &str) {
        let actual = super::day_2a_with_input(input);

        assert_eq!(actual.as_str(), expected);
    }

    #[test]
    fn sample_2a() {
        test_2a(
            "ULL
RRDDD
LURDL
UUUUD",
            "1985",
        );
    }

    fn test_2b(input: &str, expected: &str) {
        let actual = super::day_2b_with_input(input);

        assert_eq!(actual.as_str(), expected);
    }

    #[test]
    fn sample_2b() {
        test_2b(
            "ULL
RRDDD
LURDL
UUUUD",
            "5DB3",
        );
    }
}
