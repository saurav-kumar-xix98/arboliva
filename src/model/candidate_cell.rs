#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandidateCell {
    bitmask: u64,
}

impl CandidateCell {
    pub fn with_count(count: u8) -> Self {
        Self {
            bitmask: (1u64 << count) - 1,
        }
    }

    pub fn with_value(value: u8) -> Self {
        Self {
            bitmask: Self::mask_for(value),
        }
    }

    pub fn contains(&self, value: u8) -> bool {
        self.bitmask & Self::mask_for(value) != 0
    }

    pub fn len(&self) -> u8 {
        self.bitmask.count_ones() as u8
    }

    pub fn remove(&mut self, value: u8) {
        if !self.contains(value) {
            return;
        }
        self.bitmask ^= Self::mask_for(value);
    }

    pub fn fixed_value(&self) -> Option<u8> {
        if self.len() != 1 {
            return None;
        }
        return Some((self.bitmask.trailing_zeros() + 1) as u8);
    }

    fn mask_for(value: u8) -> u64 {
        1 << (value - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_count_sets_correct_low_bits() {
        let c = CandidateCell::with_count(3);

        assert!(c.contains(1));
        assert!(c.contains(2));
        assert!(c.contains(3));

        assert!(!c.contains(4));
        assert_eq!(c.len(), 3);
    }

    #[test]
    fn fixed_sets_single_value() {
        let c = CandidateCell::with_value(5);

        for i in 1..=16 {
            if i == 5 {
                assert!(c.contains(i));
            } else {
                assert!(!c.contains(i));
            }
        }

        assert_eq!(c.len(), 1);
    }

    #[test]
    fn remove_candidate_reduces_size() {
        let mut c = CandidateCell::with_count(5);

        assert_eq!(c.len(), 5);

        c.remove(3);

        assert!(!c.contains(3));
        assert_eq!(c.len(), 4);
    }

    #[test]
    fn remove_nonexistent_candidate_does_nothing() {
        let mut c = CandidateCell::with_count(3);

        c.remove(10); // should not panic or change anything

        assert_eq!(c.len(), 3);
    }

    #[test]
    fn multiple_removes_work_correctly() {
        let mut c = CandidateCell::with_count(6);

        c.remove(1);
        c.remove(2);
        c.remove(6);

        assert_eq!(c.len(), 3);
        assert!(!c.contains(1));
        assert!(!c.contains(2));
        assert!(!c.contains(6));
        assert!(c.contains(3));
    }
}
