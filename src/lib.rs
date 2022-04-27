#[macro_use]
extern crate rutie;

use rutie::{AnyObject, Class, Module, Object};

module!(Y);

class!(YTransaction);

class!(YDoc);

methods!(
    YDoc,
    _rtself,

    fn ydoc_transact() -> AnyObject {
      return Class::from_existing("YTransaction").new_instance(&[]);
    }
);

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_yrb() {
  Module::new("Y").define(|module| {
    module.define_nested_class("Transaction", None);
    module.define_nested_class("Doc", None).define(|klass| {
      klass.def("transact", ydoc_transact);
    });
  });
}
