use crate::yvalue::YValue;
use magnus::r_hash::ForEach::Continue;
use magnus::{RHash, Value};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use yrs::types::Attrs;
use yrs::Any;

pub(crate) struct YAttrs(pub(crate) Attrs);

impl From<Attrs> for YAttrs {
    fn from(value: Attrs) -> Self {
        YAttrs(value)
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

        YAttrs(attrs)
    }
}

impl Deref for YAttrs {
    type Target = Attrs;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for YAttrs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
