# 06 Finder patterns

**Goal:** draw the three big corner squares, each with its light separator.

## The problem

The **finder pattern** is how a scanner locates the code and works out which way up it is. It's 7×7:

- a dark border, one module thick
- inside that, a light ring, one module thick
- in the middle, a dark 3×3 block

There are three of them: top left, top right, bottom left. The bottom-right corner has none, which is how the scanner knows the orientation.

Top-left corners of the three, for a grid of side `size`:

| Finder | Position |
| --- | --- |
| Top left | (0, 0) |
| Top right | (0, size − 7) |
| Bottom left | (size − 7, 0) |

Define the pattern **once**, then stamp it three times with the stamp from step 05.

**Two ways to define a 7×7 pattern.** Try the one that appeals, then maybe the other:

- As a constant table: seven rows of seven values, written out.
- As a rule: a module's colour depends only on how far it is from the centre.

**Separators.** Each finder has a strip of light modules, one wide, along the sides that face into the code. They are light, but they are still function modules, not empty ones. A finder with its separator fills an 8×8 corner.

**A Rust trap.** Grid positions are `usize`, which can't be negative. If you compute "one to the left of column 0", the program panics with *attempt to subtract with overflow*. It's worth designing around this rather than patching it: if you think of each corner as an 8×8 block at a known position, you never need to step outside the grid.

## Rust you'll meet

- `const` tables, for example an array of arrays
- `usize` underflow, and `checked_sub`
- Reusing one function three times with different arguments

## Done when

- Three finder patterns in the right corners, none in the bottom right
- A light strip separates each from the rest of the grid
- 192 function modules (3 × 64), 897 still empty
- The three corners match [`checkpoints/09-function-patterns.txt`](../checkpoints/09-function-patterns.txt). The rest of that file comes in the next three steps.

<details>
<summary>Hints</summary>

- For the rule-based version: measure distance from the centre as whichever is larger, the row distance or the column distance. The finder is dark everywhere except where that distance is exactly 2.
- For the separators, one easy approach is to fill each 8×8 corner with light function modules first, then stamp the 7×7 finder on top.
- The three 8×8 corners start at (0, 0), (0, size − 8) and (size − 8, 0). Work out where the 7×7 finder sits inside each. It's a different spot in each corner.

</details>

## Read

- Thonky, [module placement](https://www.thonky.com/qr-code-tutorial/module-placement-matrix): the finder pattern and separator sections. Thonky's coordinates here are (x, y), the other way round from this folder.
