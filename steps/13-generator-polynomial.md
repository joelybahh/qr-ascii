# 13 The generator polynomial

**Goal:** a function that builds the generator polynomial for a given number of error correction codewords.

## The problem

Reed–Solomon treats a list of bytes as a **polynomial**. The list `[1, 3, 2]` means 1·x² + 3·x + 2. There's no `x` anywhere in your program. A polynomial is just its list of coefficients, highest power first.

To make n error correction codewords, you need a particular polynomial called the **generator**. It's this product, with n factors:

(x + α⁰)(x + α¹)(x + α²) … (x + αⁿ⁻¹)

Each factor is a two-item list: `[1, αⁱ]`.

That splits into two smaller problems:

**1. Multiply two polynomials.** Exactly as at school: every term of one times every term of the other, then collect the terms with the same power. The only differences are that coefficients multiply with your step 12 function, and they add with XOR.

**2. Build the generator.** Start with `[1]`. Multiply by `[1, α⁰]`, then by `[1, α¹]`, and so on, n times.

Do the n = 2 case by hand first. It's (x + 1)(x + 2), and it's small enough to check every multiplication.

## Rust you'll meet

- `Vec<u8>` as a value you build and return
- Nested loops with index arithmetic
- Slices as inputs again

## Done when

| n | Coefficients |
| --- | --- |
| 2 | 1, 3, 2 |
| 7 | 1, 127, 122, 154, 164, 11, 68, 117 |
| 20 | 1, 152, 185, 240, 5, 111, 99, 6, 220, 112, 150, 69, 36, 187, 22, 228, 198, 121, 121, 165, 174 |

Also:

- The result always has n + 1 coefficients
- The first coefficient is always 1

Version 4 at level L uses n = 20.

<details>
<summary>Hints</summary>

- Multiplying a polynomial of length a by one of length b gives one of length a + b − 1.
- In the product, the term from position i of one and position j of the other lands at position i + j.
- For n = 2: x·x = x², then x·2 and 1·x add to (2 XOR 1)·x = 3x, then 1·2 = 2. Hence 1, 3, 2.
- Thonky's tool prints coefficients as powers of α. Run yours through the log table to compare, or use the exponent table in the checkpoints file.

</details>

## Read

- Thonky, [error correction coding](https://www.thonky.com/qr-code-tutorial/error-correction-coding): the generator polynomial section
- Thonky, [generator polynomial tool](https://www.thonky.com/qr-code-tutorial/generator-polynomial-tool): the answer for any n
