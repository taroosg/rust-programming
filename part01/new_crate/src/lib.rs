// このクレートの中でmodule_aとmodule_bが有効である旨を示す
// module_aはクレートの外部から利用可能である旨を示す
pub mod module_a;
mod module_b;

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
