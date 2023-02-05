use rs_proc_macro::make_answer;

make_answer!();

#[test]
fn news_feed() {
    // println!("{}", answer());
    assert_eq!(answer(), 42);
}
