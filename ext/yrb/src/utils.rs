use std::rc::Rc;
use lib0::any::Any;
use magnus::{Error, RHash, RString, Symbol, Value};
use magnus::r_hash::ForEach::Continue;
use yrs::types::Attrs;
use crate::yvalue::YValue;

#[derive(Debug, Clone)]
pub(crate) struct TypeConversionError;

pub(crate) fn indifferent_hash_key(key: Value) -> Option<String> {
    RString::from_value(key)
        .map(|v| v.to_string().unwrap())
        .or_else(|| {
            Symbol::from_value(key)
                .map(|v| v.name().unwrap().to_string())
        })
}

pub(crate) fn map_rhash_to_attrs(hash: RHash) -> Result<Attrs, Error> {
    let mut a: Attrs = Default::default();

    let result = hash.foreach(|key: Value, value: Value| {
        let k = Rc::from(key.to_string());
        let v = Any::from(YValue::from(value));

        a.insert(k, v);

        Ok(Continue)
    });

    if result.is_err() {
        return Err(Error::runtime_error("could not map hash to attrs"));
    }

    Ok(a)
}
