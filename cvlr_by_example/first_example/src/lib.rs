mod certora;

/// Example library function with a logic error.
pub fn faulty_add(x: u64, y: u64) -> u64 {
    x + y - 1
}

/// Example library function which correctly implements the addition.
pub fn correct_add(x: u64, y: u64) -> u64 {
    x + y
}
