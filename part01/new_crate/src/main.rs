// クレート名::モジュール名::関数名で呼び出す
use new_crate::module_a::func_a;

fn add(x: i32, y: i32) -> i32 {
  x + y
}

// テストコード
#[test]
fn test_add() {
  assert_eq!(0, add(0, 0));
  assert_eq!(2, add(1, 1));
  assert_eq!(3, add(2, 1));
  assert_eq!(4, add(1, 3));
}

fn main() {
  func_a();
}
