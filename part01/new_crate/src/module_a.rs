// module_bディレクトリのmodule_cのfunc_cを呼び出す
use crate::module_b::module_c::func_c;

pub fn func_a() {
  func_c();
}
