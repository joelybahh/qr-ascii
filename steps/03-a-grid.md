# 03 A grid of modules

**Goal:** draw a square of modules. Fill it with noise.

## The problem

Stack rows from step 02 to get a box. For Version 4 the box is 33×33. The rule is `17 + 4 × version`, so make the size something you compute from a version number rather than typing 33 everywhere.

**How do you store a grid?**

| Option | Shape | Trade-off |
| --- | --- | --- |
| List of lists | `Vec<Vec<bool>>` | Reads naturally as `grid[row][col]`. Nothing stops rows being different lengths |
| One flat list | `Vec<bool>` plus a size | One allocation, always square. You compute the index yourself: `row × size + col` |

Either is fine. Whichever you pick, wrap it in a `struct` with methods to get and set a module. Then the rest of the program never knows which you chose, and you can change your mind.

**Pick a coordinate convention and never change it.** Everything in this folder uses (row, column), counted from 0, with (0, 0) at the top left. Swapping row and column is the most common bug in the later steps.

**Where does noise come from?** The standard library has no random number generator. Options:

- Any formula on row and column that looks messy.
- Write a tiny pseudo-random generator yourself. An xorshift or a linear congruential generator is a few lines and a nice side quest.

It doesn't matter that the output is garbage. The point is that you can now turn any 33×33 grid of booleans into a picture.

## Rust you'll meet

- `struct` and `impl`
- Methods, and the difference between `&self` and `&mut self`
- `Vec`, `vec![value; count]`, indexing with `usize`
- Nested loops, or `.chunks()` on a flat list

## Done when

- You see a 33×33 block of noise
- It looks square. If it's a tall rectangle, revisit the two-characters-per-module decision in step 01
- Setting one specific module, say (0, 32), changes the top-right corner and not the bottom-left

## Stretch

Implement the `Display` trait for your grid, so that `println!("{grid}")` draws it.

<details>
<summary>Hints</summary>

- For noise, try multiplying row and column by different odd numbers, adding them, and testing the remainder against some small number.
- With a flat list, `.chunks(size)` hands you one row at a time, each of which is a slice. Your step 02 function takes a slice.

</details>

## Read

- The Book, [chapter 5](https://doc.rust-lang.org/book/ch05-00-structs.html): structs and methods
- The Book, [chapter 8.1](https://doc.rust-lang.org/book/ch08-01-vectors.html): vectors
