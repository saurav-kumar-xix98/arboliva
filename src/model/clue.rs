use crate::model::Position;

pub struct KillerCage {
    pub target_sum: u16,
    pub cage_cells: Vec<Position>,
}

pub enum Direction {
    DownRight,
    DownLeft,
    UpRight,
    UpLeft,
}

pub struct LittleKillerArrow {
    pub target_sum: u16,
    pub first_cell: Position,
    pub direction: Direction,
}

pub struct Thermometer {
    pub thermometer_cells: Vec<Position>,
}

pub enum Clue {
    KillerCage(KillerCage),
    LittleKillerArrow(LittleKillerArrow),
    Thermometer(Thermometer),
}
