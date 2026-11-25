# 09 Dark module and reserved areas

**Goal:** finish the fixed layer, and prove it's right with a test.

## The problem

Two things are left.

**The dark module.** One module that is always dark, at (`4 × version + 9`, 8). For Version 4 that's (25, 8). It sits beside the top-right corner of the bottom-left finder's separator.

**The format information areas.** Step 17 writes 15 bits of format information into the grid, twice. You don't have those bits yet. But data placement in step 15 must stay out of those modules, so reserve them now.

| Copy | Where |
| --- | --- |
| 1 | Wrapped around the top-left finder: row 8 from column 0 to 8, and column 8 from row 0 to 8, skipping the two timing modules it crosses |
| 2, first half | Column 8, the bottom 7 rows (26 to 32) |
| 2, second half | Row 8, the rightmost 8 columns (25 to 32) |

That's 15 + 7 + 8 = 30 modules. The exact positions are in the [cheat sheet](../reference/version-4-cheatsheet.md).

How do you represent "this is a function module but it has no colour yet"? You may need to go back and adjust the choice you made in step 05. That's normal.

## The big check

Count the modules that are still empty. The answer must be **807**.

Why that number matters: Version 4 holds 100 codewords of 8 bits, plus 7 leftover bits. 100 × 8 + 7 = 807. If your count is 807, your fixed layer is almost certainly correct. If it's anything else, something overlaps or is missing.

Write this as your first test, so the computer checks it from now on.

Then compare your debug view against [`checkpoints/09-function-patterns.txt`](../checkpoints/09-function-patterns.txt). It should match character for character.

## Rust you'll meet

- `#[test]`, `assert_eq!` and `cargo test`
- Iterator methods: `.filter()` and `.count()`
- `include_str!`, which pulls a file's contents into your program at compile time. Useful for comparing against checkpoint files in a test

## Done when

- The dark module is at (25, 8)
- 30 modules are reserved for format information
- A test asserts that exactly 807 modules are empty, and it passes
- Your debug view matches the checkpoint file

That's the constant layer done. It never changes again. Everything from here is about the message.

<details>
<summary>Hints</summary>

- If your count is 808, check the dark module.
- If it's higher than that, a reserved strip is too short or missing. If it's lower than 807, a strip is too long, or something has been drawn where it shouldn't be.
- Diff against the checkpoint file. The first line that differs tells you where to look.
- Copy 1 of the format area crosses the timing patterns at (8, 6) and (6, 8). Those two modules stay timing modules.

</details>

## Read

- The Book, [chapter 11](https://doc.rust-lang.org/book/ch11-00-testing.html): writing tests
- Thonky, [module placement](https://www.thonky.com/qr-code-tutorial/module-placement-matrix): the dark module and reserved areas sections
- [`include_str!`](https://doc.rust-lang.org/std/macro.include_str.html)
