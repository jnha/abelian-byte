use std::ops::Add;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Z3(u8);

impl Add<Z3> for Z3 {
    type Output = Self;

    fn add(self, other: Z3) -> Z3 {
        Z3((self.0 + other.0) % 3)
    }
}

#[cfg(test)]
mod tests {
    use crate::cyclic::Z3;
    
    #[test]
    fn test_z3() {
        let one = Z3(1);
        assert_eq!(one + one, Z3(2));
        assert_eq!(one + one + one, Z3(0));
    }

}