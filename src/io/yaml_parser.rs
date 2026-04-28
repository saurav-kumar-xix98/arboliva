use crate::model::clue::{Direction, KillerCage, LittleKillerArrow, Thermometer};
use crate::model::{Clue, ClueType, Grid, Position, Puzzle, PuzzleGrid, RegionShape, Rule};
use serde_yaml::{Mapping, Sequence, Value};
use std::collections::{HashMap, HashSet};
use std::fs;

pub fn load_puzzle(path: &str) -> Result<Puzzle, String> {
    let yaml_str = fs::read_to_string(path)
        .map_err(|e| format!("Error reading {}: {}", path, e))?;
    let yaml_value: Value = serde_yaml::from_str(&yaml_str)
        .map_err(|e| format!("Error parsing YAML: {}", e))?;
    Ok(to_puzzle(&yaml_value)?)
}

fn to_puzzle(value: &Value) -> Result<Puzzle, String> {
    Ok(Puzzle {
        puzzle_grid: to_puzzle_grid(get(value, "grid")?)?,
        rules: to_rules(get(value, "rules")?)?,
        clues: to_clues(get(value, "clues")?)?,
    })
}

fn to_puzzle_grid(value: &Value) -> Result<PuzzleGrid, String> {
    let region_rows = to_u8(get(value, "region_rows")?)?;
    let region_cols = to_u8(get(value, "region_cols")?)?;

    let mut grid = Grid::from_default(
        RegionShape { region_rows, region_cols },
        None,
    );

    for cell in to_sequence(get(value, "cells")?)? {
        let pos = to_position(get(cell, "position")?)?;
        let val = to_u8(get(cell, "value")?)?;

        if pos.row >= grid.size() || pos.col >= grid.size() {
            return Err(format!("invalid position {}", pos));
        }

        if val == 0 || val > grid.size() {
            return Err(format!("invalid value {} at {}", val, pos));
        }

        if grid[pos].is_some() {
            return Err(format!("duplicate value at {}", pos));
        }

        grid[pos] = Some(val);
    }

    Ok(grid)
}

fn to_rules(value: &Value) -> Result<HashSet<Rule>, String> {
    to_sequence(value)?
        .iter()
        .map(|val| match to_str(val)? {
            "anti_knight" => Ok(Rule::AntiKnight),
            "killer" => Ok(Rule::Killer),
            "little_killer" => Ok(Rule::LittleKiller),
            "thermometer" => Ok(Rule::Thermometer),
            other => Err(format!("unknown rule: {}", other)),
        })
        .collect()
}

fn to_clues(value: &Value) -> Result<HashMap<ClueType, Vec<Clue>>, String> {
    to_mapping(value)?
        .iter()
        .map(|(key, val)| {
            let clue_type = to_clue_type(key)?;
            let clues = to_vec_clue(val, &clue_type)?;
            Ok((clue_type, clues))
        })
        .collect()
}

fn to_killer_cage(value: &Value) -> Result<KillerCage, String> {
    Ok(KillerCage {
        target_sum: to_u16(get(value, "target_sum")?)?,
        cage_cells: to_vec_position(get(value, "cage_cells")?)?,
    })
}

fn to_little_killer_arrow(value: &Value) -> Result<LittleKillerArrow, String> {
    Ok(LittleKillerArrow {
        target_sum: to_u16(get(value, "target_sum")?)?,
        first_cell: to_position(get(value, "first_cell")?)?,
        direction: to_direction(get(value, "direction")?)?,
    })
}

fn to_thermometer(value: &Value) -> Result<Thermometer, String> {
    Ok(Thermometer {
        thermometer_cells: to_vec_position(get(value, "thermometer_cells")?)?,
    })
}

fn to_vec_position(value: &Value) -> Result<Vec<Position>, String> {
    to_sequence(value)?
        .iter()
        .map(to_position)
        .collect()
}

fn to_clue_type(value: &Value) -> Result<ClueType, String> {
    match to_str(value)? {
        "killer_cage" => Ok(ClueType::KillerCage),
        "little_killer_arrow" => Ok(ClueType::LittleKillerArrow),
        "thermometer" => Ok(ClueType::Thermometer),
        other => Err(format!("unknown clue type: {}", other)),
    }
}

fn to_vec_clue(value: &Value, clue_type: &ClueType) -> Result<Vec<Clue>, String> {
    to_sequence(value)?.iter()
        .map(|clue_yaml| match clue_type {
            ClueType::KillerCage => Ok(Clue::KillerCage(to_killer_cage(clue_yaml)?)),
            ClueType::LittleKillerArrow => Ok(Clue::LittleKillerArrow(to_little_killer_arrow(clue_yaml)?)),
            ClueType::Thermometer => Ok(Clue::Thermometer(to_thermometer(clue_yaml)?)),
        })
        .collect()
}

fn to_direction(value: &Value) -> Result<Direction, String> {
    match to_str(value)? {
        "down_right" => Ok(Direction::DownRight),
        "down_left" => Ok(Direction::DownLeft),
        "up_right" => Ok(Direction::UpRight),
        "up_left" => Ok(Direction::UpLeft),
        other => Err(format!("unknown direction: {}", other)),
    }
}

fn to_position(value: &Value) -> Result<Position, String> {
    let seq = to_sequence(value)?;
    if seq.len() != 2 {
        return Err(format!("position must have exactly two values, got {}", seq.len()));
    }

    Ok(Position {
        row: to_u8(&seq[0])?.checked_sub(1).ok_or("row underflow")?,
        col: to_u8(&seq[1])?.checked_sub(1).ok_or("col underflow")?,
    })
}

/* ---------- Helpers ---------- */

fn get<'a>(value: &'a Value, key: &str) -> Result<&'a Value, String> {
    value.get(key).ok_or_else(|| format!("missing key '{}'", key))
}

fn to_u8(value: &Value) -> Result<u8, String> {
    u8::try_from(to_u64(value)?).map_err(|_| "u8 overflow".to_string())
}

fn to_u16(value: &Value) -> Result<u16, String> {
    u16::try_from(to_u64(value)?).map_err(|_| "u16 overflow".to_string())
}

fn to_u64(value: &Value) -> Result<u64, String> {
    value.as_u64().ok_or_else(|| format!("invalid number: {:?}", value))
}

fn to_str(value: &Value) -> Result<&str, String> {
    value.as_str().ok_or_else(|| format!("invalid string: {:?}", value))
}

fn to_sequence(value: &Value) -> Result<&Sequence, String> {
    value.as_sequence().ok_or_else(|| format!("invalid sequence: {:?}", value))
}

fn to_mapping(value: &Value) -> Result<&Mapping, String> {
    value.as_mapping().ok_or_else(|| format!("invalid map: {:?}", value))
}
