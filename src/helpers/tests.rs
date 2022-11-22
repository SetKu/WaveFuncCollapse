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
    // row (y) -> col (x)
    let input = vec![
        vec![0, 1, 1, 2],
        vec![0, 0, 1, 1],
        vec![1, 0, 0, 0],
        vec![1, 1, 1, 1],
    ];

    // col (x) -> row (y)
    let formatted_input = xy_swap(input.to_owned());

    println!("{:?}\n{:?}", input, formatted_input);

    // Indexes using [x][y], so expects 2d array as cols (x) -> rows (y)
    let result = overlapping_adjacencies(formatted_input, Vector2::new(2, 2), BorderMode::Clamp);

    assert_eq!(result.len(), 9);

    let origin = xy_swap(result[0].origin.to_owned());
    let e0 = vec![vec![0, 1], vec![0, 0]];
    assert_eq!(origin, e0);

    assert!(result[0].neighbours[0].is_none());
    assert!(result[0].neighbours[3].is_none());

    let i1r = result[0].neighbours[1].as_ref().unwrap();
    let i1 = xy_swap(i1r.to_owned());
    let e1 = vec![vec![1, 2], vec![1, 1]];
    assert_eq!(i1, e1);

    let i2r = result[0].neighbours[2].as_ref().unwrap();
    let i2 = xy_swap(i2r.to_owned());
    let e2 = vec![vec![1, 0], vec![1, 1]];
    assert_eq!(i2, e2);

    let exclude_test = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
    ];

    let val = xy_swap(exclude_test);
    let adjac = overlapping_adjacencies(val, Vector2::new(2, 2), BorderMode::Exclude);
    assert_eq!(adjac.len(), 1);
}

// #[test]
// fn rotate_ninety_works() {
// let i0 = vec![
// vec![0, 1],
// vec![2, 3],
// ];

// let e0 = vec![
// vec![1, 3],
// vec![0, 2],
// ];

// let r0 = rotate_ninety(i0.to_owned(), 1);
// assert_ne!(r0, i0);
// assert_eq!(r0, e0);

// let i1 = vec![
// vec![0, 1, 0],
// vec![1, 0, 1],
// vec![0, 0, 0],
// ];

// let e1 = vec![
// vec![0, 1, 0],
// vec![1, 0, 0],
// vec![0, 1, 0],
// ];

// let r1 = rotate_ninety(i1.to_owned(), 1);
// assert_ne!(r1, i1);
// assert_eq!(r1, e1);

// let i2 = vec![
// vec![0, 1, 0, 1],
// vec![1, 0, 1, 0],
// vec![0, 0, 0, 0],
// vec![1, 1, 1, 1],
// ];

// let e2 = vec![
// vec![1, 0, 0, 1],
// vec![0, 1, 0, 1],
// vec![1, 0, 0, 1],
// vec![0, 1, 0, 1],
// ];

// let r2 = rotate_ninety(i2.to_owned(), 1);
// assert_ne!(r2, i2);
// assert_eq!(r2, e2);

// let i3 = vec![
// vec![1],
// ];

// let r3 = rotate_ninety(i3.to_owned(), 1);
// assert_eq!(i3, r3);
// }
