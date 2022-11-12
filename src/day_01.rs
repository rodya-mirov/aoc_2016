pub fn day_1a() -> String {
    let contents =
        std::fs::read_to_string("src/input_1.txt").expect("Should be able to read the file");

    let val = day_1a_with_input(&contents);

    format!("{}", val.0.abs() + val.1.abs())
}

fn day_1a_with_input(input: &str) -> (i32, i32) {
    // blocks east
    let mut x = 0;

    // blocks north
    let mut y = 0;

    let mut dx = 0;
    let mut dy = 1;

    for step in input.trim().split(", ") {
        let val: i32 = step
            .chars()
            .skip(1)
            .collect::<String>()
            .parse::<i32>()
            .expect("Should parse");
        match step.chars().next().unwrap() {
            'R' => {
                right(&mut dx, &mut dy);
            }
            'L' => {
                right(&mut dx, &mut dy);
                right(&mut dx, &mut dy);
                right(&mut dx, &mut dy);
            }
            other => {
                panic!("Invalid input: {}", other);
            }
        }

        x += val * dx;
        y += val * dy;
    }

    (x, y)
}

pub fn day_1b() -> String {
    let contents =
        std::fs::read_to_string("src/input_1.txt").expect("Should be able to read the file");

    let val = day_1b_with_input(&contents);

    format!("{}", val.0.abs() + val.1.abs())
}

fn day_1b_with_input(input: &str) -> (i32, i32) {
    let mut seen = std::collections::HashSet::new();

    seen.insert((0, 0));

    // blocks east
    let mut x = 0;

    // blocks north
    let mut y = 0;

    let mut dx = 0;
    let mut dy = 1;

    for step in input.trim().split(", ") {
        let val: i32 = step
            .chars()
            .skip(1)
            .collect::<String>()
            .parse::<i32>()
            .expect("Should parse");
        match step.chars().next().unwrap() {
            'R' => {
                right(&mut dx, &mut dy);
            }
            'L' => {
                right(&mut dx, &mut dy);
                right(&mut dx, &mut dy);
                right(&mut dx, &mut dy);
            }
            other => {
                panic!("Invalid input: {}", other);
            }
        }

        for _ in 0..val {
            x += dx;
            y += dy;

            if !seen.insert((x, y)) {
                return (x, y);
            }
        }
    }

    panic!("No repeats found");
}

fn right(dx: &mut i32, dy: &mut i32) {
    if *dx == 1 {
        *dx = 0;
        *dy = -1;
    } else if *dx == -1 {
        *dx = 0;
        *dy = 1;
    } else if *dy == 1 {
        *dx = 1;
        *dy = 0;
    } else {
        *dx = -1;
        *dy = 0;
    }
}

#[cfg(test)]
mod tests_1a {
    fn test_1a(input: &str, expected: (i32, i32)) {
        let actual = super::day_1a_with_input(input);
        assert_eq!(actual, expected);
    }

    fn test_1b(input: &str, expected: (i32, i32)) {
        let actual = super::day_1b_with_input(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_1a_1() {
        test_1a("R2, L3", (2, 3));
    }

    #[test]
    fn sample_1a_2() {
        test_1a("R2, R2, R2", (0, -2));
    }

    #[test]
    fn sample_1a_3() {
        test_1a("R5, L5, R5, R3", (10, 2));
    }

    #[test]
    fn sample_1b_1() {
        test_1b("R8, R4, R4, R8", (4, 0));
    }
}
