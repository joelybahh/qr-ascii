use crate::bits::BitBuffer;

const CODEWORDS: usize = 80;
const MAX_BITS: usize = 640;
const BYTE: usize = 8;

const CODEWORD_1: u32 = 0xEC;
const CODEWORD_2: u32 = 0x11;

pub fn encode(message: &str) -> Vec<u8> {
    let mut buffer = BitBuffer::new();
    push_byte_segment(&mut buffer, message);

    let bits_left = MAX_BITS - buffer.len();
    buffer.push_bits(0, std::cmp::min(4, bits_left)); // terminator

    // while not a multiple of 8, push a bit until it is
    while buffer.len() % BYTE != 0 {
        buffer.push_bits(0, 1);
    }

    let words_left = CODEWORDS - buffer.to_bytes().len();
    for i in 0..words_left {
        let even = i % 2 == 0;
        let pad = if even { CODEWORD_1 } else { CODEWORD_2 };
        buffer.push_bits(pad, BYTE);
    }

    return buffer.to_bytes();
}

fn push_byte_segment(buffer: &mut BitBuffer, message: &str) {
    buffer.push_bits(0b0100, 4);
    buffer.push_bits(
        message
            .len()
            .try_into()
            .expect("usize value exceeded u32::MAX"),
        8,
    );

    for byte in message.bytes() {
        buffer.push_bits(u32::from(byte), BYTE); // each byte of the message
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_segment_for_hi_is_mode_count_then_bytes() {
        let mut bb = BitBuffer::new();
        push_byte_segment(&mut bb, "hi");

        // mode 0100, count 00000010, h 01101000, i 01101001. No terminator.
        assert_eq!(bb.len(), 28);
        assert_eq!(bb.to_bit_string(), "0100000000100110100001101001");
    }

    #[test]
    fn hi_encodes_to_the_checkpoint_codewords() {
        // checkpoints/README.md, step 11: 40 26 86 90, then EC 11 repeated 38 times.
        let mut expected = vec![0x40, 0x26, 0x86, 0x90];
        for _ in 0..38 {
            expected.push(0xEC);
            expected.push(0x11);
        }

        assert_eq!(encode("hi"), expected);
    }

    #[test]
    fn empty_message_is_all_padding_after_the_header() {
        let bytes = encode("");

        assert_eq!(bytes.len(), CODEWORDS);
        // mode 0100, count 00000000, terminator 0000 = 40 00, then the pads.
        assert_eq!(&bytes[..4], &[0x40, 0x00, 0xEC, 0x11]);
    }

    #[test]
    fn longest_message_leaves_no_room_for_pad_codewords() {
        let bytes = encode(&"a".repeat(78));

        assert_eq!(bytes.len(), CODEWORDS);
        // 'a' is 0x61. Shifted 4 bits by the header, the stream ends 6 1 6 1 0,
        // where the final 0 is the terminator.
        assert_eq!(&bytes[78..], &[0x16, 0x10]);
    }

    #[test]
    fn count_is_in_bytes_not_characters() {
        // 'é' is 2 bytes in UTF-8 and the crab is 4, so 2 characters but 6 bytes.
        let bytes = encode("é🦀");

        // mode 0100 then count 00000110: the first 12 bits are 0x406.
        assert_eq!(bytes[0], 0x40);
        assert_eq!(bytes[1] >> 4, 0x6);
    }

    #[test]
    #[should_panic]
    fn message_over_78_bytes_is_rejected() {
        encode(&"a".repeat(79));
    }
}
