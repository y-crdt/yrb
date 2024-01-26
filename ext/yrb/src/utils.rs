use crate::yvalue::YValue;
use magnus::r_hash::ForEach::Continue;
use magnus::{exception, Error, RArray, RHash, RString, Symbol, Value};
use std::sync::Arc;
use yrs::types::{Attrs, Value as YrsValue};
use yrs::{Any, Array, Map, TransactionMut};

#[derive(Debug, Clone)]
pub(crate) struct TypeConversionError;

pub(crate) fn indifferent_hash_key(key: Value) -> Option<String> {
    RString::from_value(key)
        .map(|v| v.to_string().unwrap())
        .or_else(|| Symbol::from_value(key).map(|v| v.name().unwrap().to_string()))
}

pub(crate) fn map_rhash_to_attrs(hash: RHash) -> Result<Attrs, Error> {
    let mut a: Attrs = Default::default();

    let result = hash.foreach(|key: Value, value: Value| {
        let k = Arc::from(key.to_string());
        let v = Any::from(YValue::from(value));

        a.insert(k, v);

        Ok(Continue)
    });

    if result.is_err() {
        return Err(Error::new(
            exception::runtime_error(),
            "could not map hash to attrs",
        ));
    }

    Ok(a)
}

pub(crate) fn convert_yvalue_to_ruby_value(value: YrsValue, tx: &TransactionMut) -> YValue {
    match value {
        YrsValue::Any(val) => YValue::from(val),
        YrsValue::YText(text) => YValue::from(text),
        YrsValue::YXmlElement(el) => YValue::from(el),
        YrsValue::YXmlText(text) => YValue::from(text),
        YrsValue::YArray(val) => {
            let arr = RArray::new();
            for item in val.iter(tx) {
                let val = convert_yvalue_to_ruby_value(item.clone(), tx);
                let val = *val.0.borrow();
                arr.push(val).expect("cannot push item event to array");
            }
            YValue::from(arr)
        }
        YrsValue::YMap(val) => {
            let iter = val.iter(tx).map(|(key, val)| {
                let val = convert_yvalue_to_ruby_value(val.clone(), tx);
                let val = val.0.into_inner();
                (key, val)
            });
            YValue::from(RHash::from_iter(iter))
        }
        v => panic!("cannot map given yrs values to yvalue: {:?}", v),
    }
}
