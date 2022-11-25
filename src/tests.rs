use super::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

#[test]
fn wave_collapse_once_works() {
    let mut wave = Wave::new();
    let test_sample = vec![vec![0, 1, 2]];
    wave.analyze(test_sample, Vector2::new(1, 1), BorderMode::Clamp);
    wave.fill(Vector2::new(3, 1)).expect("Fill failed.");
    wave.collapse_once();

    let elements_collapsed = wave.elements.iter().filter(|e| e.is_collapsed()).count();
    assert_eq!(elements_collapsed, 1);
}

#[test]
fn dedup_patterns_works() {
    let mut patterns = vec![
        Pattern {
            is_transform: false,
            id: 0,
            count: 1,
            contents: vec![vec![0]],
            rules: vec![Rule::new(0, vec![vec![1]])],
        },
        Pattern {
            is_transform: false,
            id: 1,
            count: 1,
            contents: vec![vec![1]],
            rules: vec![Rule::new(2, vec![vec![0]])],
        },
        Pattern {
            is_transform: false,
            id: 2,
            count: 1,
            contents: vec![vec![1]],
            rules: vec![Rule::new(2, vec![vec![0]]), Rule::new(2, vec![vec![0]])],
        },
    ];

    dedup_patterns(&mut patterns);

    assert_eq!(patterns.len(), 2);
    assert!(patterns.iter().all(|p| p.rules.len() == 1));
    assert_eq!(
        patterns.iter().map(|p| p.count).filter(|c| *c == 2).count(),
        1,
        "{:#?}",
        patterns,
    );

    let mut hash_list: Vec<u64> = vec![];

    for pattern in patterns.iter() {
        let mut hasher = DefaultHasher::new();
        hasher.write_usize(pattern.count);

        for row in &pattern.contents {
            for n in row {
                hasher.write_usize(*n);
            }
        }

        for rule in &pattern.rules {
            hasher.write_u8(rule.direction);

            for row in &rule.content {
                for n in row {
                    hasher.write_usize(*n);
                }
            }
        }

        hash_list.push(hasher.finish());
    }

    let copy = hash_list.to_owned();
    hash_list.dedup();

    assert_eq!(copy, hash_list);
}

#[test]
fn wave_analyzer_works() {
    let mut wave = Wave::new();
    let input = vec![vec![0, 1, 2], vec![0, 1, 2], vec![0, 1, 2]];

    wave.flags.push(Flags::NoTransforms);
    wave.analyze(input, Vector2::new(2, 2), BorderMode::Clamp);

    assert_eq!(wave.patterns.len(), 4);
}
