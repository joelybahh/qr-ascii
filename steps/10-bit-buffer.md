# 10 A bit buffer

**Goal:** something you can push "this number, using this many bits" into, and read the bits back out of.

## The problem

Put the grid away for a while. The next five steps are about numbers.

QR data is a stream of bits, and the fields in it aren't whole bytes: a 4-bit field, then an 8-bit field, then bytes that now straddle byte boundaries. You need a small tool for building such a stream.

It should be able to:

- **Append the lowest n bits of a number**, most significant bit first. Appending 5 using 3 bits adds `101`. Appending 1 using 4 bits adds `0001`. The leading zeros matter.
- Report its **length in bits**.
- Give you the contents back as **bytes**, 8 bits at a time.
- Print itself as a string of `0`s and `1`s. You'll use this for debugging in every step that follows.

**How do you store bits?**

| Option | Trade-off |
| --- | --- |
| One `bool` per bit in a `Vec` | Simple and obviously correct. Wasteful, but you have 800 bits, so nobody cares |
| Packed into bytes | Compact. Much fiddlier. Good practice at bit twiddling |

Start with the simple one. Come back for the packed one if you want a challenge.

## Rust you'll meet

- Shifts `>>` and `<<`, and masking with `&`
- The integer types, and converting between them with `as`
- Formatting in binary: `{:b}` and `{:08b}`
- More tests

## Done when

- Appending 4 (binary `0100`) using 4 bits, then 2 using 8 bits, prints `010000000010`
- Appending 1 using 4 bits prints `0001`
- 16 bits read back as bytes give 2 bytes with the right values
- Each of those is a test

<details>
<summary>Hints</summary>

- To get bit number i of a value, shift it right by i and keep the lowest bit.
- To go most significant bit first, count i downwards from n − 1 to 0.
- Shifting a number by its full width or more panics in debug builds. Appending 8 bits from a `u8` is fine. Check what happens if someone asks for 9.

</details>

## Read

- The Book, [appendix B](https://doc.rust-lang.org/book/appendix-02-operators.html): the operator table
- [`std::fmt`](https://doc.rust-lang.org/std/fmt/): formatting numbers in binary and hex
