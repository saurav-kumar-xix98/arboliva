use crate::constants::GRID_SIZE;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CandidateCell {
    Fixed(u8),
    Candidates(u16),
}

impl CandidateCell {
    pub fn new() -> Self {
        Self::Candidates((1 << GRID_SIZE) - 1)
    }

    pub fn is_fixed(&self) -> bool {
        matches!(self, CandidateCell::Fixed(_))
    }

    pub fn is_valid(&self, candidate: u8) -> bool {
        match self {
            Self::Fixed(value) => *value == candidate,
            Self::Candidates(mask) => (mask & (1 << (candidate - 1))) != 0,
        }
    }

    pub fn count_candidates(&self) -> u8 {
        match self {
            Self::Fixed(_) => 1,
            Self::Candidates(mask) => mask.count_ones() as u8,
        }
    }

    pub fn fix_value(&mut self, value: u8) {
        *self = Self::Fixed(value);
    }

    pub fn remove_candidate(&mut self, candidate: u8) {
        match self {
            Self::Fixed(_) => return,
            Self::Candidates(mask) => {
                *mask &= !(1 << (candidate - 1));

                if mask.count_ones() == 1 {
                    let val = mask.trailing_zeros() as u8 + 1;
                    *self = Self::Fixed(val);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let cell = CandidateCell::new();
        assert!(!cell.is_fixed());
        assert_eq!(cell.count_candidates(), GRID_SIZE);
    }

    #[test]
    fn test_fixed() {
        let cell = CandidateCell::Fixed(5);
        assert!(cell.is_fixed());
        assert!(cell.is_valid(5));
    }
}
