# Checkpoints

Expected output at each stage for the message `hi`, Version 4, level L, mask 0.

## Grid files

| File | State of the grid | After step |
| --- | --- | --- |
| `09-function-patterns.txt` | All function patterns drawn, format areas reserved, nothing else | 09 |
| `15-placement-order.txt` | Not a grid of modules: the order in which data positions are visited | 15 |
| `15-data-placed.txt` | Data and error correction bits placed, no mask yet | 15 |
| `16-masked.txt` | Mask 0 applied | 16 |
| `17-final.txt` | Format information written. This is the finished code | 17 |

Each grid file is 33 lines of 33 characters, one character per module, with no quiet zone.

| Character | Meaning |
| --- | --- |
| `#` | Dark |
| `.` | Light |
| `_` | Empty, nothing placed yet |
| `?` | Reserved for format information, no value yet |

In `15-placement-order.txt`, each data position holds its index in the visiting order, from 0 to 806. A `.` is a function module, which gets skipped.

## How to compare

Give your program a debug output that prints the grid in this same one-character format (step 05 asks for it). Then either:

- Send it to a file and run `diff` against the checkpoint, or
- Write a test that loads the checkpoint with `include_str!` and compares strings.

The second is better. Once the test exists, `cargo test` tells you whenever you break something.

## Numbers

### Step 11: data codewords for `hi`

Bit stream after the terminator, 32 bits:

```
0100 00000010 01101000 01101001 0000
```

The 80 data codewords, in hex:

```
40 26 86 90
```

followed by `EC 11` repeated 38 times.

### Step 12: GF(256)

| Check | Value |
| --- | --- |
| α⁰ to α¹² | 1, 2, 4, 8, 16, 32, 64, 128, 29, 58, 116, 232, 205 |
| α²⁵ | 3 |
| α²⁵⁴ | 142 |
| α²⁵⁵ | 1 (the cycle has wrapped) |
| log 2, log 3, log 29, log 255 | 1, 25, 8, 175 |
| 2 × 2 | 4 |
| 128 × 2 | 29 |
| 17 × 236 | 138 |
| 255 × 255 | 226 |
| anything × 0 | 0 |

### Step 13: generator polynomials

Coefficients from the highest power down.

| EC codewords | Coefficients |
| --- | --- |
| 2 | 1, 3, 2 |
| 7 | 1, 127, 122, 154, 164, 11, 68, 117 |
| 10 | 1, 216, 194, 159, 111, 199, 94, 95, 113, 157, 193 |
| **20** | 1, 152, 185, 240, 5, 111, 99, 6, 220, 112, 150, 69, 36, 187, 22, 228, 198, 121, 121, 165, 174 |

The same four as powers of α, which is how Thonky prints them:

| EC codewords | Exponents |
| --- | --- |
| 2 | 0, 25, 1 |
| 7 | 0, 87, 229, 146, 149, 238, 102, 21 |
| 10 | 0, 251, 67, 46, 61, 118, 70, 64, 94, 32, 45 |
| **20** | 0, 17, 60, 79, 50, 61, 163, 26, 187, 202, 180, 221, 225, 83, 239, 156, 164, 212, 212, 188, 190 |

### Step 14: error correction codewords

Thonky's worked example, `HELLO WORLD` at Version 1, level M, 10 error correction codewords:

| | |
| --- | --- |
| Data | 32, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17 |
| Expected | 196, 35, 39, 119, 235, 215, 231, 226, 93, 23 |

For `hi` at Version 4, level L, 20 error correction codewords, in hex:

```
2D 1B 55 72 39 9B 66 16 F0 B2 3A EA EF 9A 41 BE F5 66 71 49
```

### Step 15: placement

| Check | Value |
| --- | --- |
| Bits to place | 807 (800 from codewords, 7 remainder zeros) |
| First positions visited | (32, 32), (32, 31), (31, 32), (31, 31), (30, 32), (30, 31) |
| Last position visited | (24, 0) |

### Step 17: format information for level L, mask 0

| Stage | Bits |
| --- | --- |
| Level and mask | `01000` |
| BCH remainder | `1111010110` |
| Joined, before the XOR | `010001111010110` |
| After the XOR | `111011111000100` |

### Finished code

550 of the 1089 modules are dark.

## If you cross-check with another library

Not all libraries produce identical codes for the same settings. Python's `segno`, for one, adds an extra `00` byte after the terminator in byte mode, which changes every error correction codeword. Its codes still scan. The checkpoints here follow the spec and Thonky, and match Python's `qrcode` library exactly.
