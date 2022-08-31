use std::rc::Rc;
use lib0::any::Any;
use magnus::{Error, RHash, Value};
use magnus::r_hash::ForEach::Continue;
use yrs::types::Attrs;
use crate::yvalue::YValue;

#[derive(Debug, Clone)]
pub(crate) struct TypeConversionError;

pub(crate) fn map_rhash_to_attrs(hash: RHash) -> Result<Attrs, Error> {
    let mut a: Attrs = Default::default();

    let result = hash.foreach(|key: Value, value: Value| {
        let k = Rc::from(key.to_string());
        let v = Any::from(YValue::from(value));

        a.insert(k, v);

        Ok(Continue)
    });

    if result.is_err() {
        return Err(Error::runtime_error("could not map hash to attrs"))
    }

    Ok(a)
}
