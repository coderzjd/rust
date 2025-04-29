use test01::add;
#[test]
fn test_add_out() {
    let res = add(1, 2);
    assert_eq!(3, res)
}
