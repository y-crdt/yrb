mod ydoc;
mod ytext;
mod ytransaction;

#[macro_use]
extern crate rutie;

use rutie::{Module, Object};

module!(Y);

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_yrb() {
  Module::new("Y").define(|module| {
    module.define_nested_class("Text", None).define(|klass| {
      klass.def("push", ytext::ytext_push);
      klass.def("to_s", ytext::ytext_to_string);
    });

    module.define_nested_class("Transaction", None).define(|klass| {
      klass.def("get_text", ytransaction::ytransaction_get_text);
    });

    module.define_nested_class("Doc", None).define(|klass| {
      klass.def_self("new", ydoc::ydoc_new);
      klass.def("transact", ydoc::ydoc_transact);
    });
  });
}
