use crate::grid::Grid;

pub struct ActivePositionsSet {
    counter: Grid<usize>,
    history: std::collections::VecDeque<Grid<bool>>,
    max_capacity: usize,
}

impl ActivePositionsSet {
    pub fn new(max_capacity: usize, initial_entry: Grid<bool>) -> Self {
        let counter = initial_entry.map(|cell| *cell as usize);
        let mut history = std::collections::VecDeque::new();
        history.push_back(initial_entry);
        Self {
            counter,
            history,
            max_capacity,
        }
    }

    pub fn push(&mut self, new_entry: Grid<bool>) {
        (&mut self.counter).zip_apply(&new_entry, |count, &active| { *count += active as usize; });
        self.history.push_back(new_entry);

        if self.history.len() > self.max_capacity {
            let old_entry = self.history.pop_front().unwrap();
            (&mut self.counter).zip_apply(&old_entry, |count, &active| { *count -= active as usize; });
        }
    }

    pub fn aggregate(&self) -> Grid<bool> {
        (&self.counter).map(|&cell| cell > 0)
    }

    pub fn set_max_capacity(&mut self, new_capacity: usize) {
        self.max_capacity = new_capacity;

        while self.history.len() > self.max_capacity {
            let old_entry = self.history.pop_front().unwrap();
            (&mut self.counter).zip_apply(&old_entry, |count, &active| { *count -= active as usize; });
        }
    }

    pub fn max_capacity(&self) -> usize {
        self.max_capacity
    }
}
