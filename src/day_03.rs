pub fn a() -> String {
    let contents = std::fs::read_to_string("src/input_3.txt").unwrap();
    format!("{}", a_with_inputs(&contents))
}

pub fn b() -> String {
    let contents = std::fs::read_to_string("src/input_3.txt").unwrap();
    format!("{}", b_with_inputs(&contents))
}

fn a_with_inputs(input: &str) -> usize {
    input.lines().filter(|line| is_possible(line)).count()
}

fn is_possible(line: &str) -> bool {
    let mut sides: Vec<i32> = line
        .split_whitespace()
        .map(|tok| tok.parse::<i32>().unwrap())
        .collect();
    assert_eq!(sides.len(), 3);
    sides.sort();
    sides[0] + sides[1] > sides[2]
}

fn b_with_inputs(input: &str) -> usize {
    let twisted = twist_input_b(input);
    twisted
        .chunks(3)
        .filter(|slice| {
            assert_eq!(slice.len(), 3);
            let mut arr: Vec<i32> = slice.iter().copied().collect();
            arr.sort();
            arr[0] + arr[1] > arr[2]
        })
        .count()
}

fn twist_input_b(input: &str) -> Vec<i32> {
    assert_eq!(input.lines().count() % 3, 0);

    let mut out = Vec::new();

    // i don't love that we iterate through -- and parse -- everything several times
    // but it runs in 0.00049 seconds so whatever
    for col in 0..3 {
        for line in input.lines() {
            let tok = line
                .split_whitespace()
                .nth(col)
                .expect("Should have enough columns")
                .parse::<i32>()
                .unwrap();
            out.push(tok);
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use crate::day_03::a_with_inputs;

    #[test]
    fn sample_1a() {
        assert_eq!(a_with_inputs("5 10 25"), 0);
        assert_eq!(a_with_inputs("3 4 5"), 1);
    }
}
