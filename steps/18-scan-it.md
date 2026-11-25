# 18 Scan it

**Goal:** point your phone at the terminal and read `hi`.

## The problem

Draw the finished grid with the pretty renderer from steps 01 to 04, quiet zone included, and scan it.

If it works first time, enjoy it. If it doesn't, work out which half is at fault.

### Is the grid right?

Compare your debug view with [`checkpoints/17-final.txt`](../checkpoints/17-final.txt).

If you checked each stage as you went, it will match. **If it differs**, where it differs tells you which step to revisit:

| Where the differences are | Look at |
| --- | --- |
| Only in the strips beside the finders | Step 17, format information |
| Only at the left: columns 0 to 8, and the lower part of columns 9 and 10 | Steps 12 to 14. That region holds the error correction codewords |
| Starting from the bottom-right corner | Step 11, the data codewords |
| A checkerboard of differences across the data area | Step 16, the mask |
| Scattered across the data area, with the bottom right correct | Step 15, the route |
| In the corners or the dotted lines | Steps 06 to 09 |

Level L can repair only a few bad codewords, so almost any bug stops the code scanning. Don't expect a nearly-right code to scrape through.

**If it matches**, the code is correct and the drawing is the problem.

### Is the drawing right?

- **Polarity.** Dark modules must be dark on screen, and light ones light. See step 01.
- **Quiet zone.** It has to be actually light, all the way round, and 4 modules wide.
- **Shape.** Modules should be about square. A stretched code often still scans. A badly stretched one won't.
- **Gaps.** Some terminals leave thin lines between rows or between characters. If you see stripes through your modules, reduce the line spacing in your terminal settings, or switch to drawing with background colours.
- **Wrapping.** The picture is 82 characters wide. If the window is narrower, lines wrap and the code is destroyed.
- **Size.** If the code fills the whole screen, move the phone back, or make the terminal font smaller.

To take the camera out of the question, take a screenshot and decode it with `zbarimg`.

### Then make it yours

- Read the message from the command line instead of hard-coding `hi`.
- Encode a URL and open it from your phone.
- Find the longest message that fits. It should be 78 bytes.

## Rust you'll meet

- `std::env::args`
- Turning errors into a helpful message and a non-zero exit code

## Done when

Your phone reads back what you typed.

## Read

- The Book, [chapter 12.1](https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html): command line arguments
