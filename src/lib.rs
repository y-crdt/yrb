#[macro_use]
extern crate rutie;

use yrs::{Doc, Text, Transaction};
use rutie::{AnyObject, Module, NilClass, Object, RString, VerifiedObject, VM};

// containing module
module!(Y);

// structs
wrappable_struct!(Transaction, TransactionWrapper, TRANSACTION_WRAPPER);
class!(YTransaction);

impl VerifiedObject for YTransaction {
  fn is_correct_type<T: Object>(object: &T) -> bool {
    object.class() == Module::from_existing("Y").get_nested_class("Transaction")
  }

  fn error_message() -> &'static str {
    "Error converting to YTransaction"
  }
}

wrappable_struct!(Text, TextWrapper, TEXT_WRAPPER);
class!(YText);

wrappable_struct!(Doc, DocWrapper, DOC_WRAPPER);
class!(YDoc);

// text methods
methods!(
  YText,
  rtself,

  fn ytext_push(transaction: YTransaction, value: RString) -> NilClass {
    let value_str = value.
          map_err(|e| VM::raise_ex(e) ).
          unwrap().
          to_string();

    let mut txn = transaction.
      map_err(|e| VM::raise_ex(e) ).
      unwrap();

    let t = txn.get_data_mut(&*TRANSACTION_WRAPPER);

    let text = rtself.get_data_mut(&*TEXT_WRAPPER);
    text.push(t, &value_str);

    NilClass::new()
  }

  fn ytext_to_string() -> RString {
    let text = rtself.get_data(&*TEXT_WRAPPER);

    RString::new_utf8(&text.to_string())
  }
);

// transaction methods

methods!(
  YTransaction,
  rtself,

  fn ytransaction_get_text(name: RString) -> AnyObject {
    let name_str = name.
          map_err(|e| VM::raise_ex(e) ).
          unwrap().
          to_string();

    let transaction = rtself.get_data_mut(&*TRANSACTION_WRAPPER);
    let text = transaction.get_text(&name_str);

    Module::from_existing("Y")
      .get_nested_class("Text")
      .wrap_data(text, &*TEXT_WRAPPER)
  }
);

// doc methods

methods!(
  YDoc,
  rtself,

  fn ydoc_new() -> AnyObject {
    let doc = Doc::default();
    Module::from_existing("Y")
      .get_nested_class("Doc")
      .wrap_data(doc, &*DOC_WRAPPER)
  }

  fn ydoc_transact() -> AnyObject {
    let doc = rtself.get_data(&*DOC_WRAPPER);
    let transaction = doc.transact();

    Module::from_existing("Y")
      .get_nested_class("Transaction")
      .wrap_data(transaction, &*TRANSACTION_WRAPPER)
  }
);

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_yrb() {
  Module::new("Y").define(|module| {
    module.define_nested_class("Text", None).define(|klass| {
      klass.def("push", ytext_push);
      klass.def("to_s", ytext_to_string);
    });

    module.define_nested_class("Transaction", None).define(|klass| {
      klass.def("get_text", ytransaction_get_text);
    });

    module.define_nested_class("Doc", None).define(|klass| {
      klass.def_self("new", ydoc_new);
      klass.def("transact", ydoc_transact);
    });
  });
}
