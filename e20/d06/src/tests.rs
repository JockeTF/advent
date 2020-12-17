use super::*;

#[test]
fn test_first() {
    let group: Group = "abc".parse().unwrap();

    assert_eq!(Group::count(group.union()), 3);
    assert_eq!(Group::count(group.intersection()), 3);
}

#[test]
fn test_second() {
    let group: Group = "a\nb\nc".parse().unwrap();

    assert_eq!(Group::count(group.union()), 3);
    assert_eq!(Group::count(group.intersection()), 0);
}

#[test]
fn test_third() {
    let group: Group = "ab\nac".parse().unwrap();

    assert_eq!(Group::count(group.union()), 3);
    assert_eq!(Group::count(group.intersection()), 1);
}

#[test]
fn test_fourth() {
    let group: Group = "a\na\na\na\na".parse().unwrap();

    assert_eq!(Group::count(group.union()), 1);
    assert_eq!(Group::count(group.intersection()), 1);
}

#[test]
fn test_fifth() {
    let group: Group = "b".parse().unwrap();

    assert_eq!(Group::count(group.union()), 1);
    assert_eq!(Group::count(group.intersection()), 1);
}
