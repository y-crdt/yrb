use std::borrow::BorrowMut;
use std::cell::RefCell;
use lib0::any::Any;
use magnus::{Error, exception, RHash, Value};
use yrs::{Text};
use crate::utils::{map_magnus_rhash_to_lib0_attrs, map_magnus_value_to_lib0_any};
use crate::YTransaction;

#[magnus::wrap(class = "Y::Text")]
pub(crate) struct YText(pub(crate) RefCell<Text>);

impl YText {
    pub(crate) fn ytext_new(text: Text) -> Self {
        Self(RefCell::new(text))
    }
    pub(crate) fn ytext_insert(&self, transaction: &YTransaction, index: u32, chunk: String) {
        self.0
            .borrow_mut()
            .insert(&mut *transaction.0.borrow_mut(), index, &*chunk);
    }
    pub(crate) fn ytext_insert_embed(&self, transaction: &YTransaction, index: u32, content: Value) -> Result<(), Error> {
        let c = match map_magnus_value_to_lib0_any(content) {
            Ok(val) => val,
            Err(_e) => return Err(Error::new(
                exception::type_error(),
                "incompatible type for `content`"
            )),
        };

        self.0
            .borrow_mut()
            .insert_embed(&mut *transaction.0.borrow_mut(), index, c);

        Ok(())
    }
    pub(crate) fn ytext_insert_embed_with_attributes(
        &self,
        transaction: YTransaction,
        index: u32,
        embed: Value,
        attrs: RHash) -> Result<(), Error> {
        let e = match map_magnus_value_to_lib0_any(embed) {
            Ok(val) => val,
            Err(_e) => return Err(Error::new(
                exception::type_error(),
                "incompatible type for `embed`"
            )),
        };

        let a = match map_magnus_rhash_to_lib0_attrs(attrs) {
            Ok(val) => val,
            Err(_e) => return Err(Error::new(
                exception::type_error(),
                "incompatible type for `attrs`"
            )),
        };

        self.0
            .borrow_mut()
            .insert_embed_with_attributes(&mut *transaction.0.borrow_mut(), index, e, a);

        Ok(())
    }
    pub(crate) fn ytext_length(&self) -> u32 {
        return self.0
            .borrow()
            .len()
    }
    pub(crate) fn ytext_push(&self, transaction: &YTransaction, chunk: String) {
        self.0
            .borrow_mut()
            .push(&mut *transaction.0.borrow_mut(), &*chunk);
    }
    pub(crate) fn ytext_to_s(&self) -> String {
        return self.0
            .borrow()
            .to_string();
    }
}
