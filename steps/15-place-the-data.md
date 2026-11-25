# 15 Place the data

**Goal:** write the bits into the empty modules of the grid, in the zigzag order the spec demands.

## The problem

Back to the grid. This is where the two layers from step 05 meet.

**First, the final bit stream:**

1. The 80 data codewords
2. The 20 error correction codewords
3. 7 zero bits, called remainder bits

Each codeword goes most significant bit first. That's 100 × 8 + 7 = **807 bits**, which is exactly the number of empty modules you counted in step 09. A 1 bit is a dark module.

**Then, the route.** The bits go into the grid along a zigzag path:

- Start at the bottom-right corner.
- Work in vertical strips two modules wide.
- In each strip, take the right module, then the left one, then move one row along and repeat.
- The first strip goes upward. At the top, move two columns left and come back down. Keep alternating.
- **Column 6 doesn't count.** It holds the vertical timing pattern. When the strips reach it, they skip over it, so the strips to its left are columns 5–4, 3–2 and 1–0.
- **Function modules are skipped.** If the route lands on a function module or a reserved one, pass over it without using up a bit.

Break it into three problems you can test separately:

| # | Problem | How to check it |
| --- | --- | --- |
| 1 | Produce the route: every (row, column) in visiting order, ignoring what's there | The first six are (32, 32), (32, 31), (31, 32), (31, 31), (30, 32), (30, 31) |
| 2 | Filter out the function modules | Exactly 807 positions remain, and the last is (24, 0) |
| 3 | Pair the positions with the bits and write them in | The grid has no empty modules left |

[`checkpoints/15-placement-order.txt`](../checkpoints/15-placement-order.txt) shows the whole route, with each data position numbered. Try without it first. It's the best debugging aid in the folder if you get stuck.

Look at what happens around the alignment pattern in that file. There's no special case for it. The skip rule handles it.

## Rust you'll meet

- `.rev()` to walk a range backwards
- `.zip()` to pair two sequences
- The borrow checker. You'll want to read the grid (is this a function module?) while changing it (set this module). Rust may object. Collecting the route into a list first, then writing, avoids the fight

## Done when

- The route has 807 positions after filtering
- No module in the grid is empty
- Your debug view matches [`checkpoints/15-data-placed.txt`](../checkpoints/15-data-placed.txt)

The picture won't scan yet. It still needs a mask and its format information.

<details>
<summary>Hints</summary>

- Counting strips from the right, starting at 0: even strips go up, odd strips go down.
- One way to handle column 6: let the strip's right-hand column step down by 2 from 32, and when it lands on 6, move it to 5.
- If you get 807 positions but the picture differs from the checkpoint, check the bit order inside each codeword, then the order within a strip (right before left).
- If the bottom-right matches and things go wrong further left, find the first numbered position in the order file where your route differs.

</details>

## Read

- Thonky, [module placement](https://www.thonky.com/qr-code-tutorial/module-placement-matrix): the data bits section, with pictures of the route
- Thonky, [structure final message](https://www.thonky.com/qr-code-tutorial/structure-final-message): assembling the stream and the remainder bits
- Nayuki, [step by step](https://www.nayuki.io/page/creating-a-qr-code-step-by-step): shows the zigzag drawn over the grid
