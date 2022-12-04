
pub struct Range (pub i32, pub i32);

impl Range{
    pub fn includes(self: &Self, other: &Self) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }

    pub fn overlaps_with(self: &Self, other: &Self) -> bool {
        self.0 >= other.0 && self.0 <= other.1
        || self.1 >= other.0 && self.1 <= other.1
        || self.includes(other)
        || other.includes(self)
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


    #[test]
    fn overlaps_with_is_true() {
        let overlap = vec![
            (Range(5,7), Range(7,9)),
            (Range(2,8), Range(3,7)),
            (Range(6,6), Range(4,6)),
            (Range(2,6), Range(4,8)),
        ];

        for (a, b) in overlap {
            assert!(a.overlaps_with(&b));
        }
    }

    #[test]
    fn overlaps_with_is_false() {
        let overlap = vec![
            (Range(2,4), Range(6,8)),
            (Range(2,3), Range(4,5)),
        ];

        for (a, b) in overlap {
            assert!(!a.overlaps_with(&b));
        }
    }
}