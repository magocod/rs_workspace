#[cfg(feature = "rs-macro")]
use rs_macro::add;

pub fn add_conditional(left: usize, right: usize) -> usize {
    #[cfg(feature = "rs-macro")]
    let v = add!(left, right, 1);
    #[cfg(not(feature = "rs-macro"))]
    let v = left + right;
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "rs-macro")]
    fn add_conditional_with_optional_dependency() {
        let result = add_conditional(2, 2);
        assert_eq!(result, 5);
    }

    #[test]
    #[cfg(not(feature = "rs-macro"))]
    fn add_conditional_without_optional_dependency() {
        let result = add_conditional(2, 2);
        assert_eq!(result, 4);
    }
}
