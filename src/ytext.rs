use crate::util::{
    map_attrs_to_hash, map_hash_to_attrs, map_ruby_type_to_rust,
    map_yrs_value_to_ruby
};
use crate::ytransaction::{YTransaction, TRANSACTION_WRAPPER};
use rutie::{
    AnyObject, Fixnum, Hash, Integer, NilClass, Object, Proc, RString, Symbol,
    VM
};
use yrs::types::Delta;
use yrs::{SubscriptionId, Text};

wrappable_struct!(Text, TextWrapper, TEXT_WRAPPER);
class!(YText);

#[rustfmt::skip]
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
        let mapped_attrs = map_hash_to_attrs(a);

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
        let mapped_attrs = map_hash_to_attrs(a);

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
        let text: &Text = rtself.get_data_mut(&*TEXT_WRAPPER);

        text.remove_range(tx, i.to_u32(), l.to_u32());

        NilClass::new()
    },
    fn ytext_format(transaction: YTransaction, index: Fixnum, length: Fixnum, attrs: Hash) -> NilClass {
        let mut txn = transaction.map_err(|e| VM::raise_ex(e)).unwrap();
        let i = index.map_err(|e| VM::raise_ex(e)).unwrap();
        let l = length.map_err(|e| VM::raise_ex(e)).unwrap();

        let a = attrs.map_err(|e| VM::raise_ex(e)).unwrap();
        let mapped_attrs = map_hash_to_attrs(a);

        let tx = txn.get_data_mut(&*TRANSACTION_WRAPPER);
        let text: &Text = rtself.get_data_mut(&*TEXT_WRAPPER);

        text.format(tx, i.to_u32(), l.to_u32(), mapped_attrs);

        NilClass::new()
    },
    fn ytext_observe(callback: Proc) -> Integer {
        let c = callback.map_err(|e| VM::raise_ex(e)).unwrap();

        let text: &mut Text = rtself.get_data_mut(&*TEXT_WRAPPER);
        let subscription_id: SubscriptionId = text
            .observe(move |transaction, text_event| {
                let delta = text_event.delta(transaction);
                for event in delta {
                    match event {
                        Delta::Inserted(v, attrs) => {
                            let mut payload = Hash::new();
                            payload.store(Symbol::new("insert"), map_yrs_value_to_ruby(v.clone()));

                            match attrs {
                                Some(a) => {
                                    let copy = a.clone();
                                    let result = map_attrs_to_hash(*copy).to_any_object();
                                    payload.store(Symbol::new("attributes"), result);
                                },
                                None => ()
                            }

                            let args = &[payload.to_any_object()];
                            c.call(args);
                        },
                        Delta::Retain(position, attrs) => {
                            let mut payload = Hash::new();
                            payload.store(Symbol::new("retain"), Integer::from(*position));

                            match attrs {
                                Some(a) => {
                                    let copy = a.clone();
                                    let result = map_attrs_to_hash(*copy).to_any_object();
                                    payload.store(Symbol::new("attributes"), result);
                                },
                                None => ()
                            }

                            let args = &[payload.to_any_object()];
                            c.call(args);
                        },
                        Delta::Deleted(position) => {
                            let mut payload = Hash::new();
                            payload.store(Symbol::new("delete"), Integer::from(*position));

                            let args = &[payload.to_any_object()];
                            c.call(args);
                        }
                    }
                }
            })
            .into();

        Integer::from(subscription_id)
    },
    fn ytext_to_string() -> RString {
        let text = rtself.get_data(&*TEXT_WRAPPER);

        RString::new_utf8(&text.to_string())
    },
    fn ytext_unobserve(subscription_id: Integer) -> NilClass {
        let s = subscription_id.map_err(|e| VM::raise_ex(e)).unwrap();

        let text: &mut Text = rtself.get_data_mut(&*TEXT_WRAPPER);
        text.unobserve(s.into());

        NilClass::new()
    }
);
