// 声明被测试的外部 crate，就像其他使用该 crate 的程序要声明的那样。
extern crate adder;

#[test]
fn test_add() {
    assert_eq!(adder::add(3, 2), 5);
}
