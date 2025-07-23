/*
    Unit tests are small and more focused, testing one module in isolation at a time, and can test private interfaces.
    Integration tests are entirely external to your library and use your code
    in the same way any other external code would, using only the public interface
    and potentially exercising multiple modules per test.
 */
pub fn add_two(left: u64, right: u64) -> u64 {
    internal_adder(left, right)
}

// internal function
fn internal_adder(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    // testing func internal
    #[test]
    fn internal() {
        let result = internal_adder(3, 4);
        assert_eq!(result, 7);
    }
}