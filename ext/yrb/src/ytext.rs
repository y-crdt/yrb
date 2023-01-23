use crate::yattrs::YAttrs;
use crate::yvalue::YValue;
use crate::YTransaction;
use lib0::any::Any;
use magnus::block::Proc;
use magnus::value::Qnil;
use magnus::{Error, RHash, Symbol, Value};
use std::cell::RefCell;
use yrs::types::Delta;
use yrs::{GetString, Observable, Text, TextRef};

#[magnus::wrap(class = "Y::Text")]
pub(crate) struct YText(pub(crate) RefCell<TextRef>);

/// SAFETY: This is safe because we only access this data when the GVL is held.
unsafe impl Send for YText {}

impl YText {
    pub(crate) fn ytext_format(
        &self,
        transaction: &YTransaction,
        index: u32,
        length: u32,
        attrs: RHash,
    ) {
        let mut tx = transaction.transaction();
        let tx = tx.as_mut().unwrap();

        let a = YAttrs::from(attrs);

        self.0.borrow_mut().format(tx, index, length, a.0)
    }
    pub(crate) fn ytext_insert(&self, transaction: &YTransaction, index: u32, chunk: String) {
        let mut tx = transaction.transaction();
        let tx = tx.as_mut().unwrap();

        self.0.borrow_mut().insert(tx, index, chunk.as_str())
    }
    pub(crate) fn ytext_insert_embed(
        &self,
        transaction: &YTransaction,
        index: u32,
        content: Value,
    ) {
        let mut tx = transaction.transaction();
        let tx = tx.as_mut().unwrap();

        let yvalue = YValue::from(content);
        let avalue = Any::from(yvalue);

        self.0.borrow_mut().insert_embed(tx, index, avalue);
    }
    pub(crate) fn ytext_insert_embed_with_attributes(
        &self,
        transaction: &YTransaction,
        index: u32,
        embed: Value,
        attrs: RHash,
    ) {
        let mut tx = transaction.transaction();
        let tx = tx.as_mut().unwrap();

        let yvalue = YValue::from(embed);
        let avalue = Any::from(yvalue);

        let a = YAttrs::from(attrs);

        self.0
            .borrow_mut()
            .insert_embed_with_attributes(tx, index, avalue, a.0);
    }
    pub(crate) fn ytext_insert_with_attributes(
        &self,
        transaction: &YTransaction,
        index: u32,
        chunk: String,
        attrs: RHash,
    ) {
        let mut tx = transaction.transaction();
        let tx = tx.as_mut().unwrap();

        let a = YAttrs::from(attrs);

        self.0
            .borrow_mut()
            .insert_with_attributes(tx, index, chunk.as_str(), a.0)
    }
    pub(crate) fn ytext_length(&self, transaction: &YTransaction) -> u32 {
        let tx = transaction.transaction();
        let tx = tx.as_ref().unwrap();

        self.0.borrow().len(tx)
    }
    pub(crate) fn ytext_observe(&self, block: Proc) -> Result<u32, Error> {
        let delta_insert = Symbol::new("insert").to_static();
        let delta_retain = Symbol::new("retain").to_static();
        let delta_delete = Symbol::new("delete").to_static();
        let attributes = Symbol::new("attributes").to_static();

        // let mut error: Option<Error> = None;
        let subscription_id = self
            .0
            .borrow_mut()
            .observe(move |transaction, text_event| {
                let delta = text_event.delta(transaction);
                let (_, errors): (Vec<_>, Vec<_>) = delta
                    .iter()
                    .map(|change| match change {
                        Delta::Inserted(value, attrs) => {
                            let yvalue = YValue::from(value.clone());
                            let payload = RHash::new();
                            payload
                                .aset(delta_insert, yvalue.0.into_inner())
                                .map(|()| match attrs {
                                    Some(a) => a
                                        .clone()
                                        .into_iter()
                                        .map(|(key, val)| {
                                            let yvalue = YValue::from(val);
                                            (key.to_string(), yvalue.0.into_inner())
                                        })
                                        .collect::<RHash>()
                                        .into(),
                                    None => None,
                                })
                                .map(|attrs_hash| attrs_hash.map(|v| payload.aset(attributes, v)))
                                .map(|_| block.call::<(RHash,), Qnil>((payload,)))
                        }
                        Delta::Retain(index, attrs) => {
                            let payload = RHash::new();

                            let yvalue = YValue::from(*index);

                            payload
                                .aset(delta_retain, yvalue.0.into_inner())
                                .map(|()| match attrs {
                                    Some(a) => a
                                        .clone()
                                        .into_iter()
                                        .map(|(key, val)| {
                                            let yvalue = YValue::from(val);
                                            (key.to_string(), yvalue.0.into_inner())
                                        })
                                        .collect::<RHash>()
                                        .into(),
                                    None => None,
                                })
                                .map(|attrs_hash| attrs_hash.map(|v| payload.aset(attributes, v)))
                                .map(|_| block.call::<(RHash,), Qnil>((payload,)))
                        }
                        Delta::Deleted(index) => {
                            let payload = RHash::new();

                            let yvalue = YValue::from(*index);

                            payload
                                .aset(delta_delete, yvalue.0.into_inner())
                                .map(|()| block.call::<(RHash,), Qnil>((payload,)))
                        }
                    })
                    .partition(Result::is_ok);

                if !errors.is_empty() {
                    // todo: make sure we respect errors and let the method fail by
                    //  by returning a Result containing an Error
                }
            })
            .into();

        Ok(subscription_id)
    }
    pub(crate) fn ytext_push(&self, transaction: &YTransaction, chunk: String) {
        let mut tx = transaction.transaction();
        let tx = tx.as_mut().unwrap();

        self.0.borrow_mut().push(tx, chunk.as_str())
    }
    pub(crate) fn ytext_remove_range(&self, transaction: &YTransaction, start: u32, length: u32) {
        let mut tx = transaction.transaction();
        let tx = tx.as_mut().unwrap();

        self.0.borrow_mut().remove_range(tx, start, length)
    }
    pub(crate) fn ytext_to_s(&self, transaction: &YTransaction) -> String {
        let mut tx = transaction.transaction();
        let tx = tx.as_mut().unwrap();

        self.0.borrow().get_string(tx)
    }
    pub(crate) fn ytext_unobserve(&self, subscription_id: u32) {
        self.0.borrow_mut().unobserve(subscription_id);
    }
}

impl From<TextRef> for YText {
    fn from(v: TextRef) -> Self {
        YText(RefCell::from(v))
    }
}
