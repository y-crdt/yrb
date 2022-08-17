use std::borrow::Borrow;
use std::ops::{Deref, DerefMut};
use magnus::{RHash, Symbol, Value};
use yrs::types::Attrs;
use crate::yany::YAny;

struct YAttrs(Attrs);

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

impl TryInto<RHash> for YAttrs {
    type Error = ();

    fn try_into(self) -> Result<RHash, Self::Error> {
        let hash = RHash::new();

        for (key, val) in self.iter() {
            let s_key = Symbol::from(key.borrow());
            let s_val: Value = YAny(val.clone()).try_into().unwrap();
            hash.aset(s_key, s_val).expect("TODO: panic message");
        }

        Ok(hash)
    }
}
