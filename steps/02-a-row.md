# 02 A row of modules

**Goal:** a function that takes a list of `bool`s and draws them as one line.

## The problem

Walk along the list, draw each module with the function from step 01, and end the line.

Try it on a short list first: dark, light, dark, dark, light.

The interesting question is the parameter type. Rust has three things that all look like "a list of bools":

| Type | What it is |
| --- | --- |
| `[bool; 5]` | An array. Its length is part of the type |
| `Vec<bool>` | A growable list that owns its contents |
| `&[bool]` | A slice: a borrowed view into either of the above |

Which should a drawing function ask for? Think about what the function needs to do with the data, and whether it needs to keep it.

## Rust you'll meet

- Arrays, vectors and slices
- Borrowing with `&`
- `for` loops
- Iterators: `.iter()`, `.map()`, `.collect()`

## Done when

- A list of 33 alternating values prints as a dashed line. You'll meet this exact pattern again in step 07.
- The same function works on a 5-long and a 33-long list without changes

## Stretch

Write it twice: once with a `for` loop that builds up a `String`, once as a chain of iterator methods. Neither is wrong. Seeing both helps the rest of the project.

<details>
<summary>Hints</summary>

- A function that only reads a list should take a slice. Arrays and vectors both lend themselves out as slices.
- If step 01 returns text rather than printing, a row is those pieces joined together.

</details>

## Read

- The Book, chapter 4: [ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html), [references](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html) and [slices](https://doc.rust-lang.org/book/ch04-03-slices.html). The most important chapter in the Book.
