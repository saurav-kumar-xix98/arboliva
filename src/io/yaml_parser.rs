use crate::model::{Grid, Position, Puzzle, RegionShape};
use crate::solver::constraints::constraint::Constraint;
use crate::solver::constraints::constraint_set::ConstraintSet;
use crate::solver::constraints::variants::thermometer::Thermometer;
use crate::solver::constraints::variants::{killer, little_killer};
use crate::solver::constraints::variants::{AntiKnightConstraint, ClassicConstraint, KillerConstraint, LittleKillerConstraint, ThermometerConstraint};
use serde_yaml::Value;
use std::error::Error;
use std::fs;

pub fn load_puzzle(path: &str) -> Result<Puzzle, Box<dyn Error>> {
    let yaml_str = fs::read_to_string(path)?;
    let yaml_value = serde_yaml::from_str(&yaml_str)?;

    let puzzle = to_puzzle(yaml_value)?;

    Ok(puzzle)
}

fn to_puzzle(value: Value) -> Result<Puzzle, String> {
    let grid = to_grid(get_required(&value, "grid")?)?;
    let constraint_set = to_constraint_set(get_required(&value, "constraint_set")?, &grid)?;

    Ok(Puzzle { grid, constraint_set })
}

fn to_grid(value: Value) -> Result<Grid<Option<u8>>, String> {
    let region_rows = to_u8(get_required(&value, "region_rows")?)?;
    let region_cols = to_u8(get_required(&value, "region_cols")?)?;

    let mut grid = Grid::from_default(RegionShape{ region_rows, region_cols }, None);

    let cells = to_vec(get_required(&value, "cells")?)?;

    for cell in cells {
        let pos = to_position(get_required(&cell, "position")?)?;
        let val = to_u8(get_required(&cell, "value")?)?;

        if pos.row >= grid.size() || pos.col >= grid.size() {
            return Err(format!("invalid position {}", pos));
        }

        if val == 0 || val > grid.size() {
            return Err(format!("invalid value {} at position {}", val, pos));
        }

        if grid[pos].is_some() {
            return Err(format!("duplicate values at position {}", pos));
        }

        grid[pos] = Some(val);
    }

    Ok(grid)
}

fn to_constraint_set(value: Value, grid: &Grid<Option<u8>>) -> Result<ConstraintSet, String> {
    let raw_constraints = to_vec(value)?;

    let constraints = raw_constraints
        .into_iter()
        .map(|c| to_constraint(&c, grid))
        .collect::<Result<Vec<_>, String>>()?;

    Ok(ConstraintSet::new(constraints))
}

fn to_constraint(value: &Value, grid: &Grid<Option<u8>>) -> Result<Box<dyn Constraint>, String> {
    let constraint_type = to_string(get_required(value, "type")?)?;

    match constraint_type.as_str() {
        "classic" => Ok(Box::new(ClassicConstraint)),
        "anti_knight" => Ok(Box::new(AntiKnightConstraint)),
        "killer" => Ok(Box::new(to_killer_constraint(value, grid)?)),
        "little_killer" => Ok(Box::new(to_little_killer_constraint(value, grid)?)),
        "thermometer" => Ok(Box::new(to_thermometer_constraint(value, grid)?)),
        other => Err(format!("unknown constraint type: {}", other)),
    }
}

fn to_killer_constraint(value: &Value, grid: &Grid<Option<u8>>) -> Result<KillerConstraint, String> {
    let cages_yaml = to_vec(get_required(value, "cages")?)?;
    let cages = cages_yaml.into_iter()
        .map(|cage_yaml| {
            let sum = to_u16(get_required(&cage_yaml, "sum")?)?;
            let positions = to_vec_position(get_required(&cage_yaml, "positions")?)?;
            Ok(killer::Cage { sum, positions })
        })
        .collect::<Result<Vec<killer::Cage>, String>>()?;

    Ok(KillerConstraint::new(cages, grid.region_rows(), grid.region_cols()))
}

fn to_little_killer_constraint(value: &Value, grid: &Grid<Option<u8>>) -> Result<LittleKillerConstraint, String> {
    let diagonals_yaml = to_vec(get_required(&value, "diagonals")?)?;
    let diagonals = diagonals_yaml.into_iter()
        .map(|diag_yaml| {
            let sum = to_u16(get_required(&diag_yaml, "sum")?)?;
            let direction = match to_string(get_required(&diag_yaml, "direction")?)?.as_str() {
                "down_right" => little_killer::Direction::DownRight,
                "down_left" => little_killer::Direction::DownLeft,
                "up_right" => little_killer::Direction::UpRight,
                "up_left" => little_killer::Direction::UpLeft,
                other => return Err(format!("invalid direction: {}", other)),
            };
            let first_position = to_position(get_required(&diag_yaml, "first_position")?)?;
            Ok(little_killer::Diagonal::new(sum, direction, first_position, grid.size()))
        })
        .collect::<Result<Vec<little_killer::Diagonal>, String>>()?;

    Ok(LittleKillerConstraint::new(diagonals, grid.region_rows(), grid.region_cols()))
}

fn to_thermometer_constraint(value: &Value, grid: &Grid<Option<u8>>) -> Result<ThermometerConstraint, String> {
    let thermometers_yaml = to_vec(get_required(value, "thermometers")?)?;
    let thermometers = thermometers_yaml.into_iter()
        .map(|thermometer_yaml| {
            let positions = to_vec_position(get_required(&thermometer_yaml, "positions")?)?;
            Ok(Thermometer { positions })
        })
        .collect::<Result<Vec<Thermometer>, String>>()?;

    Ok(ThermometerConstraint::new(thermometers, grid.region_rows(), grid.region_cols()))
}

fn to_u8(value: Value) -> Result<u8, String> {
    u8::try_from(to_u64(value)?).map_err(|_| "value out of range for u8".to_string())
}

fn to_u16(value: Value) -> Result<u16, String> {
    u16::try_from(to_u64(value)?).map_err(|_| "value out of range for u16".to_string())
}

fn to_u64(value: Value) -> Result<u64, String> {
    value.as_u64().ok_or_else(|| format!("invalid value: {:?}", value))
}

fn to_string(value: Value) -> Result<String, String> {
    value.as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| format!("invalid string value: {:?}", value))
}

fn to_vec(value: Value) -> Result<Vec<Value>, String> {
    value.as_sequence()
        .map(|v| v.clone())
        .ok_or_else(|| format!("invalid sequence value: {:?}", value))
}

fn to_position(value: Value) -> Result<Position, String> {
    let seq = to_vec(value)?;
    if seq.len() != 2 {
        return Err("Position must have exactly 2 elements".to_string());
    }

    let row = to_u8(seq[0].clone())?.checked_sub(1).ok_or_else(|| "")?;
    let col = to_u8(seq[1].clone())?.checked_sub(1).ok_or_else(|| "")?;
    Ok(Position{row, col})
}

fn to_vec_position(value: Value) -> Result<Vec<Position>, String> {
    to_vec(value)?
        .into_iter()
        .map(to_position)
        .collect::<Result<Vec<_>, _>>()
}

fn get_required(value: &Value, key: &str) -> Result<Value, String> {
    value.get(key)
        .cloned()
        .ok_or_else(|| format!("missing key '{}'", key))
}
