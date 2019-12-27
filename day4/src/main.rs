fn has_same_adjacent_digits(pass: &[i8; 6]) -> bool {
    pass[0] == pass[1] || pass[1] == pass[2] || pass[2] == pass[3]
        || pass[3] == pass[4] || pass[4] == pass[5]
}

#[test]
fn has_same_adjacent_digits_test() {
    assert!(has_same_adjacent_digits(&[1, 1, 2, 3, 4, 5]));
    assert!(has_same_adjacent_digits(&[1, 2, 2, 3, 4, 5]));
    assert!(has_same_adjacent_digits(&[1, 2, 3, 3, 4, 5]));
    assert!(has_same_adjacent_digits(&[1, 2, 3, 4, 4, 5]));
    assert!(has_same_adjacent_digits(&[1, 2, 3, 4, 5, 5]));
    assert!(!has_same_adjacent_digits(&[1, 2, 3, 4, 5, 6]));
}

fn never_decreases(pass: &[i8; 6]) -> bool {
    pass[0] <= pass[1] && pass[1] <= pass[2] && pass[2] <= pass[3]
        && pass[3] <= pass[4] && pass[4] <= pass[5]
}

#[test]
fn never_decreases_test() {
    assert!(never_decreases(&[1, 1, 1, 1, 1, 1]));
    assert!(never_decreases(&[1, 2, 3, 4, 5, 6]));
    assert!(!never_decreases(&[1, 2, 1, 2, 1, 2]));
}

#[test]
fn both_conditions_test() {
    let p = [1, 1, 1, 1, 1, 1];
    assert!(has_same_adjacent_digits(&p) && never_decreases(&p));
    let p = [2, 2, 3, 4, 5, 0];
    assert!(!(has_same_adjacent_digits(&p) && never_decreases(&p)));
    let p = [1, 2, 3, 7, 8, 9];
    assert!(!(has_same_adjacent_digits(&p) && never_decreases(&p)));
}

fn main() {
    let mut eligible = 0;
    for i in 146810i32..=612564i32 {
        let pass: [i8; 6] = [
            (i / 100000 % 10) as i8,
            (i / 10000 % 10) as i8,
            (i / 1000 % 10) as i8,
            (i / 100 % 10) as i8,
            (i / 10 % 10) as i8,
            (i % 10) as i8
        ];
        if has_same_adjacent_digits(&pass) && never_decreases(&pass) {
            eligible += 1;
        }
    }
    println!("Eligible passwords: {}", eligible);
}
