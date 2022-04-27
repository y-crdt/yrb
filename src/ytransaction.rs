use yrs::{Transaction};
use rutie::{AnyObject, Module, Object, RString, VerifiedObject, VM};
use crate::ytext::TEXT_WRAPPER;

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
