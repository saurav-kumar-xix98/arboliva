use crate::constants::GRID_SIZE;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CandidateCell {
    FixedValue(usize),
    ValidCandidates([bool; GRID_SIZE]),
}

impl CandidateCell {
    pub fn new() -> Self {
        Self::ValidCandidates([true; GRID_SIZE])
    }

    pub fn contains(&self, candidate: usize) -> bool {
        match self {
            Self::FixedValue(fixed_value) => fixed_value == &candidate,
            Self::ValidCandidates(valid_candidates) => valid_candidates[candidate - 1],
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::FixedValue(_) => 1,
            Self::ValidCandidates(valid_candidates) => valid_candidates.iter().filter(|c| **c).count(),
        }
    }

    pub fn remove(&mut self, candidate: usize) {
        match self {
            Self::FixedValue(_) => panic!("Cannot remove fixed"),
            Self::ValidCandidates(valid_candidates) => {
                valid_candidates[candidate - 1] = false;
                let count = valid_candidates.iter().filter(|&&c| c).count();
                if count == 1 {
                    for i in 0..GRID_SIZE {
                        if valid_candidates[i] {
                            *self = Self::FixedValue(i + 1);
                            return;
                        }
                    }
                    panic!("Shouldn't happen");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_cell_has_all_candidates() {
        let cell = CandidateCell::new();

        if let CandidateCell::ValidCandidates(bits) = cell {
            for i in 1..=GRID_SIZE {
                assert!(bits[i - 1]);
            }
        } else {
            panic!("Expected ValidCandidates");
        }
    }

    #[test]
    fn contains_works_for_valid_candidates() {
        let cell = CandidateCell::new();

        assert!(cell.contains(1));
        assert!(cell.contains(GRID_SIZE));
        assert!(cell.contains(GRID_SIZE / 2));
    }

    #[test]
    fn remove_eliminates_candidate() {
        let mut cell = CandidateCell::new();

        cell.remove(3);
        assert!(!cell.contains(3));

        // other values should still exist
        assert!(cell.contains(2));
        assert!(cell.contains(4));
    }

    #[test]
    fn len_matches_number_of_bits_set() {
        let mut cell = CandidateCell::new();

        let initial_len = cell.len();
        assert_eq!(initial_len, GRID_SIZE);

        cell.remove(1);
        assert_eq!(cell.len(), GRID_SIZE - 1);

        cell.remove(2);
        assert_eq!(cell.len(), GRID_SIZE - 2);
    }

    #[test]
    fn fixed_value_behaves_correctly() {
        let cell = CandidateCell::FixedValue(5);

        assert!(cell.contains(5));
        assert!(!cell.contains(4));
        assert!(!cell.contains(6));

        assert_eq!(cell.len(), 1);
    }

    #[test]
    #[should_panic(expected = "Cannot remove fixed")]
    fn removing_fixed_value_panics() {
        let mut cell = CandidateCell::FixedValue(1);
        cell.remove(1);
    }

    #[test]
    fn removing_same_candidate_twice_is_safe() {
        let mut cell = CandidateCell::new();

        cell.remove(1);
        let len_after_first = cell.len();

        cell.remove(1); // should not panic
        assert_eq!(cell.len(), len_after_first);
    }
}
