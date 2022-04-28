use yrs::{Transaction, Update};
use yrs::updates::decoder::Decode;
use rutie::{AnyObject, Array, Module, NilClass, Object, RString, VerifiedObject, VM};
use crate::util::convert_array_to_vecu8;
use crate::ytext::TEXT_WRAPPER;

wrappable_struct!(Transaction, TransactionWrapper, TRANSACTION_WRAPPER);
class!(YTransaction);

impl VerifiedObject for YTransaction {
  fn is_correct_type<T: Object>(object: &T) -> bool {
    object.class() == Module::from_existing("Y")
      .get_nested_class("Transaction")
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

  fn ytransaction_commit() -> NilClass {
    let transaction = rtself.get_data_mut(&*TRANSACTION_WRAPPER);
    transaction.commit();

    NilClass::new()
  }

  fn ytransaction_apply_update(update: Array) -> NilClass {
    let u = convert_array_to_vecu8(update.unwrap());

    let transaction = rtself.get_data_mut(&*TRANSACTION_WRAPPER);
    transaction.apply_update(Update::decode_v1(u.as_slice()));

    NilClass::new()
  }
);
