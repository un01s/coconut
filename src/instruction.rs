#[derive(Debug, PartialEq, Clone)]
pub enum Op {
    Add, // addition operation
    Mul, // multiplication operation
    Push { value: u64 }, // load numeric value onto stack
}
