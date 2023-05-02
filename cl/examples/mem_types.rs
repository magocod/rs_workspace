use cl3::types::cl_int;
use std::cmp::Ordering;

const LIST_SIZE: u8 = u8::MAX;

pub type ClArray = [cl_int; LIST_SIZE as usize];

fn to_array(s: &[u8]) -> ClArray {
    let mut temp: ClArray = [0; LIST_SIZE as usize];

    for (i, v) in s.iter().enumerate() {
        // println!("{i}");
        temp[i] = *v as cl_int;
    }

    temp
}

fn value_nans_last(a: &f64, b: &f64) -> Ordering {
    match (a, b) {
        (x, y) if x.is_nan() && y.is_nan() => Ordering::Equal,
        (x, _) if x.is_nan() => Ordering::Greater,
        (_, y) if y.is_nan() => Ordering::Less,
        (_, _) => a.partial_cmp(b).unwrap(),
    }
}

fn main() {
    let h = String::from("Hello");
    let h_b = h.as_bytes();
    println!("h_b {h_b:?}");

    let w = String::from("world");
    let w_b = w.as_bytes();
    println!("w_b {w_b:?}");

    println!(
        "h_b to string {}",
        String::from_utf8(h_b.to_vec()).expect("yeah")
    );
    println!(
        "w_b to string {}",
        String::from_utf8(w_b.to_vec()).expect("yeah")
    );

    let hello = String::from("Hello, world!");
    let hello_bytes = hello.as_bytes();

    let arr = [72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33];
    let v = arr.to_vec();

    let hello_from_arr = String::from_utf8(v.clone()).expect("yeah");

    let v_to_arr = to_array(v.as_slice());
    println!("v_to_arr {:?}", v_to_arr);
    let v_clean: Vec<&i32> = v_to_arr.iter().filter(|&x| *x > 0).collect();
    println!("v_clean {:?}", v_clean);

    println!("string {hello}");
    println!("bytes {hello_bytes:?}");
    println!("from bytes {hello_from_arr}");

    let b = false;
    let b_bytes = b as u8;
    println!("bool string {b}");
    println!("b_bytes string {b_bytes}");

    // let x_coordinates: [i32; 4] = [1, 2, 3, 4];
    //
    // let x_coordinates_index = [1 + 4, 2 + 2, 3 + 1, 4 + 3];
    // let y_coordinates: [i32; 4] = [4, 2, 1, 3];
    // let mut vec: Vec<(&i32, &i32)> = vec![];
    //
    // for (x, y) in x_coordinates.iter().zip(y_coordinates.iter()) {
    //     println!("x{x} y{y}");
    //     vec.push((x, y));
    // }
    //
    // println!("---");
    //
    // for (x, y) in x_coordinates_index.iter().zip(y_coordinates.iter()) {
    //     println!("x{} y{}", x - y, y);
    // }

    // vec.sort_by(|a, b| a.1.cmp(b.1));
    //
    // for v  in vec {
    //     println!("x{} y{}", v.0, v.1);
    // }
    //
    // let mut v = 1.0;
    // v = v + 0.255;
    //
    // let vv = 0.244;
    // let vvv = 0.856;
    //
    // println!("{v}");
    // println!("b {}", v > vv);
    // println!("b {}", v > vvv);
    //
    //
    // let arr = [1.1, 2.3, 3.2, 4.4];
    // // arr.sort();
    //
    // let mut vec = arr.to_vec();
    //
    // println!("original");
    // for a  in vec.clone() {
    //     println!("v{}", a);
    // }
    //
    // println!("sorted");
    // vec.sort_by(value_nans_last);
    //
    // for a  in vec {
    //     println!("v{}", a);
    // }
    //
    // let index = 0;
    // let value = 254;
    //
    // let result = format!("{index}.{value}");
    // println!("{result}");
    // println!("{}", result.parse::<f64>().unwrap());
}
