# Arboliva — Sudoku Solver in Rust
![Arboliva](https://www.pokemon.com/static-assets/content-assets/cms2/img/pokedex/full/930.png)

Arboliva is a Sudoku solver written in Rust that supports classic Sudoku rules along with several advanced constraint types such as Killer cages, Little Killer diagonals, and Thermometer constraints. It uses constraint propagation combined with backtracking search to solve puzzles defined in YAML format.

---

## ✨ Features

- Classic Sudoku rules (row, column, and box constraints)
- Anti-Knight constraint
- Killer Sudoku cages (sum constraints)
- Little Killer diagonal sums
- Thermometer constraints
- YAML-based puzzle definitions
- Constraint propagation + recursive backtracking solver
- Extensible constraint system via `Constraint` trait

---

## 🚀 Running the Solver

The solver currently runs a fixed puzzle file defined in `src/main.rs`:

```rust
load_puzzle("puzzles/thermometer.yaml")
```

Run the project:

```bash
cargo run
```

The program will:

1. Load the puzzle from the hardcoded YAML file
2. Parse grid, rules, and clues
3. Solve using the constraint engine
4. Print either:
    - the solved Sudoku grid, or
    - "No solution found"

---

## 📥 Puzzle Format (YAML)

Puzzles are defined in YAML with three sections:

- `grid`
- `rules`
- `clues`

### Example

```yaml
grid:
  region_rows: 3
  region_cols: 3
  cells: []

rules:
  - anti_knight
  - little_killer
  - killer
  - thermometer

clues:
  little_killer_arrow:
    - target_sum: 39
      first_cell: [1, 1]
      direction: down_right

  killer_cage:
    - target_sum: 7
      cage_cells: [[3,3], [4,3]]
    - target_sum: 13
      cage_cells: [[4,5], [5,5]]

  thermometer:
    - thermometer_cells: [[1,1], [1,2], [1,3], [2,3]]
```

### Grid format

- `region_rows` / `region_cols` define the block size (e.g. 3×3 for standard Sudoku)
- `cells` defines pre-filled values:

```yaml
cells:
  - position: [1, 1]
    value: 5
```

Positions are 1-based in YAML but converted internally to 0-based indexing.

---

## 🧠 How It Works

The solver uses a multi-stage constraint system:

### 1. Grid Representation
- `PuzzleGrid`: initial puzzle state
- `CandidateGrid`: possible values per cell
- `SolutionGrid`: final resolved grid

### 2. Constraint System

Each rule implements the `Constraint` trait:

```rust
trait Constraint {
    fn update(&self, grid: &mut CandidateGrid, active_positions: Grid<bool>) -> Option<Grid<bool>>;
}
```

Constraints propagate reductions in candidate values.

Implemented constraints:
- `ClassicConstraint` (row/col/box elimination)
- `AntiKnightConstraint`
- `KillerConstraint`
- `LittleKillerConstraint`
- `ThermometerConstraint`

---

### 3. Constraint Engine

The `ConstraintSet`:
- Iterates constraints repeatedly
- Propagates changes until stable
- Detects contradictions early

---

### 4. Backtracking Search

When constraint propagation is not enough:

- Picks the cell with the fewest candidates
- Tries values recursively
- Backtracks on failure

Heuristic:

```rust
find_best_candidate(grid)
```

(minimum remaining values strategy)

---

## 📂 Project Structure

```
src/
├── io/                  # YAML parsing
├── model/               # Core data structures
│   ├── grid.rs
│   ├── puzzle.rs
│   ├── clue.rs
│   └── rule.rs
├── solver/
│   ├── solver.rs       # Main solving loop
│   └── constraints/    # Constraint engine
│       ├── variants/   # Specific rule implementations
│       └── constraint.rs
```

---

## ➕ Adding New Constraints

To add a new rule:

1. Add a variant to `Rule`
2. Create a new struct implementing `Constraint`
3. Register it in `to_constraint_set()`
4. Add YAML parsing support in `yaml_parser.rs`

---

## ⚠️ Notes

- The solver currently uses a **hardcoded puzzle path**
- Debug output is printed during solving (constraint propagation + guesses)
- Not optimized for large-scale performance yet
- YAML parsing assumes valid structure (limited validation)

---

## 🧪 Example Output

```
Guessing value 7 at Position { row: 3, col: 4 }
ClassicConstraint::update
KillerConstraint::update
...
```

---

## 📌 Future Improvements

- CLI argument for puzzle path
- Optional verbose/debug mode
- Improved candidate heuristics
- Parallel search optimization
- Stronger YAML schema validation
```
```
