#[derive(Clone, Debug)]
struct Puzzle {
    pub num_pieces: u32,
    pub name: String,
}

impl Default for Puzzle {
    fn default() -> Self {
        Puzzle {
            num_pieces: 10,
            name: "default".to_string(),
        }
    }
}

impl PartialEq for Puzzle {
    fn eq(&self, other: &Self) -> bool {
        (self.num_pieces == other.num_pieces)
            && (self.name.to_lowercase() == other.name.to_lowercase())
    }
}

impl From<&Puzzle> for String {
    fn from(value: &Puzzle) -> Self {
        value.name.clone()
    }
}

impl From<Puzzle> for String {
    fn from(value: Puzzle) -> Self {
        value.name
    }
}

pub fn show<T: Into<String>>(s: T) {
    println!("{}", s.into());
}

fn main() {
    let puzzle = Puzzle {
        num_pieces: 15,
        ..Default::default()
    };
    println!("{puzzle:#?}");

    show(&puzzle);
    show(puzzle);
}
