//! Implements basic arithmetic operations in GF(2^8).

/// Multiply two elements of GF(2^8).
pub fn mul(e: u8, a: u8) -> u8 {
    if e == 0 || a == 0 {
        0
    } else {
        let l0 = subtle::constant_log(e as usize);
        let l1 = subtle::constant_log(a as usize);
        let v = (l0 + l1).wrapping_rem_euclid(255) as usize;
        subtle::constant_exp(v)
    }
}

/// Divide one element of GF(2^8) by another.
pub fn div(e: u8, a: u8) -> u8 {
    if a == 0 {
        panic!("Divide by zero: {} / {}", e, a)
    }

    if e == 0 {
        return 0;
    }

    let l0 = subtle::constant_log(e as usize);
    let l1 = subtle::constant_log(a as usize);
    subtle::constant_exp((l0 - l1).wrapping_rem_euclid(255) as usize)
}

// Don't touch the stuff inside here.
mod subtle {
    pub fn constant_log(n: usize) -> isize {
        let mut x: isize = 0;
        for (i, &v) in LOG.iter().enumerate() {
            let z = (i + 1) ^ (n + 1);
            let m = 1isize.wrapping_shr(z.count_zeros());
            x = x.wrapping_add(v * m);
        }
        x
    }

    pub fn constant_exp(n: usize) -> u8 {
        let mut x: u8 = 0;
        for (i, &v) in EXP.iter().enumerate() {
            let z = (i + 1) ^ (n + 1);
            let m = 1usize.wrapping_shr(z.count_zeros());
            x = x.wrapping_add(v * m as u8);
        }
        x
    }

    // 0x11b prime polynomial and 0x03 as generator
    static EXP: [u8; 256] = [
        0x01, 0x03, 0x05, 0x0f, 0x11, 0x33, 0x55, 0xff, 0x1a, 0x2e, 0x72, 0x96, 0xa1, 0xf8, 0x13,
        0x35, 0x5f, 0xe1, 0x38, 0x48, 0xd8, 0x73, 0x95, 0xa4, 0xf7, 0x02, 0x06, 0x0a, 0x1e, 0x22,
        0x66, 0xaa, 0xe5, 0x34, 0x5c, 0xe4, 0x37, 0x59, 0xeb, 0x26, 0x6a, 0xbe, 0xd9, 0x70, 0x90,
        0xab, 0xe6, 0x31, 0x53, 0xf5, 0x04, 0x0c, 0x14, 0x3c, 0x44, 0xcc, 0x4f, 0xd1, 0x68, 0xb8,
        0xd3, 0x6e, 0xb2, 0xcd, 0x4c, 0xd4, 0x67, 0xa9, 0xe0, 0x3b, 0x4d, 0xd7, 0x62, 0xa6, 0xf1,
        0x08, 0x18, 0x28, 0x78, 0x88, 0x83, 0x9e, 0xb9, 0xd0, 0x6b, 0xbd, 0xdc, 0x7f, 0x81, 0x98,
        0xb3, 0xce, 0x49, 0xdb, 0x76, 0x9a, 0xb5, 0xc4, 0x57, 0xf9, 0x10, 0x30, 0x50, 0xf0, 0x0b,
        0x1d, 0x27, 0x69, 0xbb, 0xd6, 0x61, 0xa3, 0xfe, 0x19, 0x2b, 0x7d, 0x87, 0x92, 0xad, 0xec,
        0x2f, 0x71, 0x93, 0xae, 0xe9, 0x20, 0x60, 0xa0, 0xfb, 0x16, 0x3a, 0x4e, 0xd2, 0x6d, 0xb7,
        0xc2, 0x5d, 0xe7, 0x32, 0x56, 0xfa, 0x15, 0x3f, 0x41, 0xc3, 0x5e, 0xe2, 0x3d, 0x47, 0xc9,
        0x40, 0xc0, 0x5b, 0xed, 0x2c, 0x74, 0x9c, 0xbf, 0xda, 0x75, 0x9f, 0xba, 0xd5, 0x64, 0xac,
        0xef, 0x2a, 0x7e, 0x82, 0x9d, 0xbc, 0xdf, 0x7a, 0x8e, 0x89, 0x80, 0x9b, 0xb6, 0xc1, 0x58,
        0xe8, 0x23, 0x65, 0xaf, 0xea, 0x25, 0x6f, 0xb1, 0xc8, 0x43, 0xc5, 0x54, 0xfc, 0x1f, 0x21,
        0x63, 0xa5, 0xf4, 0x07, 0x09, 0x1b, 0x2d, 0x77, 0x99, 0xb0, 0xcb, 0x46, 0xca, 0x45, 0xcf,
        0x4a, 0xde, 0x79, 0x8b, 0x86, 0x91, 0xa8, 0xe3, 0x3e, 0x42, 0xc6, 0x51, 0xf3, 0x0e, 0x12,
        0x36, 0x5a, 0xee, 0x29, 0x7b, 0x8d, 0x8c, 0x8f, 0x8a, 0x85, 0x94, 0xa7, 0xf2, 0x0d, 0x17,
        0x39, 0x4b, 0xdd, 0x7c, 0x84, 0x97, 0xa2, 0xfd, 0x1c, 0x24, 0x6c, 0xb4, 0xc7, 0x52, 0xf6,
        0x01,
    ];

    static LOG: [isize; 256] = [
        0, 0, 25, 1, 50, 2, 26, 198, 75, 199, 27, 104, 51, 238, 223, 3, 100, 4, 224, 14, 52, 141,
        129, 239, 76, 113, 8, 200, 248, 105, 28, 193, 125, 194, 29, 181, 249, 185, 39, 106, 77,
        228, 166, 114, 154, 201, 9, 120, 101, 47, 138, 5, 33, 15, 225, 36, 18, 240, 130, 69, 53,
        147, 218, 142, 150, 143, 219, 189, 54, 208, 206, 148, 19, 92, 210, 241, 64, 70, 131, 56,
        102, 221, 253, 48, 191, 6, 139, 98, 179, 37, 226, 152, 34, 136, 145, 16, 126, 110, 72, 195,
        163, 182, 30, 66, 58, 107, 40, 84, 250, 133, 61, 186, 43, 121, 10, 21, 155, 159, 94, 202,
        78, 212, 172, 229, 243, 115, 167, 87, 175, 88, 168, 80, 244, 234, 214, 116, 79, 174, 233,
        213, 231, 230, 173, 232, 44, 215, 117, 122, 235, 22, 11, 245, 89, 203, 95, 176, 156, 169,
        81, 160, 127, 12, 246, 111, 23, 196, 73, 236, 216, 67, 31, 45, 164, 118, 123, 183, 204,
        187, 62, 90, 251, 96, 177, 134, 59, 82, 161, 108, 170, 85, 41, 157, 151, 178, 135, 144, 97,
        190, 220, 252, 188, 149, 207, 205, 55, 63, 91, 209, 83, 57, 132, 60, 65, 162, 109, 71, 20,
        42, 158, 93, 86, 242, 211, 171, 68, 17, 146, 217, 35, 32, 46, 137, 180, 124, 184, 38, 119,
        153, 227, 165, 103, 74, 237, 222, 197, 49, 254, 24, 13, 99, 140, 128, 192, 247, 112, 7,
    ];
}

#[cfg(test)]
mod test {
    extern crate proptest;
    use self::proptest::prelude::*;
    use super::*;

    proptest! {
        #[test]
        fn div_is_inverse_of_mul(a in 0u8..=255, b in 1u8..=255) {
            assert_eq!(mul(div(a, b), b), a);
        }

        #[test]
        fn mul_is_inverse_of_div(a in 0u8..=255, b in 1u8..=255) {
            assert_eq!(div(mul(a, b), b), a);
        }

        #[test]
        fn mul_is_commutative(a in 0u8..=255, b in 0u8..=255) {
            assert_eq!(mul(a, b), mul(b, a));
        }
    }

    #[test]
    fn test_mul() {
        assert_eq!(mul(90, 21), 254);
        assert_eq!(mul(133, 5), 167);
    }

    #[test]
    fn test_mul_zero() {
        assert_eq!(mul(0, 21), 0);
    }

    #[test]
    fn test_div() {
        assert_eq!(div(90, 21), 189);
        assert_eq!(div(6, 55), 151);
        assert_eq!(div(22, 192), 138);
    }

    #[test]
    fn test_div_zero() {
        assert_eq!(div(0, 21), 0);
    }
}
