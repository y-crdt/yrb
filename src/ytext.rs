use yrs::{Text};
use rutie::{NilClass, Object, RString, VM};
use crate::ytransaction::{YTransaction, TRANSACTION_WRAPPER};

wrappable_struct!(Text, TextWrapper, TEXT_WRAPPER);
class!(YText);

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
