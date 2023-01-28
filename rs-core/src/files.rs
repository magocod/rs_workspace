pub fn read() -> u8 {
    0
}

pub fn write() -> u8 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_works() {
        let result = read();
        assert_eq!(result, 0);
    }

    #[test]
    fn write_works() {
        let result = write();
        assert_eq!(result, 1);
    }
}
