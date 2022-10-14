//! # My Crate
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds two numbers given.
/// # Example
/// ```
/// let five = 5;
/// assert_eq!(6, my_crate::add(1,4));
/// ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
