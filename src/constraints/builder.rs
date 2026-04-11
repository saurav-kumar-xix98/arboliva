use crate::constraints::constraint::Constraint;
use crate::constraints::variants::{AntiKnightConstraint, ClassicConstraint, KillerConstraint, LittleKillerConstraint, ThermometerConstraint};
use crate::constraints::variants::{killer, little_killer, thermometer};
use crate::grid::Position;
use serde_yaml::Value;
use crate::constraints::variants::thermometer::Thermometer;

pub fn to_constraint(value: &Value) -> Result<Box<dyn Constraint>, String> {
    let constraint_type = to_string(value, "type")?;

    match constraint_type.as_str() {
        "classic" => Ok(Box::new(ClassicConstraint)),
        "anti_knight" => Ok(Box::new(AntiKnightConstraint)),
        "killer" => Ok(Box::new(to_killer_constraint(value)?)),
        "little_killer" => Ok(Box::new(to_little_killer_constraint(value)?)),
        "thermometer" => Ok(Box::new(to_thermometer_constraint(value)?)),
        other => Err(format!("unknown constraint type: {}", other)),
    }
}

fn to_killer_constraint(value: &Value) -> Result<KillerConstraint, String> {
    let cages_yaml = to_vec(value, "cages")?;
    let cages = cages_yaml.into_iter()
        .map(|cage_yaml| {
            let sum = to_u8(&cage_yaml, "sum")?;
            let positions = to_positions(&cage_yaml, "positions")?;
            Ok(killer::Cage { sum, positions })
        })
        .collect::<Result<Vec<killer::Cage>, String>>()?;

    Ok(KillerConstraint::new(cages))
}

fn to_little_killer_constraint(value: &Value) -> Result<LittleKillerConstraint, String> {
    let diagonals_yaml = to_vec(value, "diagonals")?;
    let diagonals = diagonals_yaml.into_iter()
        .map(|diag_yaml| {
            let sum = to_u8(&diag_yaml, "sum")?;
            let direction = match to_string(&diag_yaml, "direction")?.as_str() {
                "down_right" => little_killer::Direction::DownRight,
                "down_left" => little_killer::Direction::DownLeft,
                "up_right" => little_killer::Direction::UpRight,
                "up_left" => little_killer::Direction::UpLeft,
                other => return Err(format!("invalid direction: {}", other)),
            };
            let first_position = to_position(&diag_yaml, "first")?;
            Ok(little_killer::Diagonal::new(sum, direction, first_position))
        })
        .collect::<Result<Vec<little_killer::Diagonal>, String>>()?;

    Ok(LittleKillerConstraint::new(diagonals))
}

fn to_thermometer_constraint(value: &Value) -> Result<ThermometerConstraint, String> {
    let thermometers_yaml = to_vec(value, "thermometers")?;
    let thermometers = thermometers_yaml.into_iter()
        .map(|thermometer_yaml| {
            let positions = to_positions(&thermometer_yaml, "positions")?;
            Ok(thermometer::Thermometer { positions })
        })
        .collect::<Result<Vec<Thermometer>, String>>()?;

    Ok(ThermometerConstraint::new(thermometers))
}

fn to_u8(value: &Value, key: &str) -> Result<u8, String> {
    value.get(key)
        .and_then(Value::as_u64)
        .and_then(|n| u8::try_from(n).ok())
        .ok_or_else(|| format!("missing or invalid '{}'", key))
}

fn to_string(value: &Value, key: &str) -> Result<String, String> {
    value.get(key)
        .and_then(Value::as_str)
        .map(|s| s.to_string())
        .ok_or_else(|| format!("missing or invalid '{}'", key))
}

fn to_vec(value: &Value, key: &str) -> Result<Vec<Value>, String> {
    value.get(key)
        .and_then(Value::as_sequence)
        .map(|v| v.clone())
        .ok_or_else(|| format!("missing or invalid '{}'", key))
}

fn parse_position(seq: &[Value]) -> Result<Position, String> {
    if seq.len() != 2 {
        return Err("Position must have exactly 2 elements".to_string());
    }

    let row = seq[0]
        .as_u64()
        .and_then(|n| u8::try_from(n).ok())
        .ok_or_else(|| "invalid row in Position".to_string())?;

    let col = seq[1]
        .as_u64()
        .and_then(|n| u8::try_from(n).ok())
        .ok_or_else(|| "invalid col in Position".to_string())?;

    Ok(Position::new(row - 1, col - 1))
}

fn to_position(value: &Value, key: &str) -> Result<Position, String> {
    let seq = to_vec(value, key)?;
    parse_position(&seq)
}

fn to_positions(value: &Value, key: &str) -> Result<Vec<Position>, String> {
    let seq = to_vec(value, key)?;

    seq.into_iter()
        .map(|v| {
            let pos_seq = v.as_sequence()
                .ok_or_else(|| format!("each element in '{}' must be a sequence", key))?;
            parse_position(pos_seq)
        })
        .collect()
}
