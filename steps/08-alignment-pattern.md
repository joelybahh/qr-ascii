# 08 Alignment pattern

**Goal:** draw the small square near the bottom-right corner.

## The problem

The **alignment pattern** helps a scanner correct for a code that's bent or photographed at an angle. It's 5×5:

- a dark border, one module thick
- a light ring
- a single dark module in the centre

It's a small cousin of the finder pattern, and you can build it the same way.

**Where does it go?** Each version has a list of alignment coordinates. For Version 4 the list is `[6, 26]`. The rule is:

1. Every pairing of numbers from the list is a candidate **centre**: (6, 6), (6, 26), (26, 6), (26, 26).
2. Throw away any candidate that would overlap a finder pattern.

For Version 4 that leaves one: centre (26, 26).

You could just hard-code that position. Implementing the rule is more interesting, and it's what you need for bigger versions, which have many alignment patterns.

Notice that unlike everything so far, this pattern sits in the middle of the data area. It's the reason data placement in step 15 can't assume the data region is a simple shape.

## Rust you'll meet

Nothing new. This step is a test of whether steps 05 and 06 left you with reusable pieces.

## Done when

- One 5×5 pattern covering rows 24 to 28 and columns 24 to 28
- 25 more function modules. 838 still empty

<details>
<summary>Hints</summary>

- The stamp takes a top-left corner. The spec gives you a centre. Convert.
- The distance-from-centre rule from step 06 works here too. This time the light ring is at distance 1.
- To test for overlap with a finder, you can ask whether the module at the candidate centre is already a function module. For Version 4, all three rejected centres land on a finder's corner.
- That shortcut has a catch in bigger versions. From Version 7 up, some alignment patterns sit on top of the timing lines, and that's allowed. So there the check has to look at finders only, or run before the timing patterns are drawn.

</details>

## Read

- Thonky, [alignment pattern locations](https://www.thonky.com/qr-code-tutorial/alignment-pattern-locations): the list for every version
