# 19 Where next

You have a working QR code generator with three choices fixed. Each idea below removes one of those limits or adds something new. They're roughly in order of how naturally they follow.

## Smaller output with half blocks

The characters `▀`, `▄` and `█`, plus a space, let one character cell show two modules stacked vertically. One module per character across and two per character down gives square modules at half the size: 41 columns by 21 lines.

It's a pure rendering change. If your renderer is separate from your grid, nothing else moves.

## Choose the best mask

Implement all 8 mask formulas. Then score each result with the spec's four penalty rules and keep the lowest score:

1. Runs of 5 or more modules of the same colour in a row or column
2. 2×2 blocks of one colour
3. Patterns that look like a finder
4. An uneven balance of dark and light

Remember the format string has to change to match the mask you pick.

Read: Thonky, [data masking](https://www.thonky.com/qr-code-tutorial/data-masking), which works through each rule.

## Other error correction levels

Levels M, Q and H in Version 4 split the data into blocks. Each block gets its own error correction codewords, and then the blocks are **interleaved**: first codeword of each block, then second codeword of each block, and so on.

| Level | Blocks | Data codewords per block | EC codewords per block |
| --- | --- | --- | --- |
| M | 2 | 32 | 18 |
| Q | 2 | 24 | 26 |
| H | 4 | 9 | 16 |

Your Reed–Solomon code doesn't change. You call it once per block.

Read: Thonky, [structure final message](https://www.thonky.com/qr-code-tutorial/structure-final-message).

## Other versions

Make the version a parameter. What changes:

- The size, which you already compute
- The alignment pattern list, where your general rule from step 08 pays off
- The capacity and block tables
- The number of remainder bits
- From Version 7: two extra blocks of **version information**
- From Version 10: the character count field in byte mode becomes 16 bits

Then pick the smallest version that fits the message automatically.

Read: Thonky's [error correction table](https://www.thonky.com/qr-code-tutorial/error-correction-table) and [alignment locations](https://www.thonky.com/qr-code-tutorial/alignment-pattern-locations).

## Denser encoding modes

Numeric mode packs 3 digits into 10 bits. Alphanumeric mode packs 2 characters into 11 bits. Both let the same version hold more.

Read: Thonky, [data encoding](https://www.thonky.com/qr-code-tutorial/data-encoding).

## Write an image file

Still without crates: the PBM format is plain text, a header followed by 0s and 1s. SVG is text too. Either gives you something you can print.

## Go the other way

Write the decoder. Take a grid, read the format information, undo the mask, walk the zigzag, rebuild the codewords and recover the text.

Then the big one: use the error correction codewords to *repair* damage. Flip some modules on purpose and recover the message anyway. This is the second half of the Wikiversity article, and it's a substantial project in its own right.

Read: Wikiversity, [Reed–Solomon codes for coders](https://en.wikiversity.org/wiki/Reed%E2%80%93Solomon_codes_for_coders), the decoding sections.

## Tidy up

- Split the code into modules: grid, renderer, encoder, Reed–Solomon
- Give your errors a proper type
- Compare your design with [Nayuki's Rust implementation](https://github.com/nayuki/QR-Code-generator). You've earned the spoiler now
