use std::collections::{HashMap, HashSet};
use crate::model::{Clue, ClueType, Grid, Position, Puzzle, RegionShape, Rule};
use serde_yaml::Value;
use std::error::Error;
use std::fs;
use crate::model::clue::{Direction, KillerCage, LittleKillerArrow, Thermometer};

pub fn load_puzzle(path: &str) -> Result<Puzzle, Box<dyn Error>> {
    let yaml_str = fs::read_to_string(path)?;
    let yaml_value = serde_yaml::from_str(&yaml_str)?;

    let puzzle = to_puzzle(&yaml_value)?;

    Ok(puzzle)
}

fn to_puzzle(value: &Value) -> Result<Puzzle, String> {
    let puzzle_grid = to_puzzle_grid(get_required(&value, "grid")?)?;
    let rules = to_rules(get_required(&value, "rules")?)?;
    let clues = to_clues(get_required(&value, "clues")?)?;

    Ok(Puzzle { puzzle_grid, rules, clues })
}

fn to_puzzle_grid(value: Value) -> Result<Grid<Option<u8>>, String> {
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

fn to_rules(value: Value) -> Result<HashSet<Rule>, String> {
    to_vec(value)?.into_iter()
        .map(|s| {
            let rule = match to_string(s)?.as_str() {
                "anti_knight" => Rule::AntiKnight,
                "killer" => Rule::Killer,
                "little_killer" => Rule::LittleKiller,
                "thermometer" => Rule::Thermometer,
                other => panic!("unknown rule: {}", other),
            };
            Ok(rule)
        }).collect()
}

fn to_clues(value: Value) -> Result<HashMap<ClueType, Vec<Clue>>, String> {
    to_map(value)?.into_iter()
        .map(|(key, value)| {
            let clue_type = match key.as_str() {
                "killer_cage" => ClueType::KillerCage,
                "little_killer_arrow" => ClueType::LittleKillerArrow,
                "thermometer" => ClueType::Thermometer,
                other => panic!("unknown clue type: {}", other),
            };
            let clues: Vec<Clue> = to_vec(value)?
                .into_iter()
                .map(|clue_yaml| {
                    let clue = match clue_type {
                        ClueType::KillerCage => {
                            Clue::KillerCage(to_killer_cage(clue_yaml)?)
                        }
                        ClueType::LittleKillerArrow => {
                            Clue::LittleKillerArrow(to_little_killer_arrow(clue_yaml)?)
                        }
                        ClueType::Thermometer => {
                            Clue::Thermometer(to_thermometer(clue_yaml)?)
                        }
                    };
                    Ok(clue)
                })
                .collect::<Result<Vec<_>, String>>()?;
            Ok((clue_type, clues))
        })
        .collect()
}

fn to_killer_cage(value: Value) -> Result<KillerCage, String> {
    let target_sum = to_u16(get_required(&value, "target_sum")?)?;
    let cage_cells = to_vec_position(get_required(&value, "cage_cells")?)?;
    Ok(KillerCage{target_sum, cage_cells})
}

fn to_little_killer_arrow(value: Value) -> Result<LittleKillerArrow, String> {
    let target_sum = to_u16(get_required(&value, "target_sum")?)?;
    let first_cell = to_position(get_required(&value, "first_cell")?)?;
    let direction = match to_string(get_required(&value, "direction")?)?.as_str() {
        "down_right" => Direction::DownRight,
        "down_left" => Direction::DownLeft,
        "up_right" => Direction::UpRight,
        "up_left" => Direction::UpLeft,
        other => panic!("unknown direction: {}", other),
    };
    Ok(LittleKillerArrow{target_sum, first_cell, direction})
}

fn to_thermometer(value: Value) -> Result<Thermometer, String> {
    let thermometer_cells = to_vec_position(get_required(&value, "thermometer_cells")?)?;
    Ok(Thermometer{thermometer_cells})
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

fn to_map(value: Value) -> Result<HashMap<String, Value>, String> {
    value.as_mapping()
        .ok_or_else(|| format!("invalid map value: {:?}", value))?
        .iter()
        .map(|(k, v)| {
            let key = k.as_str().ok_or_else(|| "")?.to_string();
            Ok((key, v.clone()))
        })
        .collect()
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
