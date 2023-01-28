pub fn get() -> u8 {
    3
}

pub fn remove() -> u8 {
    4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_works() {
        let result = get();
        assert_eq!(result, 3);
    }

    #[test]
    fn add_works() {
        let result = remove();
        assert_eq!(result, 4);
    }
}
