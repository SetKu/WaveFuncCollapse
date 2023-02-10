use super::*;

#[test]
fn orthogonals_works() {
    let neighbours = orthogonal(Vector2::new(4, 4));
    assert_eq!(neighbours[0], Vector2::new(4, 5));
    assert_eq!(neighbours[1], Vector2::new(5, 4));
    assert_eq!(neighbours[2], Vector2::new(4, 3));
    assert_eq!(neighbours[3], Vector2::new(3, 4));
}

#[test]
fn remove_indexes_works() {
    let mut input = vec![5, 10, 15, 20, 25, 30];
    let to_remove = vec![0, 2, 4];
    remove_indexes(&mut input, &to_remove);
    assert_eq!(input, vec![10, 20, 30]);
}

#[test]
fn string_deconstructor_works() {
    let input = "S C L\nC L L\nL L L";
    let result = deconstruct_string(input, false).grid;

    assert_eq!(
        result,
        vec![vec![0, 1, 2,], vec![1, 2, 2,], vec![2, 2, 2,],]
    );
}

#[test]
fn adjacencies_works() {
    let input = vec![vec![0, 1, 2], vec![1, 2, 2], vec![2, 2, 2]];

    let simple_test = adjacencies(&input, vec2(1, 1), true);
    assert_eq!(simple_test.len(), 9);

    let complex_test = adjacencies(&input, vec2(1, 2), true);
    assert_eq!(complex_test.len(), 6);
    assert_eq!(
        complex_test
            .iter()
            .filter(|v| v.root_data == vec![vec![2, 2]]
                && v.neighbours_data
                    == [None, Some(vec![vec![2, 2]]), None, Some(vec![vec![1, 2]]),])
            .count(),
        1
    );
}
