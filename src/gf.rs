const POS_LIMIT: u16 = 255;
const XOR_NUM_256: u16 = 285;

pub struct Gf256 {
    exp: [u8; 255],
    log: [u8; 256],
}

impl Gf256 {
    /// Builds both tables
    pub fn new() -> Gf256 {
        let mut exp = [0u8; 255];
        let mut log = [0u8; 256];

        let mut amount: u16 = 1;
        for i in 0..255 {
            exp[i] = u8::try_from(amount).expect("value should fit byte after reduction");
            log[usize::from(amount)] = u8::try_from(i).expect("position should fit in a byte");

            amount = amount << 1;
            if amount > 255 {
                amount = amount ^ XOR_NUM_256;
            }
        }

        Gf256 { exp, log }
    }

    /// Multiplies 2 bytes in GF(256)
    pub fn mul(&self, a: u8, b: u8) -> u8 {
        if a == 0 || b == 0 {
            return 0;
        }

        let a_pos = self.log[usize::from(a)];
        let b_pos = self.log[usize::from(b)];

        let mut res_pos: u16 = u16::from(a_pos) + u16::from(b_pos);
        if res_pos >= POS_LIMIT {
            res_pos = res_pos - POS_LIMIT;
        }

        return self.exp[usize::from(res_pos)];
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn all_values_appear_only_once_1_to_255() {
        let gf = Gf256::new();
        let distinct: HashSet<u8> = gf.exp.iter().copied().collect();
        assert_eq!(distinct.len(), 255);
        assert!(!gf.exp.contains(&0));
    }

    #[test]
    fn a_mul_b_equals_b_mul_a_every_pair() {
        let gf = Gf256::new();
        for i in 0..=255 {
            for j in 0..=255 {
                assert_eq!(gf.mul(i, j), gf.mul(j, i));
            }
        }
    }

    #[test]
    fn known_checkpoints_match() {
        let gf = Gf256::new();

        assert_eq!(gf.mul(128, 2), 29);
        assert_eq!(gf.mul(17, 236), 138);
        assert_eq!(gf.mul(255, 255), 226);
    }

    #[test]
    fn a_times_1_is_a_and_a_time_0_is_0() {
        let gf = Gf256::new();

        for a in 0..=255 {
            assert_eq!(gf.mul(a, 1), a);
            assert_eq!(gf.mul(a, 0), 0);
        }
    }

    #[test]
    fn first_13_match_known_set() {
        let gf = Gf256::new();
        let first_13: [u8; 13] = [1, 2, 4, 8, 16, 32, 64, 128, 29, 58, 116, 232, 205];
        assert_eq!(&gf.exp[..13], &first_13);
    }
}
