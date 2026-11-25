# 11 Encode the message

**Goal:** turn a piece of text into exactly 80 data codewords.

## The problem

A **codeword** is the QR word for a byte. Version 4 at level L holds 80 data codewords, which is 640 bits. Your message has to be wrapped up and padded to fill them exactly.

Build the stream in this order, using the bit buffer from step 10:

| # | Field | Size | Value |
| --- | --- | --- | --- |
| 1 | Mode indicator | 4 bits | `0100`, meaning byte mode |
| 2 | Character count | 8 bits | The number of bytes in the message |
| 3 | The message | 8 bits per byte | Each byte in order |
| 4 | Terminator | Up to 4 bits | Zeros. Fewer than 4 if there's less room than that left |
| 5 | Bit padding | 0 to 7 bits | Zeros, until the length is a multiple of 8 |
| 6 | Pad codewords | Whatever's left | `0xEC`, `0x11`, `0xEC`, `0x11`, … until there are 80 codewords |

**Bytes, not characters.** The count is the number of bytes. In Rust, strings are UTF-8, so `é` is two bytes and `🦀` is four. Think about which length you're asking for.

**What if the message is too long?** 78 bytes fit. 79 don't. Decide what your function does then. This is a good place to meet `Result`.

## Rust you'll meet

- `&str` versus bytes, and `.as_bytes()`
- `Result` and the `?` operator
- `.cycle()` for a sequence that repeats forever

## Done when

For the message `hi`:

- The stream after the terminator is `0100 00000010 01101000 01101001 0000`, 32 bits
- The codewords are `40 26 86 90`, then `EC 11` repeated 38 times
- There are exactly 80 codewords

And at the edges:

- A 78-byte message gives 80 codewords with no pad codewords at all
- A 79-byte message is rejected in whatever way you chose
- An empty message works

Work the `hi` example out by hand on paper first. It's four bytes of real content, and doing it once makes the bit order obvious.

<details>
<summary>Hints</summary>

- `h` is 104, binary `01101000`. `i` is 105, binary `01101001`.
- For a 78-byte message: 4 + 8 + 624 = 636 bits, and the terminator brings it to 640. The terminator is the only padding.
- In byte mode the stream is always a whole number of bytes after a full 4-bit terminator, so rule 5 adds nothing here. Implement it anyway. Other modes need it.

</details>

## Read

- Thonky, [data encoding](https://www.thonky.com/qr-code-tutorial/data-encoding): the terminator and padding rules, with a worked example
- Thonky, [byte mode](https://www.thonky.com/qr-code-tutorial/byte-mode-encoding)
- The Book, [chapter 8.2](https://doc.rust-lang.org/book/ch08-02-strings.html): strings and UTF-8
- The Book, [chapter 9](https://doc.rust-lang.org/book/ch09-00-error-handling.html): `Result`
