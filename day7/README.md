# Day 7

<https://adventofcode.com/2024/day/7>

## Table of Contents

- [Task 1](task1/src/main.rs)
- [Task 2](task2/src/main.rs)

Running through all permutations in task 2 was still pretty quick here.

```bash
~/advent-of-code-2024/day7/task2$ time ./target/release/task2
Sum of valid equations: ...

real    0m4.972s
user    0m37.236s
sys     0m30.462s
```

Ended up adding some performance improvements to the code, which ended up being ~5x faster. The improvements were:

- Added momoization to the `multi_cartesian_product` function calls.
- Utilised `into_par_iter` to parallelize more of the iterators.
- Improvments to the concatenations to avoid string processing in favor of just calling pow.

```bash
~/advent-of-code-2024/day7/task2$ time ./target/release/task2
Sum of valid equations: ...

real    0m1.174s
user    0m9.680s
sys     0m1.025s
```
