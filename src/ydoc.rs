use yrs::{Doc};
use rutie::{AnyObject, Module, Object};
use crate::ytransaction::TRANSACTION_WRAPPER;

wrappable_struct!(Doc, DocWrapper, DOC_WRAPPER);
class!(YDoc);

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
