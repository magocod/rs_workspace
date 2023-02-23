// use std::env;
use shared::environment::{Environment, MyEnum};

fn main() {
    let v = Environment::Local.as_str();

    println!("Environment to str {v}");
    println!("Environment to str {}", Environment::Production.as_str());

    let x = MyEnum::C as i32;

    let try_into = |x: i32| match x.try_into() {
        Ok(MyEnum::A) => println!("a"),
        Ok(MyEnum::B) => println!("b"),
        Ok(MyEnum::C) => println!("c"),
        Err(_) => eprintln!("unknown number"),
    };

    // match x.try_into() {
    //     Ok(MyEnum::A) => println!("a"),
    //     Ok(MyEnum::B) => println!("b"),
    //     Ok(MyEnum::C) => println!("c"),
    //     Err(_) => eprintln!("unknown number"),
    // }

    try_into(x);

    let x = 1;

    try_into(x);

    let x = 4;

    try_into(x);
}
