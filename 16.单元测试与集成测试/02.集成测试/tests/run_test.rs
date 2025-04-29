// 集成测试：相当于外部调用库文件
// 需要在src目录同级别目录创建tests目录，存放测试文件
use test01::add;
#[test]
fn test_add_out() {
    let res = add(1, 2);
    assert_eq!(3, res)
}
