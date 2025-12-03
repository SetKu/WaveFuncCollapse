#[derive(Clone)]
struct Wave<C> {
    cells: Vec<C>,
}

#[derive(PartialEq, Eq)]
enum Relation {
    CanBeAdjacentTo,
    ConsistsOf,
}

#[derive(PartialEq, Eq)]
struct Constraint {
    kind: Relation,
}

