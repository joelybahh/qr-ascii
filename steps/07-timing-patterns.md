# 07 Timing patterns

**Goal:** draw the two dotted lines that connect the finder patterns.

## The problem

The **timing patterns** are one row and one column of alternating dark and light modules. A scanner uses them to work out how big a module is and where the grid lines fall.

- One runs along **row 6**, between the top two finders.
- One runs down **column 6**, between the two left finders.

Each covers indexes 8 to `size − 9` inclusive. For Version 4 that's 8 to 24, so 17 modules each. A module is dark when its index is even, so both lines start dark and end dark.

They're function modules, like the finders.

A design choice: you can draw only the 17 modules between the finders, or you can draw the whole row and column and make sure the finders and separators win where they overlap. Both give the same picture.

Something to notice: why row 6 and column 6? Look at where the finder's edge is.

## Rust you'll meet

- Ranges: `a..b` and `a..=b`, and the difference
- The remainder operator `%`

## Done when

- A dotted line joins the top two finders, and another joins the two left finders
- Each line starts and ends on a dark module, right next to a separator
- 34 more function modules. 863 still empty

<details>
<summary>Hints</summary>

- This is step 02's alternating row again, written into the grid instead of onto the screen.
- If your line ends on a light module, you have an off-by-one in the range.

</details>

## Read

- Thonky, [module placement](https://www.thonky.com/qr-code-tutorial/module-placement-matrix): the timing pattern section
