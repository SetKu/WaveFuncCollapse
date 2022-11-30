# Next Up

- [x] Integrate overlapping adjacencies into analyzer to make rules.
- [x] Create superposition filler.
- [x] Create collapser.
- [x] Create collapse_all (generator).
- [x] Hook system up with main.rs.
  - [x] Finish string reconstructor in main.rs.
- [x] Add random colors to the output if not using the default example.
- [x] Clean up input arguments and parsing.
- [ ] History tracking and seeding system (JSON export?).
    - Create some sort of `Record` type to record the actions taken by the collapsing process. All the other functions which are deterministic don't need to have their processes encoded, as they will always unfold the same given the current state of the wave function. The sample should also include the encoded rules.
- [ ] Implement result mirrorring mode where it only generates a quarter of half the output the symmetrically reflects it to produce a larger output. This avoids number crunching the larger output sizes.
