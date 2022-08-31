use std::borrow::Borrow;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use lib0::any::Any;
use magnus::{RHash, Symbol, Value};
use magnus::r_hash::ForEach::Continue;
use yrs::types::Attrs;
use crate::yany::YAny;
use crate::yvalue::YValue;

pub(crate) struct YAttrs(pub(crate) Attrs);

impl From<Attrs> for YAttrs {
    fn from(value: Attrs) -> Self {
        YAttrs { 0: value }
    }
}

impl From<RHash> for YAttrs {
    fn from(value: RHash) -> Self {
        let mut attrs = Attrs::new();

        value.foreach(|key: Value, value: Value| {
            let k = key.to_string();
            let yvalue = YValue::from(value);
            let avalue = Any::from(yvalue);
            attrs.insert(Rc::from(k), avalue);

            Ok(Continue)
        }).expect("cannot iterate attributes hash");

        YAttrs { 0: attrs }
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
