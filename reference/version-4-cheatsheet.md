# Version 4 cheat sheet

Everything here is for **Version 4**. Coordinates are always **(row, column)**, counted from 0, with (0, 0) at the top left.

## Size

| | |
| --- | --- |
| Side length | 33 modules (`17 + 4 × version`) |
| Total modules | 1089 |
| Quiet zone | 4 light modules on every side, so 41×41 when drawn |

## Module budget

| What | Modules |
| --- | --- |
| 3 finder patterns with separators (8×8 each) | 192 |
| 2 timing patterns (17 each) | 34 |
| 1 alignment pattern (5×5) | 25 |
| Dark module | 1 |
| Format information (2 copies × 15) | 30 |
| **Function modules in total** | **282** |
| **Left for data** | **807** |

807 = 100 codewords × 8 bits + 7 remainder bits.

## Function patterns

**Finder patterns.** 7×7: a dark border, a light ring, a dark 3×3 centre. Top-left corners at:

| Corner | Position |
| --- | --- |
| Top left | (0, 0) |
| Top right | (0, 26) |
| Bottom left | (26, 0) |

**Separators.** A light strip one module wide along the inner sides of each finder. With its separator, each finder owns an 8×8 corner.

**Timing patterns.** Row 6 and column 6, from index 8 to index 24. Dark where the index is even, light where it's odd.

**Alignment pattern.** 5×5: a dark border, a light ring, one dark centre. Centre at (26, 26), so it covers rows 24–28 and columns 24–28.

The general rule: Version 4's alignment coordinate list is `[6, 26]`. Every pairing of those is a candidate centre. Drop any that would overlap a finder. Only (26, 26) is left.

**Dark module.** Always dark, at (`4 × version + 9`, 8) = (25, 8).

## Format information positions

The format string is 15 bits. Call them f0 to f14, where f0 is the leftmost (most significant) bit as you'd write it down.

**Copy 1**, around the top-left finder:

| Bit | f0 | f1 | f2 | f3 | f4 | f5 | f6 | f7 | f8 | f9 | f10 | f11 | f12 | f13 | f14 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Row | 8 | 8 | 8 | 8 | 8 | 8 | 8 | 8 | 7 | 5 | 4 | 3 | 2 | 1 | 0 |
| Col | 0 | 1 | 2 | 3 | 4 | 5 | 7 | 8 | 8 | 8 | 8 | 8 | 8 | 8 | 8 |

It runs left to right along row 8, turns the corner at (8, 8), then runs up column 8. It jumps over (8, 6) and (6, 8), which belong to the timing patterns.

**Copy 2**, split between the other two finders:

| Bit | f0 | f1 | f2 | f3 | f4 | f5 | f6 | f7 | f8 | f9 | f10 | f11 | f12 | f13 | f14 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Row | 32 | 31 | 30 | 29 | 28 | 27 | 26 | 8 | 8 | 8 | 8 | 8 | 8 | 8 | 8 |
| Col | 8 | 8 | 8 | 8 | 8 | 8 | 8 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 |

f0 to f6 run up column 8 from the bottom edge, stopping just below the dark module. f7 to f14 run left to right along row 8 to the right edge.

## Capacity

Version 4 always has 100 codewords in total.

| Level | Data codewords | EC codewords per block | Blocks | Data codewords per block | Max bytes in byte mode |
| --- | --- | --- | --- | --- | --- |
| **L** | **80** | **20** | **1** | **80** | **78** |
| M | 64 | 18 | 2 | 32 | 62 |
| Q | 48 | 26 | 2 | 24 | 46 |
| H | 36 | 16 | 4 | 9 | 34 |

Remainder bits after the last codeword: **7**, all zero.

## Data encoding

| Mode | Indicator |
| --- | --- |
| Numeric | `0001` |
| Alphanumeric | `0010` |
| **Byte** | **`0100`** |
| Kanji | `1000` |

- Character count field in byte mode, Versions 1 to 9: **8 bits**
- Terminator: up to four `0` bits
- Pad bytes: `0xEC` then `0x11`, alternating

## Reed–Solomon

| | |
| --- | --- |
| Field | GF(256) |
| Reducing polynomial | `0x11D` (285, binary `100011101`) |
| Generator element α | 2 |
| Roots of the generator polynomial | α⁰, α¹, … αⁿ⁻¹ |

## Mask patterns

A data module at (row, col) is flipped when the formula is true. Division rounds down.

| Mask | Flip when |
| --- | --- |
| **0** | **`(row + col) % 2 == 0`** |
| 1 | `row % 2 == 0` |
| 2 | `col % 3 == 0` |
| 3 | `(row + col) % 3 == 0` |
| 4 | `(row / 2 + col / 3) % 2 == 0` |
| 5 | `(row * col) % 2 + (row * col) % 3 == 0` |
| 6 | `((row * col) % 2 + (row * col) % 3) % 2 == 0` |
| 7 | `((row + col) % 2 + (row * col) % 3) % 2 == 0` |

## Format information contents

| | |
| --- | --- |
| Level bits | L `01`, M `00`, Q `11`, H `10` |
| Layout | 2 level bits, 3 mask bits, 10 BCH bits |
| BCH generator | `10100110111` (`0x537`) |
| Final XOR | `101010000010010` (`0x5412`) |

Finished format strings for level L:

| Mask | Format string |
| --- | --- |
| **0** | **`111011111000100`** |
| 1 | `111001011110011` |
| 2 | `111110110101010` |
| 3 | `111100010011101` |
| 4 | `110011000101111` |
| 5 | `110001100011000` |
| 6 | `110110001000001` |
| 7 | `110100101110110` |
