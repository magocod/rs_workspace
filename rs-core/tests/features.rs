use rs_core::add;
use rs_core::cache::get;
use rs_core::files::read;

#[test]
fn add_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}

#[test]
fn cache_get_works() {
    let result = get();
    assert_eq!(result, 3);
}

#[test]
fn files_read_works() {
    let result = read();
    assert_eq!(result, 0);
}
