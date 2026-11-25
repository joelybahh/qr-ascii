# 16 Apply the mask

**Goal:** flip some of the data modules according to a formula.

## The problem

Raw data can come out with large blank areas, long runs of one colour, or a patch that looks like a finder pattern by accident. Scanners struggle with all of those.

The fix is a **mask**: a formula that says, for each position, flip this module or leave it. The scanner knows the formula and undoes it.

There are 8 masks. Use **mask 0**:

> Flip the module at (row, column) when `(row + column) % 2 == 0`.

That's a checkerboard.

The rule that matters: **only data modules are flipped.** That includes the error correction bits and the remainder bits. It does not include finder patterns, timing patterns, the alignment pattern, the dark module or the format areas. This is the second place your two layers earn their keep.

Any of the 8 masks produces a valid code that scans. Choosing the *best* one is a refinement, in step 19. But write this so the other seven are easy to add: the mask is a function from (row, column) to yes or no, and the code that walks the grid shouldn't care which one it's been given.

## Rust you'll meet

- Passing a function as a value: function pointers and closures
- `match` on a number to choose between formulas

## Done when

- Your debug view matches [`checkpoints/16-masked.txt`](../checkpoints/16-masked.txt)
- The function patterns are unchanged from step 09
- Applying the same mask a second time gives back the grid from step 15. Write that as a test

With `hi`, most of the code is pad codewords, so you'll see a repeating texture both before and after masking. That's expected. A longer message looks more random.

<details>
<summary>Hints</summary>

- Flipping is XOR with true.
- If the finder patterns have changed, you're masking everything rather than data modules only.
- The other seven formulas are in the [cheat sheet](../reference/version-4-cheatsheet.md).

</details>

## Read

- Thonky, [data masking](https://www.thonky.com/qr-code-tutorial/data-masking)
- Thonky, [mask patterns](https://www.thonky.com/qr-code-tutorial/mask-patterns): what each of the 8 looks like
- The Book, [chapter 13.1](https://doc.rust-lang.org/book/ch13-01-closures.html): closures
