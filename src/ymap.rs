use crate::util::{map_ruby_type_to_rust, map_yrs_value_to_ruby};
use crate::ytransaction::{YTransaction, TRANSACTION_WRAPPER};
use lib0::any::Any;
use rutie::{
    AnyObject, Boolean, Class, Fixnum, Hash, NilClass, Object, Proc, RString,
    Symbol, VM,
};
use std::rc::Rc;
use yrs::Map;

wrappable_struct!(Map, MapWrapper, MAP_WRAPPER);
class!(YMap);

#[rustfmt::skip]
methods!(
    YMap,
    rtself,
    fn ymap_clear(transaction: YTransaction) -> NilClass {
        let mut tx = transaction.map_err(|e| VM::raise_ex(e)).unwrap();
        let txn = tx.get_data_mut(&*TRANSACTION_WRAPPER);

        let m: &Map = rtself.get_data(&*MAP_WRAPPER);
        m.clear(txn);

        NilClass::new()
    },
    fn ymap_contains(key: AnyObject) -> Boolean {
        let k = key.map_err(|e| VM::raise_ex(e)).unwrap();

        let k2 = if let Ok(t) = k.try_convert_to::<Symbol>() {
            t.to_string()
        } else if let Ok(t) = k.try_convert_to::<RString>() {
            t.to_string()
        } else {
            VM::raise(Class::from_existing("IllegalArgumentError"), "Only strings and symbols are supported as map keys.");
            return Boolean::new(false);
        };

        let m: &Map = rtself.get_data(&*MAP_WRAPPER);
        let result = m.contains(&k2);

        Boolean::new(result)
    },
    fn ymap_each(block: Proc) -> NilClass {
        let b = block.map_err(|e| VM::raise_ex(e)).unwrap();

        let m: &Map = rtself.get_data(&*MAP_WRAPPER);

        m
            .iter()
            .for_each(|(key, val)| {
                let args = [
                    RString::new_utf8(key).to_any_object(),
                    map_yrs_value_to_ruby(val)
                ];
                b.call(&args);
            });

        NilClass::new()
    },
    fn ymap_get(key: AnyObject) -> AnyObject {
        let k = key.map_err(|e| VM::raise_ex(e)).unwrap();

        let k2 = if let Ok(t) = k.try_convert_to::<Symbol>() {
            t.to_string()
        } else if let Ok(t) = k.try_convert_to::<RString>() {
            t.to_string()
        } else {
            VM::raise(Class::from_existing("IllegalArgumentError"), "Only strings and symbols are supported as map keys.");
            return NilClass::new().to_any_object();
        };

        let m: &Map = rtself.get_data(&*MAP_WRAPPER);
        let result = m.get(&k2);

        map_yrs_value_to_ruby(result.unwrap_or(yrs::types::Value::Any(Any::Null)))
    },
    fn ymap_insert(transaction: YTransaction, key: AnyObject, value: AnyObject) -> AnyObject {
        let mut tx = transaction.map_err(|e| VM::raise_ex(e)).unwrap();
        let k = key.map_err(|e| VM::raise_ex(e)).unwrap();
        let v = value.map_err(|e| VM::raise_ex(e)).unwrap();

        let k2 = if let Ok(t) = k.try_convert_to::<Symbol>() {
            t.to_string()
        } else if let Ok(t) = k.try_convert_to::<RString>() {
            t.to_string()
        } else {
            VM::raise(Class::from_existing("IllegalArgumentError"), "Only strings and symbols are supported as map keys.");
            return NilClass::new().to_any_object();
        };

        let txn = tx.get_data_mut(&*TRANSACTION_WRAPPER);

        let m: &Map = rtself.get_data(&*MAP_WRAPPER);

        let result = m.insert(
            txn,
            Rc::from(k2),
            map_ruby_type_to_rust(v).unwrap()
        );

        map_yrs_value_to_ruby(result.unwrap_or(yrs::types::Value::Any(Any::Null)))
    },
    fn ymap_remove(transaction: YTransaction, key: AnyObject) -> AnyObject {
        let mut tx = transaction.map_err(|e| VM::raise_ex(e)).unwrap();
        let k = key.map_err(|e| VM::raise_ex(e)).unwrap();

        let k2 = if let Ok(t) = k.try_convert_to::<Symbol>() {
            t.to_string()
        } else if let Ok(t) = k.try_convert_to::<RString>() {
            t.to_string()
        } else {
            VM::raise(Class::from_existing("IllegalArgumentError"), "Only strings and symbols are supported as map keys.");
            return NilClass::new().to_any_object();
        };

        let txn = tx.get_data_mut(&*TRANSACTION_WRAPPER);

        let m: &Map = rtself.get_data(&*MAP_WRAPPER);
        let result = m.remove(txn, &k2);

        map_yrs_value_to_ruby(result.unwrap_or(yrs::types::Value::Any(Any::Null)))
    },
    fn ymap_size() -> Fixnum {
        let m: &Map = rtself.get_data(&*MAP_WRAPPER);
        Fixnum::new(i64::from(m.len()))
    },
    fn ymap_to_hash() -> Hash {
        let m: &Map = rtself.get_data(&*MAP_WRAPPER);
        let mut h = Hash::new();

        for (key, val) in m.iter() {
            h.store(Symbol::new(key), map_yrs_value_to_ruby(val));
        }

        h
    },
);
