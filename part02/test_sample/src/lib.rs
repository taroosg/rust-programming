pub fn abs(x: i32) -> u32 {
  if x > 0 {
    return x as u32;
  } else {
    return -x as u32;
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_works() {
    assert_eq!(abs(2), 2);
    assert_eq!(abs(-2), 2);
  }
}
