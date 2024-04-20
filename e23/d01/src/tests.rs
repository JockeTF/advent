use crate::find_value;
use crate::scan_lines;
use crate::sum_values;

const FIRST_SAMPLE: &str = "
    1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet
";

const SECOND_SAMPLE: &str = "
    two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen
";

fn collect_values(input: &str) -> Option<Vec<i32>> {
    scan_lines(input).map(find_value).collect()
}

#[test]
fn test_first_values() {
    let actual = collect_values(FIRST_SAMPLE);
    let expect = Some(vec![12, 38, 15, 77]);

    assert_eq!(actual, expect);
}

#[test]
fn test_first_result() {
    let actual = sum_values(FIRST_SAMPLE);
    let expect = Some(142);

    assert_eq!(actual, expect);
}

#[test]
fn test_second_values() {
    let actual = collect_values(SECOND_SAMPLE);
    let expect = Some(vec![29, 83, 13, 24, 42, 14, 76]);

    assert_eq!(actual, expect);
}

#[test]
fn test_second_result() {
    let actual = sum_values(SECOND_SAMPLE);
    let expect = Some(281);

    assert_eq!(actual, expect);
}
