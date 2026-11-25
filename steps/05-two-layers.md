# 05 Two layers

**Goal:** make the grid able to tell fixed patterns from data from "nothing here yet", and give it a way to stamp a small pattern onto itself.

## The problem

A QR code is two layers on top of each other:

- **Function patterns.** The same in every Version 4 code: the corner squares, the dotted lines and so on. This is the constant layer.
- **Data.** Changes with the message.

Later steps need to ask each module a question a plain `bool` can't answer:

- Step 15 fills in data but must skip every function module.
- Step 16 flips data modules but must leave function modules alone.

So a module needs to know more than dark or light. It needs to know what kind of thing it is.

**How do you represent that?**

| Option | Shape |
| --- | --- |
| One richer value per module | An `enum` with cases such as empty, function (dark or light), data (dark or light) |
| Two grids | One for colour, one for "is this a function module?" |
| Optional colour plus a flag grid | `Option<bool>` for the colour, where `None` means not set yet |

All three work. Pick the one you could explain to someone else most easily.

**Then write a stamp.** Given a small rectangular pattern and a (row, column) position, copy the pattern into the grid at that position and mark those modules as function modules. Steps 06 and 08 are just calls to this.

Decide what happens when a stamp would hang off the edge of the grid. Refuse? Clip? Crash? Any is defensible, but choose on purpose.

**Update the renderer.** Draw an empty module so it looks different from both dark and light. Over the next ten steps you'll watch the empty ones disappear.

**Add a debug view.** A second way to print the grid, one plain character per module:

| Character | Meaning |
| --- | --- |
| `#` | Dark |
| `.` | Light |
| `_` | Empty |
| `?` | Reserved, no value yet (you'll need this in step 09) |

This is the format of the files in [`checkpoints/`](../checkpoints/README.md). Having it means you can diff your grid against the expected one at every stage.

## Rust you'll meet

- `enum` and `match`, and the compiler insisting you handle every case
- `#[derive(Clone, Copy, PartialEq, Debug)]`
- `Option`
- `&mut self`

## Done when

- A new 33×33 grid draws as entirely empty
- Stamping a 3×3 test pattern at (10, 20) shows it at row 10, column 20, and nowhere else
- You can ask any module "are you a function module?" and get the right answer
- The debug view prints 33 lines of 33 characters

<details>
<summary>Hints</summary>

- An enum case can carry a value. A case for "function module" can carry its colour.
- A stamp is two nested loops over the small pattern, adding the offset to each coordinate.
- For the empty module in the pretty renderer, a dim grey background or a pair of middle dots both read well.

</details>

## Read

- The Book, [chapter 6](https://doc.rust-lang.org/book/ch06-00-enums.html): enums and `match`
