# 04 The quiet zone

**Goal:** draw the grid with a border of light modules, 4 wide, on every side.

## The problem

A scanner finds a QR code partly by the empty margin around it. The spec asks for at least 4 light modules on all four sides. This is called the **quiet zone**.

Light means *light*, not *nothing*. On a dark terminal, blank space is dark, so the border has to be drawn like any other light module.

**Where should the border live?**

- In the grid: make it 41×41 and shift everything by 4.
- In the drawing code: keep the grid 33×33 and add the border while printing.

Think about which keeps your coordinates matching the ones in the spec and the cheat sheet.

## Rust you'll meet

- Constants and simple arithmetic on `usize`
- Repeating a string or a value a number of times

## Done when

- The noise square sits inside an even light frame
- The whole picture is 41 modules across, which is 82 characters. Check your terminal window is at least that wide, or the lines will wrap
- The frame is the same thickness on all four sides, measured in modules

At this point you have a renderer. What you're looking at is a broken QR code, and every step from here makes it less broken.

<details>
<summary>Hints</summary>

- Top and bottom are 4 full rows of light modules, each 41 modules long.
- Each grid row gets 4 light modules before it and 4 after.
- You already have a function that draws a row from a slice.

</details>
