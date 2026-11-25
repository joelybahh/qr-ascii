# 14 Error correction codewords

**Goal:** given the data codewords, compute the error correction codewords.

## The problem

The error correction codewords are the **remainder** when you divide the data polynomial by the generator polynomial.

Concretely:

1. Treat your 80 data codewords as a polynomial.
2. Multiply it by xⁿ, where n is the number of error correction codewords. In list terms, that means appending n zeros.
3. Divide by the generator from step 13, using polynomial long division.
4. Throw the quotient away. The remainder is n bytes long. Those are your error correction codewords.

Polynomial long division is the long division you learned at school, with two changes: subtracting is XOR, and multiplying uses your GF(256) function. Because the generator's first coefficient is 1, you never have to divide a coefficient by anything.

Read Thonky's worked example and do the first two rounds of it by hand. It's tedious, but nothing else makes the loop as clear, and the code is short once you see it.

**Test on a known answer first.** Don't start with your own 80 codewords. Thonky's example is `HELLO WORLD` at Version 1, level M, with 10 error correction codewords:

| | |
| --- | --- |
| Data | 32, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17 |
| Expected | 196, 35, 39, 119, 235, 215, 231, 226, 93, 23 |

When that passes, move to yours.

**Why level L was a good choice.** At level L, Version 4 keeps all its data in a single block, so you run this once and you're done. Higher levels split the data into several blocks and interleave them. That's step 19 material.

## Rust you'll meet

- A mutable working buffer that you update in place
- Slices of part of a vector
- Tests built from known input and output

## Done when

- The `HELLO WORLD` test passes
- For `hi`, the 20 error correction codewords are `2D 1B 55 72 39 9B 66 16 F0 B2 3A EA EF 9A 41 BE F5 66 71 49`
- You always get exactly n codewords back, including when some of them are zero

That was the hard part of the whole project.

<details>
<summary>Hints</summary>

- Make a working list: the data followed by n zeros.
- Go through the data positions from first to last. At each one, look at the value currently in the working list at that position. Call it the lead.
- If the lead is 0, move on.
- Otherwise, multiply every coefficient of the generator by the lead, and XOR the results into the working list, starting at the current position.
- After the last data position, the final n entries of the working list are the remainder.
- If `HELLO WORLD` fails, check the pieces before the loop: is your n = 10 generator the one in the checkpoints file?
- Thonky's [show division steps](https://www.thonky.com/qr-code-tutorial/show-division-steps) tool prints every round for any input, so you can find the exact round where yours goes wrong.

</details>

## Read

- Thonky, [error correction coding](https://www.thonky.com/qr-code-tutorial/error-correction-coding): the full worked division
- Wikiversity, [Reed–Solomon codes for coders](https://en.wikiversity.org/wiki/Reed%E2%80%93Solomon_codes_for_coders): the encoding section
