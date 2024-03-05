use crate::yvalue::YValue;
use magnus::r_hash::ForEach::Continue;
use magnus::{RHash, Value};
use std::cell::RefCell;
use std::sync::Arc;
use yrs::types::Attrs;
use yrs::Any;

#[magnus::wrap(class = "Y::Attrs")]
#[derive(Clone)]
pub(crate) struct YAttrs(pub(crate) RefCell<Attrs>);

/// SAFETY: This is safe because we only access this data when the GVL is held.
unsafe impl Send for YAttrs {}

impl From<Attrs> for YAttrs {
    fn from(value: Attrs) -> Self {
        YAttrs(RefCell::from(value))
    }
}

impl From<RHash> for YAttrs {
    fn from(value: RHash) -> Self {
        let mut attrs = Attrs::new();

        value
            .foreach(|key: Value, value: Value| {
                let k = key.to_string();
                let yvalue = YValue::from(value);
                let avalue = Any::from(yvalue);
                attrs.insert(Arc::from(k), avalue);

                Ok(Continue)
            })
            .expect("cannot iterate attributes hash");

        YAttrs(RefCell::from(attrs))
    }
}
