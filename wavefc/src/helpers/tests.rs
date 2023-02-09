use super::*;

#[test]
fn orthogonals_works() {
    let neighbours = orthogonal(&Vector2::new(4, 4));
    assert_eq!(neighbours[0], Vector2::new(4, 5));
    assert_eq!(neighbours[1], Vector2::new(5, 4));
    assert_eq!(neighbours[2], Vector2::new(4, 3));
    assert_eq!(neighbours[3], Vector2::new(3, 4));
}

#[test]
fn remove_indexes_works() {
    let mut input = vec![5, 10, 15, 20, 25, 30];
    let to_remove = vec![0, 2, 4];
    remove_indexes(&mut input, to_remove);
    assert_eq!(input, vec![10, 20, 30]);
}
