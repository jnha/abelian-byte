use std::ops::Add;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Z2u8(u8);

impl Add<Z2u8> for Z2u8 {
    type Output = Self;

    fn add(self, other: Z2u8) -> Z2u8 {
        Z2u8(self.0 ^ other.0)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Z4(u8);

impl Add<Z4> for Z4 {
    type Output = Self;

    fn add(self, other: Z4) -> Z4 {
        Z4((self.0 + other.0) & 0b11)
    }
}

#[cfg(test)]
mod tests {
    use crate::two::Z2u8;
    use crate::two::Z4;
    
    #[test]
    fn test_z2u8() {
        let one = Z2u8(1);
        assert_eq!(one + one, Z2u8(0));
    }

    #[test]
    fn test_z4() {
        let one = Z4(1);
        assert_eq!(one + one, Z4(2));
        assert_eq!(one + one + one, Z4(3));
        assert_eq!(one + one + one + one, Z4(0));
    }
}