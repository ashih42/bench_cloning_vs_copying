# Cloning vs Copying

This is a quick experiment to measure the low-level performance impacts of using Rust's explicit `.clone()` or implicit copy operation on a struct with no heap data (a struct that can and should be copied, instead of cloned).

This experiment explores: How much worse is it really to use cloning instead of copying in this case? (Spoiler: The difference appears negligible.)

## How to Run

1. Run `cargo bench`.
2. See output at `target/criterion/report/index.html`.

You can also check out my results running this experiment on an M1 macbook at [my_results/criterion/report/index.html](my_results/criterion/report/index.html).
