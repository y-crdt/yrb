use crate::yattrs::YAttrs;
use crate::ydiff::YDiff;
use crate::yvalue::YValue;
use crate::YTransaction;
use magnus::block::Proc;
use magnus::value::Qnil;
use magnus::RArray;
pub(crate) use magnus::{Error, IntoValue, RHash, Ruby, Value};
use std::cell::RefCell;
use yrs::types::text::YChange;
use yrs::types::Delta;
use yrs::{Any, GetString, Observable, Text, TextRef};

#[magnus::wrap(class = "Y::Text")]
pub(crate) struct YText(pub(crate) RefCell<TextRef>);

/// SAFETY: This is safe because we only access this data when the GVL is held.
unsafe impl Send for YText {}

impl YText {
    pub(crate) fn ytext_diff(&self, transaction: &YTransaction) -> RArray {
        let ruby = unsafe { Ruby::get_unchecked() };
        let tx = transaction.transaction();
        let tx = tx.as_ref().unwrap();

        let array = ruby.ary_new();
        for diff in self.0.borrow().diff(tx, YChange::identity).iter() {
            let yvalue = YValue::from(diff.insert.clone());
            let insert = yvalue.0.into_inner();
            let attributes = diff.attributes.as_ref().map_or_else(
                || None,
                |boxed_attrs| {
                    let attributes = ruby.hash_new();
                    for (key, value) in boxed_attrs.iter() {
                        let key = key.to_string();
                        let value = YValue::from(value.clone()).0.into_inner();
                        attributes.aset(key, value).expect("cannot add value");
                    }
                    Some(attributes)
                },
            );
            let ydiff = YDiff {
                ydiff_insert: insert,
                ydiff_attrs: attributes,
            };
            array
                .push(ydiff.into_value_with(&ruby))
                .expect("cannot push diff to array");
        }
        array
    }
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

        self.0
            .borrow_mut()
            .format(tx, index, length, a.0.into_inner())
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
            .insert_embed_with_attributes(tx, index, avalue, a.0.into_inner());
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
            .insert_with_attributes(tx, index, chunk.as_str(), a.0.into_inner())
    }
    pub(crate) fn ytext_length(&self, transaction: &YTransaction) -> u32 {
        let tx = transaction.transaction();
        let tx = tx.as_ref().unwrap();

        self.0.borrow().len(tx)
    }
    pub(crate) fn ytext_observe(&self, block: Proc) -> Result<u32, Error> {
        let ruby = unsafe { Ruby::get_unchecked() };
        let delta_insert = ruby.to_symbol("insert").to_static();
        let delta_retain = ruby.to_symbol("retain").to_static();
        let delta_delete = ruby.to_symbol("delete").to_static();
        let attributes = ruby.to_symbol("attributes").to_static();

        // let mut error: Option<Error> = None;
        let subscription_id = self
            .0
            .borrow_mut()
            .observe(move |transaction, text_event| {
                let ruby = unsafe { Ruby::get_unchecked() };
                let delta = text_event.delta(transaction);
                let (_, errors): (Vec<_>, Vec<_>) = delta
                    .iter()
                    .map(|change| match change {
                        Delta::Inserted(value, attrs) => {
                            let yvalue = YValue::from(value.clone());
                            let payload = ruby.hash_new();
                            payload
                                .aset(delta_insert, yvalue.0.into_inner())
                                .map(|()| match attrs {
                                    Some(a) => {
                                        let attrs_hash = ruby.hash_new();
                                        for (key, val) in a.clone().into_iter() {
                                            let yvalue = YValue::from(val);
                                            attrs_hash
                                                .aset(key.to_string(), yvalue.0.into_inner())
                                                .expect("cannot add attr");
                                        }
                                        Some(attrs_hash)
                                    }
                                    None => None,
                                })
                                .map(|attrs_hash| attrs_hash.map(|v| payload.aset(attributes, v)))
                                .map(|_| block.call::<(RHash,), Qnil>((payload,)))
                        }
                        Delta::Retain(index, attrs) => {
                            let payload = ruby.hash_new();

                            let yvalue = YValue::from(*index);

                            payload
                                .aset(delta_retain, yvalue.0.into_inner())
                                .map(|()| match attrs {
                                    Some(a) => {
                                        let attrs_hash = ruby.hash_new();
                                        for (key, val) in a.clone().into_iter() {
                                            let yvalue = YValue::from(val);
                                            attrs_hash
                                                .aset(key.to_string(), yvalue.0.into_inner())
                                                .expect("cannot add attr");
                                        }
                                        Some(attrs_hash)
                                    }
                                    None => None,
                                })
                                .map(|attrs_hash| attrs_hash.map(|v| payload.aset(attributes, v)))
                                .map(|_| block.call::<(RHash,), Qnil>((payload,)))
                        }
                        Delta::Deleted(index) => {
                            let payload = ruby.hash_new();

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
