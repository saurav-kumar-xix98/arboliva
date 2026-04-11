use crate::constraints::active_positions_set::ActivePositionsSet;
use crate::constraints::{builder, helpers};
use crate::constraints::constraint::Constraint;
use crate::grid::{CandidateCell, Grid};

pub struct ConstraintSet {
    constraints: Vec<Box<dyn Constraint>>,
}

impl ConstraintSet {

    pub fn from_yaml_values(raw: Vec<serde_yaml::Value>) -> Result<Self, String> {
        let mut set = ConstraintSet { constraints: Vec::new() };
        for c in raw {
            let constraint = builder::to_constraint(&c)?;
            set.add_constraint(constraint);
        }
        Ok(set)
    }

    pub fn add_constraint(&mut self, constraint: Box<dyn Constraint>) {
        self.constraints.push(constraint);
    }

    pub fn update(&self,
        grid: &mut Grid<CandidateCell>,
        active_positions: Grid<bool>
    ) -> bool {

        if self.constraints.is_empty() {
            return true;
        }

        if self.constraints.len() == 1 {
            return match self.constraints[0].update_recursive(grid, active_positions) {
                Some(_) => true,
                None => false,
            };
        }

        let mut active_positions_set = ActivePositionsSet::new(self.constraints.len());
        active_positions_set.push(active_positions);

        loop {
            for constraint in &self.constraints {
                let active_positions = active_positions_set.aggregate();
                if !helpers::is_any_true(&active_positions) {
                    return true;
                }
                match constraint.update_recursive(grid, active_positions) {
                    Some(updated_positions) => active_positions_set.push(updated_positions),
                    None => return false,
                }
            }
            if active_positions_set.max_capacity() > self.constraints.len() - 1 {
                active_positions_set.set_max_capacity(self.constraints.len() - 1);
            }
        }
    }
}
