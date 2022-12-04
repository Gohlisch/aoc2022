
pub struct Range (pub i32, pub i32);

impl Range{
    pub fn includes(self: &Self, other: &Self) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }
}

#[cfg(test)]
mod tests {
    use crate::Range;

    #[test]
    fn includes_is_true() {
        let a = Range(0, 5);
        let b = Range(3, 5);

        assert!(a.includes(&b));
    }

    #[test]
    fn includes_is_false() {
        let a = Range(0, 5);
        let b = Range(20, 30);

        assert!(!a.includes(&b));
    }
}