use crate::utils::map_attrs_to_rhash;
use crate::yattrs::YAttrs;
use crate::yvalue::YValue;
use crate::YTransaction;
use lib0::any::Any;
use magnus::block::Proc;
use magnus::value::Qnil;
use magnus::{Error, RHash, StaticSymbol, Symbol, Value, QNIL};
use std::cell::RefCell;
use std::ops::Deref;
use yrs::types::text::ChangeKind;
use yrs::types::Value::{YArray, YMap, YXmlElement, YXmlText};
use yrs::types::{text::YChange, Delta};
use yrs::Text;

#[magnus::wrap(class = "Y::Text")]
pub(crate) struct YText(pub(crate) RefCell<Text>);

/// SAFETY: This is safe because we only access this data when the GVL is held.
unsafe impl Send for YText {}

impl YText {
    pub(crate) fn ytext_diff(&self, transaction: &YTransaction) -> Vec<RHash> {
        let diff_attributes = StaticSymbol::from("attributes");
        let diff_insert = StaticSymbol::from("insert");
        let diff_ychange = StaticSymbol::from("ychange");

        let insert_type = StaticSymbol::from("type");
        let insert_value = StaticSymbol::from("value");

        let insert_type_any = StaticSymbol::from("any");
        let insert_type_text = StaticSymbol::from("text");
        let insert_type_array = StaticSymbol::from("array");
        let insert_type_map = StaticSymbol::from("map");
        let insert_type_xml_element = StaticSymbol::from("xml_element");
        let insert_type_xml_text = StaticSymbol::from("xml_text");

        self.0
            .borrow()
            .diff(&mut *transaction.0.borrow_mut(), YChange::identity)
            .iter()
            .map(move |diff| {
                let attributes = match &diff.attributes {
                    Some(attrs) => map_attrs_to_rhash(attrs.deref())
                        .map(Value::from)
                        .unwrap_or_else(|| Value::from(QNIL)),
                    _ => Value::from(QNIL),
                };

                let (i_type, i_value) = match diff.insert.clone() {
                    yrs::types::Value::Any(v) => (insert_type_any, YValue::from(v).0.into_inner()),
                    yrs::types::Value::YText(v) => {
                        (insert_type_text, YValue::from(v).0.into_inner())
                    }
                    YArray(v) => (insert_type_array, YValue::from(v).0.into_inner()),
                    YMap(v) => (insert_type_map, YValue::from(v).0.into_inner()),
                    YXmlElement(v) => (insert_type_xml_element, YValue::from(v).0.into_inner()),
                    YXmlText(v) => (insert_type_xml_text, YValue::from(v).0.into_inner()),
                };
                let insert = RHash::new();
                insert
                    .aset(insert_type, i_type)
                    .expect("cannot set insert type");
                insert
                    .aset(insert_value, i_value)
                    .expect("cannot set insert value");

                let ychange = match diff.ychange.clone() {
                    None => Value::from(QNIL),
                    Some(yv) => {
                        let id = RHash::new();
                        id.aset("client_id", yv.id.client)
                            .expect("cannot extract client_id");
                        id.aset("clock", yv.id.clock).expect("cannot extract clock");

                        let added = StaticSymbol::from("added");
                        let removed = StaticSymbol::from("removed");

                        let kind = match yv.kind {
                            ChangeKind::Added => added,
                            ChangeKind::Removed => removed,
                        };

                        let change = RHash::new();
                        change.aset("id", id).expect("cannot set id");
                        change.aset("kind", kind).expect("cannot set kind");

                        Value::from(change)
                    }
                };

                let diff = RHash::new();
                diff.aset(diff_insert, insert).expect("cannot set insert");
                diff.aset(diff_attributes, attributes)
                    .expect("cannot set attributes");
                diff.aset(diff_ychange, ychange)
                    .expect("cannot set ychange");

                diff
            })
            .collect()
    }

    pub(crate) fn ytext_format(
        &self,
        transaction: &YTransaction,
        index: u32,
        length: u32,
        attrs: RHash,
    ) -> Result<(), Error> {
        let a = YAttrs::from(attrs);
        self.0
            .borrow_mut()
            .format(&mut *transaction.0.borrow_mut(), index, length, a.0);

        Ok(())
    }
    pub(crate) fn ytext_insert(
        &self,
        transaction: &YTransaction,
        index: u32,
        chunk: String,
    ) -> Result<(), Error> {
        self.0
            .borrow_mut()
            .insert(&mut *transaction.0.borrow_mut(), index, &*chunk);

        Ok(())
    }
    pub(crate) fn ytext_insert_embed(
        &self,
        transaction: &YTransaction,
        index: u32,
        content: Value,
    ) -> Result<(), Error> {
        let yvalue = YValue::from(content);
        let avalue = Any::from(yvalue);

        self.0
            .borrow_mut()
            .insert_embed(&mut *transaction.0.borrow_mut(), index, avalue);

        Ok(())
    }
    pub(crate) fn ytext_insert_embed_with_attributes(
        &self,
        transaction: &YTransaction,
        index: u32,
        embed: Value,
        attrs: RHash,
    ) -> Result<(), Error> {
        let yvalue = YValue::from(embed);
        let avalue = Any::from(yvalue);

        let a = YAttrs::from(attrs);

        self.0.borrow_mut().insert_embed_with_attributes(
            &mut *transaction.0.borrow_mut(),
            index,
            avalue,
            a.0,
        );

        Ok(())
    }
    pub(crate) fn ytext_insert_with_attributes(
        &self,
        transaction: &YTransaction,
        index: u32,
        chunk: String,
        attrs: RHash,
    ) -> Result<(), Error> {
        let a = YAttrs::from(attrs);

        self.0.borrow_mut().insert_with_attributes(
            &mut *transaction.0.borrow_mut(),
            index,
            &*chunk,
            a.0,
        );

        Ok(())
    }
    pub(crate) fn ytext_length(&self) -> u32 {
        self.0.borrow().len()
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
        self.0
            .borrow_mut()
            .push(&mut *transaction.0.borrow_mut(), &*chunk);
    }
    pub(crate) fn ytext_remove_range(
        &self,
        transaction: &YTransaction,
        start: u32,
        length: u32,
    ) -> Result<(), Error> {
        self.0
            .borrow_mut()
            .remove_range(&mut *transaction.0.borrow_mut(), start, length);

        Ok(())
    }
    pub(crate) fn ytext_to_s(&self) -> String {
        return self.0.borrow().to_string();
    }
    pub(crate) fn ytext_unobserve(&self, subscription_id: u32) {
        return self.0.borrow_mut().unobserve(subscription_id);
    }
}
