mod my {
    use getter_derive::Getters;

    #[derive(Debug, Getters)]
    pub struct Example {
        name: String,
    }

    impl Example {
        pub fn new() -> Example {
            Example {
                name: "example".to_string(),
            }
        }
    }

    impl Default for Example {
        fn default() -> Self {
            Self::new()
        }
    }
}

fn main() {
    let d = my::Example::default();

    println!("struct - {d:?}");
    println!("struct.name - {}", d.name());
}
