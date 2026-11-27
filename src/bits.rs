pub struct BitBuffer {
    bits: Vec<bool>, // one entry per bit, in order
}

impl BitBuffer {
    pub fn new() -> BitBuffer {
        BitBuffer { bits: vec![] }
    }

    /// Append the lowest `count` bits of `value`, most significant first.
    pub fn push_bits(&mut self, value: u32, count: usize) {
        assert!(
            count >= 32 || value >> count == 0,
            "{value} does not fit in {count} bits"
        );

        for i in (0..count).rev() {
            let val = (value >> i) & 1;
            if val == 1 {
                self.bits.push(true);
            } else {
                self.bits.push(false);
            }
        }
    }

    /// Number of bits stored.
    pub fn len(&self) -> usize {
        return self.bits.len();
    }

    /// The contents packed 8 bits at a time.
    pub fn to_bytes(&self) -> Vec<u8> {
        assert!(
            self.bits.len() % 8 == 0,
            "bit length is not a multiple of 8"
        );

        let chunks = self.bits.chunks(8);

        let mut result: Vec<u8> = vec![];

        // The bits are stored as separate bools, but Reed-Solomon needs whole bytes.
        // Each group of 8 is fed in like tape: shift the value left to open a space
        // on the right, then put the next bit in it.
        // e.g. the bits 0001 0001 build up as 0, 0, 0, 1, 2, 4, 8, 17.
        for chunk in chunks {
            let mut value: u8 = 0;
            for &bit in chunk {
                value = (value << 1) + u8::from(bit);
            }
            result.push(value);
        }

        return result;
    }

    /// The contents as text, e.g. "0100".
    pub fn to_bit_string(&self) -> String {
        return self
            .bits
            .iter()
            .map(|&f| if f { '1' } else { '0' })
            .collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn four_in_4_bits_then_2_in_8_bits() {
        let mut bb = BitBuffer::new();
        bb.push_bits(4, 4);
        bb.push_bits(2, 8);
        assert_eq!(bb.to_bit_string(), "010000000010");
        assert_eq!(bb.len(), 12);
    }

    #[test]
    fn one_packed_in_4_bits() {
        let mut bb = BitBuffer::new();
        bb.push_bits(1, 4);
        assert_eq!(bb.to_bit_string(), "0001");
        assert_eq!(bb.len(), 4);
    }
}
