use hello::add_three;

#[test]
fn it_adds_two() {
    assert_eq!(4, add_three(2));
}
