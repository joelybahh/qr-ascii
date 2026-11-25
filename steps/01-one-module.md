# 01 One module

**Goal:** a function that takes one `bool` and draws one square.

## The problem

A **module** is what the QR spec calls one square of the grid. `true` means dark, `false` means light. That's the whole input. Everything in this project ends up as a call to this function.

Three things to decide.

**1. How wide is a module?**

A character cell in a terminal is about twice as tall as it is wide. One character per module gives you a tall, thin code. Two characters per module is close to square.

**2. Which characters?**

| Option | Dark | Light | Notes |
| --- | --- | --- | --- |
| Plain ASCII | `##` | two spaces | Truly ASCII. Has gaps between characters, so it can be hard to scan |
| Full block | `██` (U+2588) | two spaces | Solid. What most terminal QR tools use |
| Background colours | two spaces on a black background | two spaces on a white background | Uses ANSI escape codes. The most reliable, because you control both colours |

Start with whichever you like. You can swap later, as long as only this one function knows about it.

**3. Which way round?**

Scanners expect **dark modules on a light background**. Your terminal probably has a dark background and light text. So if you print `██` for a dark module, the block comes out *light*, and you've drawn the code inverted. Some scanners cope with inverted codes. Plenty don't.

Two ways out:

- On a dark terminal, print the block for **light** modules and spaces for dark ones.
- Set both colours yourself with ANSI escape codes, so the terminal theme doesn't matter.

Decide now. It saves a confusing hour at step 18.

**One more design question:** should the function print the module, or return the text for it? Returning it makes testing easier later.

## Rust you'll meet

- Functions, parameters and return types
- `bool`, and `if` as an expression that produces a value
- `&str`, `String` and `char`, and why there are three
- `print!` and `println!`
- `const`

## Done when

- Running the program shows a dark module next to a light one
- You can tell them apart, and each looks roughly square
- On your terminal, the dark one really is the darker one

<details>
<summary>Hints</summary>

- A character you can't type goes in a string as a Unicode escape. The full block is U+2588.
- ANSI escape sequences start with the escape character, byte 27, hex 1B. Rust strings can contain a byte written in hex.
- If you set a colour, reset it afterwards, or the rest of your terminal session inherits it.
- The background colour codes you want are in the "SGR parameters" table on the Wikipedia page below. Look for 40 to 47 and 100 to 107.

</details>

## Read

- [Block Elements](https://en.wikipedia.org/wiki/Block_Elements)
- [ANSI escape codes](https://en.wikipedia.org/wiki/ANSI_escape_code)
- The Book, chapter 3, [functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html) and [control flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
