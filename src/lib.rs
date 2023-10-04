#![warn(clippy::pedantic)]

pub mod raii;

/// Example of a doctest:
///
/// ```
/// use rs_scratch::add;
/// assert_eq!(add(4, 4), 8);
/// ```
#[must_use]
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
