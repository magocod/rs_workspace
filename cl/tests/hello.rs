use cl::hello;

#[test]
fn it_works() {
    let result = hello();
    assert_eq!(result, "hello");
}