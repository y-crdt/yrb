use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use magnus::block::Proc;
use magnus::{Error, RHash, RString, Value};
use magnus::value::Qnil;
use lib0::any::Any;
use magnus::r_hash::ForEach::Continue;
use yrs::{Xml, XmlText};
use yrs::types::Attrs;
use crate::{YTransaction, YXmlElement};
use crate::utils::map_rhash_to_attrs;
use crate::yvalue::YValue;

#[magnus::wrap(class = "Y::XMLText")]
pub(crate) struct YXmlText(pub(crate) RefCell<XmlText>);

impl YXmlText {
    pub(crate) fn yxml_text_attributes(&self) -> RHash {
        RHash::from_iter(self.0.borrow().attributes().into_iter())
    }
    pub(crate) fn yxml_text_format(&self, transaction: &YTransaction, index: u32, length: u32, attrs: RHash) -> Result<(), Error> {
        map_rhash_to_attrs(attrs).map(|a| {
            self.0.borrow_mut()
                .format(&mut *transaction.0.borrow_mut(), index, length, a);
        })
    }
    pub(crate) fn yxml_text_get_attribute(&self, name: String) -> Option<String> {
        self.0.borrow()
            .get_attribute(&*name)
    }
    pub(crate) fn yxml_text_insert(&self, transaction: &YTransaction, index: u32, content: String) {
        self.0.borrow_mut()
            .insert(&mut *transaction.0.borrow_mut(), index, &*content)
    }
    pub(crate) fn yxml_text_insert_attribute(&self, transaction: &YTransaction, name: String, value: String) {
        self.0.borrow_mut()
            .insert_attribute(&mut *transaction.0.borrow_mut(), name, value)
    }
    pub(crate) fn yxml_text_insert_embed_with_attributes(&self, transaction: &YTransaction, index: u32, content: Value, attrs: RHash) -> Result<(), Error> {
        let yvalue = YValue::from(content);
        let avalue = Any::from(yvalue);

        map_rhash_to_attrs(attrs).map(|a| {
            self.0.borrow_mut()
                .insert_embed_with_attributes(&mut *transaction.0.borrow_mut(), index, avalue, a);
        })
    }
    pub(crate) fn yxml_text_insert_embed(&self, transaction: &YTransaction, index: u32, embed: Value) {
        self.0.borrow_mut()
            .insert_embed(&mut *transaction.0.borrow_mut(), index, Any::from(YValue::from(embed)))
    }
    pub(crate) fn yxml_text_insert_with_attributes(&self, transaction: &YTransaction, index: u32, content: String, attrs: RHash) -> Result<(), Error> {
        map_rhash_to_attrs(attrs).map(|a| {
            self.0.borrow_mut()
                .insert_with_attributes(&mut *transaction.0.borrow_mut(), index, &*content, a);
        })
    }
    pub(crate) fn yxml_text_length(&self) -> u32 {
        self.0.borrow().len()
    }
    pub(crate) fn yxml_text_next_sibling(&self) -> Option<Value> {
        self.0.borrow().next_sibling().map(|item| match item {
            Xml::Element(el) => Value::from(YXmlElement(RefCell::from(el))),
            Xml::Text(text) => Value::from(YXmlText(RefCell::from(text)))
        })
    }
    pub(crate) fn yxml_text_parent(&self) -> Option<Value> {
        self.0.borrow().parent().map(|item| {
            Value::from(YXmlElement(RefCell::from(item)))
        })
    }
    pub(crate) fn yxml_text_prev_sibling(&self) -> Option<Value> {
        self.0.borrow().prev_sibling().map(|item| match item {
            Xml::Element(el) => Value::from(YXmlElement(RefCell::from(el))),
            Xml::Text(text) => Value::from(YXmlText(RefCell::from(text)))
        })
    }
    pub(crate) fn yxml_text_push(&self, transaction: &YTransaction, content: String) {
        self.0.borrow_mut()
            .push(&mut *transaction.0.borrow_mut(), &*content)
    }
    pub(crate) fn yxml_text_remove_range(&self, transaction: &YTransaction, index: u32, length: u32) {
        self.0.borrow_mut()
            .remove_range(&mut *transaction.0.borrow_mut(), index, length);
    }
    pub(crate) fn yxml_text_to_s(&self) -> String {
        self.0.borrow()
            .to_string()
    }
}
