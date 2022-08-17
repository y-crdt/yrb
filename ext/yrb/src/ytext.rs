use std::cell::RefCell;
use lib0::any::Any;
use magnus::{Error, exception, RHash, Value};
use magnus::block::Proc;
use yrs::{Text};
use crate::utils::{map_magnus_rhash_to_lib0_attrs};
use crate::YTransaction;
use crate::yvalue::YValue;

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
        let yvalue = YValue::from(content);
        let avalue = Any::from(yvalue);

        self.0.borrow_mut()
            .insert_embed(&mut *transaction.0.borrow_mut(), index, avalue);

        Ok(())
    }
    pub(crate) fn ytext_insert_embed_with_attributes(
        &self,
        transaction: YTransaction,
        index: u32,
        embed: Value,
        attrs: RHash) -> Result<(), Error> {
        let yvalue = YValue::from(embed);
        let avalue = Any::from(yvalue);

        let a = match map_magnus_rhash_to_lib0_attrs(attrs) {
            Ok(val) => val,
            Err(_e) => return Err(Error::new(
                exception::type_error(),
                "incompatible type for `attrs`",
            )),
        };

        self.0
            .borrow_mut()
            .insert_embed_with_attributes(&mut *transaction.0.borrow_mut(), index, avalue, a);

        Ok(())
    }
    pub(crate) fn ytext_length(&self) -> u32 {
        return self.0
            .borrow()
            .len();
    }
    pub(crate) fn ytext_observe(&self, _block: Proc) -> u32 {
        let subscription_id = self.0
            .borrow_mut()
            .observe(move |transaction, text_event| {
                let delta = text_event.delta(transaction);
                for _event in delta {
                    // match event {
                    //     Delta::Inserted(v, attrs) => {
                    //         let yattrs = YAttrs(attrs);
                    //         yattrs.try_into();
                    //         let mut payload = RHash::new();
                    //         payload.aset(Symbol::from_value("attributes"), attrs);
                    //     }
                    //     Delta::Deleted()
                    //     _ => {}
                    // }
                }
            });

        subscription_id.into()
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
    pub(crate) fn ytext_unobserve(&self, subscription_id: u32) {
        return self.0
            .borrow_mut()
            .unobserve(subscription_id);
    }
}
