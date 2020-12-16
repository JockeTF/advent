use super::*;

#[test]
fn test_first() {
    let spec: Seat = "BFFFBBFRRR".parse().unwrap();

    assert_eq!(spec.row_number(), 70);
    assert_eq!(spec.col_number(), 7);
    assert_eq!(spec.seat_number(), 567);
}

#[test]
fn test_second() {
    let spec: Seat = "FFFBBBFRRR".parse().unwrap();

    assert_eq!(spec.row_number(), 14);
    assert_eq!(spec.col_number(), 7);
    assert_eq!(spec.seat_number(), 119);
}

#[test]
fn test_third() {
    let spec: Seat = "BBFFBBFRLL".parse().unwrap();

    assert_eq!(spec.row_number(), 102);
    assert_eq!(spec.col_number(), 4);
    assert_eq!(spec.seat_number(), 820);
}

#[test]
fn test_extra() {
    let spec: Seat = "BBFFBBBRLR".parse().unwrap();

    assert_eq!(spec.row_number(), 103);
    assert_eq!(spec.col_number(), 5);
    assert_eq!(spec.seat_number(), 829);
}
