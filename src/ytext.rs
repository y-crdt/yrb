use crate::ytransaction::{YTransaction, TRANSACTION_WRAPPER};
use rutie::{AnyObject, Fixnum, Hash, NilClass, Object, RString, VM};
use std::rc::Rc;
use yrs::types::Attrs;
use yrs::{Text};
use crate::util::{map_hash_to_rust, map_ruby_type_to_rust};

wrappable_struct!(Text, TextWrapper, TEXT_WRAPPER);
class!(YText);

methods!(
    YText,
    rtself,
    fn ytext_insert(transaction: YTransaction, index: Fixnum, chunk: RString) -> NilClass {
        let mut txn = transaction.map_err(|e| VM::raise_ex(e)).unwrap();

        let i = index.map_err(|e| VM::raise_ex(e)).unwrap();

        let c = chunk.map_err(|e| VM::raise_ex(e)).unwrap().to_string();

        let tx = txn.get_data_mut(&*TRANSACTION_WRAPPER);

        let text: &Text = rtself.get_data_mut(&*TEXT_WRAPPER);

        text.insert(tx, i.to_u32(), &c);

        NilClass::new()
    },
    fn ytext_insert_embed(
        transaction: YTransaction,
        index: Fixnum,
        content: AnyObject) -> NilClass {
        let mut txn = transaction.map_err(|e| VM::raise_ex(e)).unwrap();
        let i = index.map_err(|e| VM::raise_ex(e)).unwrap();

        let c = content.map_err(|e| VM::raise_ex(e)).unwrap();
        let v = map_ruby_type_to_rust(c).map_err(|e| VM::raise_ex(e)).unwrap();

        let tx = txn.get_data_mut(&*TRANSACTION_WRAPPER);

        let text: &Text = rtself.get_data_mut(&*TEXT_WRAPPER);

        text.insert_embed(tx, i.to_u32(), v);

        NilClass::new()
    }
    fn ytext_insert_embed_with_attributes(
        transaction: YTransaction,
        index: Fixnum,
        embed: AnyObject,
        attrs: Hash) -> NilClass {
        let mut txn = transaction.map_err(|e| VM::raise_ex(e)).unwrap();
        let i = index.map_err(|e| VM::raise_ex(e)).unwrap();

        let c = embed.map_err(|e| VM::raise_ex(e)).unwrap();
        let v = map_ruby_type_to_rust(c).map_err(|e| VM::raise_ex(e)).unwrap();

        let a = attrs.map_err(|e| VM::raise_ex(e)).unwrap();

        let map = map_hash_to_rust(a);
        let mut mapped_attrs = Attrs::with_capacity(map.len());
        for (k, v) in map {
            mapped_attrs.insert(Rc::from(k), v);
        }

        let tx = txn.get_data_mut(&*TRANSACTION_WRAPPER);

        let text: &Text = rtself.get_data_mut(&*TEXT_WRAPPER);

        text.insert_embed_with_attributes(tx, i.to_u32(), v, mapped_attrs);

        NilClass::new()
    }
    fn ytext_insert_with_attributes(
        transaction: YTransaction,
        index: Fixnum,
        chunk: RString,
        attrs: Hash) -> NilClass {
        let mut txn = transaction.map_err(|e| VM::raise_ex(e)).unwrap();
        let i = index.map_err(|e| VM::raise_ex(e)).unwrap();

        let c = chunk.map_err(|e| VM::raise_ex(e)).unwrap().to_string();

        let a = attrs.map_err(|e| VM::raise_ex(e)).unwrap();

        let map = map_hash_to_rust(a);
        let mut mapped_attrs = Attrs::with_capacity(map.len());
        for (k, v) in map {
            mapped_attrs.insert(Rc::from(k), v);
        }

        let tx = txn.get_data_mut(&*TRANSACTION_WRAPPER);

        let text: &Text = rtself.get_data_mut(&*TEXT_WRAPPER);

        text.insert_with_attributes(tx, i.to_u32(), &c, mapped_attrs);

        NilClass::new()
    },
    fn ytext_length() -> Fixnum {
        let text = rtself.get_data(&*TEXT_WRAPPER);

        Fixnum::new(i64::from(text.len()))
    }
    fn ytext_push(transaction: YTransaction, value: RString) -> NilClass {
        let value_str = value.map_err(|e| VM::raise_ex(e)).unwrap().to_string();

        let mut txn = transaction.map_err(|e| VM::raise_ex(e)).unwrap();

        let t = txn.get_data_mut(&*TRANSACTION_WRAPPER);

        let text = rtself.get_data_mut(&*TEXT_WRAPPER);
        text.push(t, &value_str);

        NilClass::new()
    },
    fn ytext_remove_range(transaction: YTransaction, index: Fixnum, length: Fixnum) -> NilClass {
        let mut txn = transaction.map_err(|e| VM::raise_ex(e)).unwrap();
        let i = index.map_err(|e| VM::raise_ex(e)).unwrap();
        let l = length.map_err(|e| VM::raise_ex(e)).unwrap();

        let tx = txn.get_data_mut(&*TRANSACTION_WRAPPER);
        let text = rtself.get_data_mut(&*TEXT_WRAPPER);

        text.remove_range(tx, i.to_u32(), l.to_u32());

        NilClass::new()
    },
    fn format(transaction: YTransaction, index: Fixnum, length: Fixnum, attrs: Hash) -> NilClass {
        let mut txn = transaction.map_err(|e| VM::raise_ex(e)).unwrap();
        let i = index.map_err(|e| VM::raise_ex(e)).unwrap();
        let l = length.map_err(|e| VM::raise_ex(e)).unwrap();

        let a = attrs.map_err(|e| VM::raise_ex(e)).unwrap();

        let map = map_hash_to_rust(a);
        let mut mapped_attrs = Attrs::with_capacity(map.len());
        for (k, v) in map {
            mapped_attrs.insert(Rc::from(k), v);
        }

        let tx = txn.get_data_mut(&*TRANSACTION_WRAPPER);
        let text = rtself.get_data_mut(&*TEXT_WRAPPER);

       text.format(tx, i.to_u32(), l.to_u32(), mapped_attrs);

        NilClass::new()
    }
    fn ytext_to_string() -> RString {
        let text = rtself.get_data(&*TEXT_WRAPPER);

        RString::new_utf8(&text.to_string())
    }
);
