use super::*;

#[test]
fn arrayify_works() {
    let input = vec![
        (0, Vector2::new(0, 0)),
        (1, Vector2::new(0, 1)),
        (2, Vector2::new(0, 2)),
        (3, Vector2::new(1, 0)),
        (4, Vector2::new(1, 1)),
        (5, Vector2::new(1, 2)),
        (6, Vector2::new(2, 0)),
        (7, Vector2::new(2, 1)),
        (8, Vector2::new(2, 2)),
        (9, Vector2::new(3, 0)),
        (10, Vector2::new(3, 1)),
        (11, Vector2::new(3, 2)),
    ];

    let formatted = arrayify(input, &Vector2::new(4, 3));
    assert_eq!(formatted.len(), 4);
    assert_eq!(formatted[0].len(), 3);

    let expected_1 = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8], vec![9, 10, 11]];

    assert_eq!(formatted, expected_1);
}

#[test]
fn roll_works() {
    let mut array = vec![
        vec![0, 0, 1, 1],
        vec![1, 1, 0, 0],
        vec![0, 1, 1, 0],
        vec![1, 0, 0, 1],
    ];

    let expected_1 = vec![
        vec![1, 0, 0, 1],
        vec![0, 1, 1, 0],
        vec![0, 0, 1, 1],
        vec![1, 1, 0, 0],
    ];

    let expected_2 = vec![
        vec![1, 1, 0, 0],
        vec![0, 0, 1, 1],
        vec![1, 0, 0, 1],
        vec![0, 1, 1, 0],
    ];

    let expected_3 = vec![
        vec![0, 1, 1, 0],
        vec![1, 1, 0, 0],
        vec![0, 0, 1, 1],
        vec![1, 0, 0, 1],
    ];

    roll(&mut array, 1, true, false);
    assert_eq!(array, expected_1);
    roll(&mut array, 1, true, false);
    assert_eq!(array, expected_2);
    roll(&mut array, 1, false, true);
    assert_eq!(array, expected_3);
}

#[test]
fn overlapping_adjacencies_works() {
    todo!()
}
