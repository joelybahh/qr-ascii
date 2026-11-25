# Resources

## QR codes

- **[Thonky's QR Code Tutorial](https://www.thonky.com/qr-code-tutorial/)**. The best step-by-step guide. Each step file links to the matching page. Pages you'll use most:
  - [Data encoding](https://www.thonky.com/qr-code-tutorial/data-encoding) and [byte mode](https://www.thonky.com/qr-code-tutorial/byte-mode-encoding)
  - [Error correction coding](https://www.thonky.com/qr-code-tutorial/error-correction-coding)
  - [Module placement](https://www.thonky.com/qr-code-tutorial/module-placement-matrix)
  - [Data masking](https://www.thonky.com/qr-code-tutorial/data-masking)
  - [Format and version information](https://www.thonky.com/qr-code-tutorial/format-version-information)
  - Tables: [error correction](https://www.thonky.com/qr-code-tutorial/error-correction-table), [capacities](https://www.thonky.com/qr-code-tutorial/character-capacities), [log/antilog](https://www.thonky.com/qr-code-tutorial/log-antilog-table), [format strings](https://www.thonky.com/qr-code-tutorial/format-version-tables), [alignment locations](https://www.thonky.com/qr-code-tutorial/alignment-pattern-locations)
  - Tools: [generator polynomial](https://www.thonky.com/qr-code-tutorial/generator-polynomial-tool), [show division steps](https://www.thonky.com/qr-code-tutorial/show-division-steps)
- **[Nayuki: Creating a QR Code step by step](https://www.nayuki.io/page/creating-a-qr-code-step-by-step)**. Interactive. Type a message and it shows every intermediate stage with pictures. Very good for seeing the zigzag and the masks.
- **[Nayuki's QR Code generator library](https://www.nayuki.io/page/qr-code-generator-library)** ([source](https://github.com/nayuki/QR-Code-generator)). A clean reference implementation, including one in Rust. Reading it is a spoiler, so save it for when you're stuck or finished.
- **[Wikipedia: QR code](https://en.wikipedia.org/wiki/QR_code)**. Good diagrams of the overall structure.
- **[ISO/IEC 18004:2015](https://www.iso.org/standard/62021.html)**. The official spec. It costs money and you don't need it.

A warning about Thonky: it isn't consistent about whether a coordinate pair means (x, y) or (row, column). When a position looks wrong, go by the pictures, or by the cheat sheet in this folder, which is always (row, column).

## Reed–Solomon and finite fields

- **[Reed–Solomon codes for coders](https://en.wikiversity.org/wiki/Reed%E2%80%93Solomon_codes_for_coders)** (Wikiversity). Explains GF(256) and the encoder for programmers rather than mathematicians. You only need the sections up to and including encoding. The decoding half is for step 19.
- Thonky's [error correction coding](https://www.thonky.com/qr-code-tutorial/error-correction-coding) page works a full division by hand.

## Rust

- **[The Rust Programming Language](https://doc.rust-lang.org/book/)** ("the Book"). Chapters that match this project:
  - [1 Getting started](https://doc.rust-lang.org/book/ch01-00-getting-started.html) and [3 Common concepts](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html): before step 01
  - [4 Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html): around steps 02 and 03
  - [5 Structs](https://doc.rust-lang.org/book/ch05-00-structs.html): step 03
  - [6 Enums and pattern matching](https://doc.rust-lang.org/book/ch06-00-enums.html): step 05
  - [8 Collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html): step 03 onwards
  - [9 Error handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html): step 11
  - [11 Testing](https://doc.rust-lang.org/book/ch11-00-testing.html): step 09 onwards
  - [13 Closures and iterators](https://doc.rust-lang.org/book/ch13-00-functional-features.html): steps 15 and 16
  - [12 An I/O project](https://doc.rust-lang.org/book/ch12-00-an-io-project.html): step 18, for command line arguments
- **[Rust by Example](https://doc.rust-lang.org/rust-by-example/)**. Short runnable examples, good for "how do I write a match again?".
- **[Standard library docs](https://doc.rust-lang.org/std/)**. Worth learning to search. Start with [`u8`](https://doc.rust-lang.org/std/primitive.u8.html) and [`std::fmt`](https://doc.rust-lang.org/std/fmt/).
- **[Rustlings](https://github.com/rust-lang/rustlings)**. Small exercises, if you want a warm-up on a specific topic.
- **[Cargo book](https://doc.rust-lang.org/cargo/)** and **[Clippy](https://doc.rust-lang.org/clippy/)**. Run `cargo clippy` often. Its suggestions teach idiomatic Rust.

Read compiler errors all the way through. Rust's are unusually good and often contain the fix.

## Terminal drawing

- [Block Elements](https://en.wikipedia.org/wiki/Block_Elements): the Unicode characters `█ ▀ ▄` and friends.
- [ANSI escape codes](https://en.wikipedia.org/wiki/ANSI_escape_code): setting foreground and background colours.

## Tools

- **Your phone's camera.** The real test.
- **`zbarimg`** ([zbar](https://github.com/mchehab/zbar), `brew install zbar`). Decodes a screenshot from the command line. Useful for telling "my code is wrong" from "my camera doesn't like my terminal".
- **`qrencode`** ([site](https://fukuchi.org/works/qrencode/), `brew install qrencode`). `qrencode -t ANSIUTF8 "hi"` prints a QR code in the terminal, so you can see what a working one looks like. It picks its own mask, so its modules won't match yours.
- **Python `qrcode`** (`pip install qrcode`). Lets you force the version, level and mask, so it can produce a module-for-module reference for any message. The checkpoints here were verified with it.
