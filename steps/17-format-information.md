# 17 Format information

**Goal:** fill in the 30 reserved modules, so a scanner knows how to read the rest.

## The problem

Before a scanner can read your data, it needs two facts: which error correction level you used, and which mask. Those are stored in a 15-bit **format string**, written into the grid twice.

A wrong format string is the most common reason a finished-looking code won't scan. There are two halves to getting it right.

### Part 1: build the 15 bits

| # | Do this | For level L, mask 0 |
| --- | --- | --- |
| 1 | Two bits for the level. L is `01` | `01` |
| 2 | Three bits for the mask number | `000` |
| 3 | Join them: 5 bits | `01000` |
| 4 | Compute 10 error correction bits for those 5 | `1111010110` |
| 5 | Join again: 15 bits | `010001111010110` |
| 6 | XOR with `101010000010010` | `111011111000100` |

Row 4 is the work. Put ten zeros after your 5 bits, divide by the generator `10100110111`, and keep the 10-bit remainder. It's long division again, but simpler than step 14: these are single bits, so "subtract" is plain XOR, and there's no GF(256) involved.

There are only 32 possible format strings, and tables of them exist. A sensible route: hard-code `111011111000100` first and get your code scanning, then write the calculation and test it against the table.

### Part 2: put them in the right modules

Each of the 15 bits goes into two places. The positions are in the [cheat sheet](../reference/version-4-cheatsheet.md), as two tables.

Roughly:

- **Copy 1** wraps around the top-left finder. It runs along row 8 from the left edge, turns the corner at (8, 8), then runs up column 8 to the top edge. It hops over the two timing modules.
- **Copy 2** is in two pieces. The first 7 bits run up column 8 from the bottom edge. The last 8 run along row 8 to the right edge.

The trap is **bit order**. Both copies start from the same end of the string, but they travel in directions that are easy to get backwards. Try to get it from Thonky's diagram first, then check yourself against the tables.

These modules are not masked. Write them after step 16, or make sure the mask leaves them alone.

## Rust you'll meet

- Bit manipulation on a `u16` or `u32`
- A `const` table of positions

## Done when

- The calculation gives `111011111000100` for level L and mask 0, and matches the cheat sheet for the other seven masks
- No `?` is left in your debug view
- Your debug view matches [`checkpoints/17-final.txt`](../checkpoints/17-final.txt), character for character
- 550 modules are dark

<details>
<summary>Hints</summary>

- For the division: look at the bits from the left. Wherever the current leading bit is 1, XOR the generator in, lined up under that bit. Stop when what's left fits in 10 bits.
- For level L and mask 0 the bits read: f0 to f14 = 1 1 1 0 1 1 1 1 1 0 0 0 1 0 0.
- So in the finished grid, row 8 from the left edge reads dark, dark, dark, light, dark, dark, then the timing module, then dark, dark.
- And column 8 from the bottom edge upward reads dark, dark, dark, light, dark, dark, dark, then the dark module.

</details>

## Read

- Thonky, [format and version information](https://www.thonky.com/qr-code-tutorial/format-version-information): the calculation worked through, and the placement diagram
- Thonky, [format string table](https://www.thonky.com/qr-code-tutorial/format-version-tables): all 32
