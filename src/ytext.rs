use crate::ytransaction::{YTransaction, TRANSACTION_WRAPPER};
use rutie::{Fixnum, NilClass, Object, RString, VM};
use yrs::Text;

wrappable_struct!(Text, TextWrapper, TEXT_WRAPPER);
class!(YText);

methods!(
    YText,
    rtself,
    fn ytext_push(transaction: YTransaction, value: RString) -> NilClass {
        let value_str = value.map_err(|e| VM::raise_ex(e)).unwrap().to_string();

        let mut txn = transaction.map_err(|e| VM::raise_ex(e)).unwrap();

        let t = txn.get_data_mut(&*TRANSACTION_WRAPPER);

        let text = rtself.get_data_mut(&*TEXT_WRAPPER);
        text.push(t, &value_str);

        NilClass::new()
    },
    fn ytext_insert(transaction: YTransaction, index: Fixnum, chunk: RString) -> NilClass {
        let mut txn = transaction.map_err(|e| VM::raise_ex(e)).unwrap();

        let i = index.map_err(|e| VM::raise_ex(e)).unwrap();

        let c = chunk.map_err(|e| VM::raise_ex(e)).unwrap().to_string();

        let tx = txn.get_data_mut(&*TRANSACTION_WRAPPER);

        let text: &Text = rtself.get_data_mut(&*TEXT_WRAPPER);

        text.insert(tx, i.to_u32(), &c);

        NilClass::new()
    },
    fn ytext_to_string() -> RString {
        let text = rtself.get_data(&*TEXT_WRAPPER);

        RString::new_utf8(&text.to_string())
    }
);
