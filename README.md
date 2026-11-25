# A QR code in your terminal, in Rust

Twenty small steps. In step 01 you print a single square. By step 18 you print a QR code your phone can scan.

There is no code in this folder. You write all of it. Each step tells you what to build, what to think about, which bits of Rust you'll run into, and how to know you're done.

## What you're building

A **Version 4** QR code (33×33 modules), drawn with characters in the terminal.

To keep the first pass small, three choices are fixed:

| Choice | Value | Why |
| --- | --- | --- |
| Encoding mode | Byte | Works for any text, simplest to encode |
| Error correction level | L | In Version 4 it needs only one block, so there's no interleaving |
| Mask | 0 | Any mask scans. Choosing the best one comes later |

That gives you room for 78 bytes of message, which is enough for a URL.

## How to use this

- Do the steps in order. Each one builds on the one before.
- Each step has **hints** folded away at the bottom. Try the problem before you open them.
- Steps end with a **Done when** check. Several point at files in [`checkpoints/`](checkpoints/README.md) holding the exact expected output, so you can diff instead of squinting.
- Keep [`reference/version-4-cheatsheet.md`](reference/version-4-cheatsheet.md) open. It has every number and table you need for Version 4.
- Suggested rule: **no crates**, standard library only. Everything here is doable without them and you'll learn more.

The running example message is `hi`. All the checkpoints are for `hi`, Version 4, level L, mask 0.

## The path

### Part 1: draw things

You don't need to know anything about QR codes yet.

- [ ] [00 Setup](steps/00-setup.md)
- [ ] [01 One module](steps/01-one-module.md)
- [ ] [02 A row of modules](steps/02-a-row.md)
- [ ] [03 A grid of modules](steps/03-a-grid.md)
- [ ] [04 The quiet zone](steps/04-quiet-zone.md)

### Part 2: the fixed layer

The patterns that are the same in every Version 4 code.

- [ ] [05 Two layers](steps/05-two-layers.md)
- [ ] [06 Finder patterns](steps/06-finder-patterns.md)
- [ ] [07 Timing patterns](steps/07-timing-patterns.md)
- [ ] [08 Alignment pattern](steps/08-alignment-pattern.md)
- [ ] [09 Dark module and reserved areas](steps/09-dark-module-and-reserved-areas.md)

### Part 3: text to codewords

- [ ] [10 A bit buffer](steps/10-bit-buffer.md)
- [ ] [11 Encode the message](steps/11-encode-the-message.md)

### Part 4: Reed–Solomon

The hard part. Three steps, each testable on its own.

- [ ] [12 Arithmetic in GF(256)](steps/12-gf256.md)
- [ ] [13 The generator polynomial](steps/13-generator-polynomial.md)
- [ ] [14 Error correction codewords](steps/14-error-correction-codewords.md)

### Part 5: put it together

- [ ] [15 Place the data](steps/15-place-the-data.md)
- [ ] [16 Apply the mask](steps/16-apply-the-mask.md)
- [ ] [17 Format information](steps/17-format-information.md)
- [ ] [18 Scan it](steps/18-scan-it.md)

### Part 6: after it scans

- [ ] [19 Where next](steps/19-where-next.md)

## What's in this folder

| Path | What it is |
| --- | --- |
| `steps/` | The twenty steps |
| `reference/version-4-cheatsheet.md` | Every constant, position and table for Version 4 |
| `checkpoints/` | Expected output at each stage for the message `hi` |
| `RESOURCES.md` | Tutorials, references and tools, for QR codes and for Rust |

## How long it takes

Parts 1 and 2 are short steps, ten minutes to an hour each. Part 4 is where the time goes: plan on an evening or two. Step 17 is short but fiddly.

The checkpoints were produced by a throwaway reference script and matched module for module against the Python `qrcode` library, so if your output differs from them, the bug is on your side.
