use crate::constraints::helpers;
use crate::grid::Grid;

pub struct ActivePositionsSet {
    history: std::collections::VecDeque<Grid<bool>>,
    counter: Grid<u8>,
    max_capacity: usize,
}

impl ActivePositionsSet {
    pub fn new(max_capacity: usize) -> Self {
        Self {
            history: std::collections::VecDeque::new(),
            counter: Grid::from_default(0),
            max_capacity,
        }
    }

    pub fn push(&mut self, new_entry: Grid<bool>) {
        helpers::increment_count(&mut self.counter, &new_entry);
        self.history.push_back(new_entry);

        if self.history.len() > self.max_capacity {
            let old_entry = self.history.pop_front().unwrap();
            helpers::decrement_count(&mut self.counter, &old_entry);
        }
    }

    pub fn aggregate(&self) -> Grid<bool> {
        helpers::aggregate(&self.counter)
    }
    
    pub fn set_max_capacity(&mut self, new_capacity: usize) {
        self.max_capacity = new_capacity;
        
        while self.history.len() > self.max_capacity {
            if let Some(old_entry) = self.history.pop_front() {
                helpers::decrement_count(&mut self.counter, &old_entry);
            }
        }
    }

    pub fn max_capacity(&self) -> usize {
        self.max_capacity
    }
}
