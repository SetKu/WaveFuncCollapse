# Next Up

- [x] Integrate overlapping adjacencies into analyzer to make rules.
- [x] Create superposition filler.
- [x] Create collapser.
- [x] Create collapse_all (generator).
- [x] Hook system up with main.rs.
  - [x] Finish string reconstructor in main.rs.
- [x] Add random colours to the output if not using the default example.
- [x] Clean up input arguments and parsing.
- [ ] History tracking and seeding system (JSON export?).
    - [x] Create some sort of `Record` type to record the actions taken by the collapsing process. All the other functions which are deterministic don't need to have their processes encoded, as they will always unfold the same given the current state of the wave function. The sample should also include the encoded rules.
- [ ] Implement result mirroring mode where it only generates a quarter of half the output the symmetrically reflects it to produce a larger output. This avoids number crunching the larger output sizes.
- [ ] Test to see if simply propagating by blankly iterating over each element would be more efficient or correct.
    - My hunch is that doing this will cause the propagation time to decrease significantly. However, I believe it will come with the cost of more contradictions as propagation isn't starting from the propagations source.
    - Another similar way to improve efficiency to this would be to test if not caching the previous propagation loop's elements would lead to quicker times. This would reduce the number of heap allocations. However, I believe it would drastically increase the time spent searching the elements.
        - It might potentially be better to compromise and just store some sort of pointer to the elements that will be used for the next propagation loop. The issue I see with this is that it violates borrow checking and would have to be implemented using unsafe types.
