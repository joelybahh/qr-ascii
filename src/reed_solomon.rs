use crate::gf::Gf256;

pub fn build_generator_polynomial(gf: &Gf256, n: usize) -> Vec<u8> {
    let mut result = vec![1u8];

    for i in 0..n {
        result = multiply_polynomial(gf, result, vec![1, gf.value_at_pos(i)]);
    }

    return result;
}

pub fn multiply_polynomial(gf: &Gf256, a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
    let length = a.len() + b.len() - 1;

    let mut result = vec![0u8; length];

    for i in 0..a.len() {
        for j in 0..b.len() {
            let res = gf.mul(a[i], b[j]);
            result[i + j] ^= res;
        }
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generator_for_2_codewords() {
        let gf = Gf256::new();
        assert_eq!(build_generator_polynomial(&gf, 2), [1, 3, 2]);
    }

    #[test]
    fn generator_for_7_codewords() {
        let gf = Gf256::new();
        assert_eq!(
            build_generator_polynomial(&gf, 7),
            [1, 127, 122, 154, 164, 11, 68, 117]
        );
    }

    // The one this project uses: Version 4 at level L has 20 error correction codewords.
    #[test]
    fn generator_for_20_codewords() {
        let gf = Gf256::new();
        assert_eq!(
            build_generator_polynomial(&gf, 20),
            [
                1, 152, 185, 240, 5, 111, 99, 6, 220, 112, 150, 69, 36, 187, 22, 228, 198, 121,
                121, 165, 174
            ]
        );
    }
}
