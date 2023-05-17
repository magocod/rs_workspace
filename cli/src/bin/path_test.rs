use std::fs;
use std::path::PathBuf;

fn main() {
    let v = "./tmp/ts_express_node_modules/cliui/README.md";
    let tmp_str = "./tmp";
    let vs = &v[tmp_str.len()..v.len()];
    println!("{v}");
    println!("{}", vs);

    let dir_v = format!("./tmp/vram{vs}");
    println!("{}", dir_v);
    let mut p = PathBuf::from(dir_v.as_str());
    p.pop();
    println!("{:?}", p);

    match fs::create_dir_all(p) {
        Ok(_) => {
            match fs::write(dir_v, b"content") {
                Ok(_) => {}
                Err(e) => {
                    println!("{}", e);
                }
            };
        }
        Err(e) => {
            println!("{}", e);
        }
    };
}
