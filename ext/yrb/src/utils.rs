use crate::yvalue::YValue;
use magnus::r_hash::ForEach::Continue;
use magnus::{Error, RHash, RString, Ruby, Symbol, Value};
use std::sync::Arc;
use yrs::types::{Attrs, Value as YrsValue};
use yrs::{Any, Array, Map, TransactionMut};

#[allow(dead_code)]
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
        let ruby = Ruby::get().unwrap();
        return Err(Error::new(
            ruby.exception_runtime_error(),
            "could not map hash to attrs",
        ));
    }

    Ok(a)
}

pub(crate) fn convert_yvalue_to_ruby_value(value: YrsValue, tx: &TransactionMut) -> YValue {
    let ruby = unsafe { Ruby::get_unchecked() };
    match value {
        YrsValue::Any(val) => YValue::from(val),
        YrsValue::YText(text) => YValue::from(text),
        YrsValue::YXmlElement(el) => YValue::from(el),
        YrsValue::YXmlText(text) => YValue::from(text),
        YrsValue::YArray(val) => {
            let arr = ruby.ary_new();
            for item in val.iter(tx) {
                let val = convert_yvalue_to_ruby_value(item.clone(), tx);
                let val = *val.0.borrow();
                arr.push(val).expect("cannot push item event to array");
            }
            YValue::from(arr)
        }
        YrsValue::YMap(val) => {
            let hash = ruby.hash_new();
            for (key, value) in val.iter(tx) {
                let val = convert_yvalue_to_ruby_value(value.clone(), tx);
                let val = val.0.into_inner();
                hash.aset(key, val).expect("cannot insert into hash");
            }
            YValue::from(hash)
        }
        v => panic!("cannot map given yrs values to yvalue: {:?}", v),
    }
}
