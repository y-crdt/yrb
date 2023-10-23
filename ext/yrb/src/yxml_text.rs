use crate::utils::map_rhash_to_attrs;
use crate::yvalue::YValue;
use crate::yxml_fragment::YXmlFragment;
use crate::{YTransaction, YXmlElement};
use lib0::any::Any;
use magnus::{Error, IntoValue, RHash, Value};
use std::cell::RefCell;
use yrs::{GetString, Text, Xml, XmlNode, XmlTextRef};

#[magnus::wrap(class = "Y::XMLText")]
pub(crate) struct YXmlText(pub(crate) RefCell<XmlTextRef>);

/// SAFETY: This is safe because we only access this data when the GVL is held.
unsafe impl Send for YXmlText {}

impl YXmlText {
    pub(crate) fn yxml_text_attributes(&self, transaction: &YTransaction) -> RHash {
        let tx = transaction.transaction();
        let tx = tx.as_ref().unwrap();

        RHash::from_iter(self.0.borrow().attributes(tx))
    }
    pub(crate) fn yxml_text_format(
        &self,
        transaction: &YTransaction,
        index: u32,
        length: u32,
        attrs: RHash,
    ) -> Result<(), Error> {
        let mut tx = transaction.transaction();
        let tx = tx.as_mut().unwrap();

        map_rhash_to_attrs(attrs).map(|a| self.0.borrow_mut().format(tx, index, length, a))
    }
    pub(crate) fn yxml_text_get_attribute(
        &self,
        transaction: &YTransaction,
        name: String,
    ) -> Option<String> {
        let tx = transaction.transaction();
        let tx = tx.as_ref().unwrap();

        self.0.borrow().get_attribute(tx, name.as_str())
    }
    pub(crate) fn yxml_text_insert(&self, transaction: &YTransaction, index: u32, content: String) {
        let mut tx = transaction.transaction();
        let tx = tx.as_mut().unwrap();

        self.0.borrow_mut().insert(tx, index, content.as_str())
    }
    pub(crate) fn yxml_text_insert_attribute(
        &self,
        transaction: &YTransaction,
        name: String,
        value: String,
    ) {
        let mut tx = transaction.transaction();
        let tx = tx.as_mut().unwrap();

        self.0.borrow_mut().insert_attribute(tx, name, value)
    }
    pub(crate) fn yxml_text_insert_embed_with_attributes(
        &self,
        transaction: &YTransaction,
        index: u32,
        content: Value,
        attrs: RHash,
    ) -> Result<(), Error> {
        let mut tx = transaction.transaction();
        let tx = tx.as_mut().unwrap();

        let yvalue = YValue::from(content);
        let avalue = Any::from(yvalue);

        map_rhash_to_attrs(attrs)
            .map(|a| {
                self.0
                    .borrow_mut()
                    .insert_embed_with_attributes(tx, index, avalue, a)
            })
            .map(|_| ())
    }
    pub(crate) fn yxml_text_insert_embed(
        &self,
        transaction: &YTransaction,
        index: u32,
        embed: Value,
    ) {
        let mut tx = transaction.transaction();
        let tx = tx.as_mut().unwrap();

        self.0
            .borrow_mut()
            .insert_embed(tx, index, Any::from(YValue::from(embed)));
    }
    pub(crate) fn yxml_text_insert_with_attributes(
        &self,
        transaction: &YTransaction,
        index: u32,
        content: String,
        attrs: RHash,
    ) -> Result<(), Error> {
        let mut tx = transaction.transaction();
        let tx = tx.as_mut().unwrap();

        map_rhash_to_attrs(attrs).map(|a| {
            self.0
                .borrow_mut()
                .insert_with_attributes(tx, index, content.as_str(), a);
        })
    }
    pub(crate) fn yxml_text_length(&self, transaction: &YTransaction) -> u32 {
        let tx = transaction.transaction();
        let tx = tx.as_ref().unwrap();

        self.0.borrow().len(tx)
    }
    pub(crate) fn yxml_text_next_sibling(&self, transaction: &YTransaction) -> Option<Value> {
        let tx = transaction.transaction();
        let tx = tx.as_ref().unwrap();

        self.0.borrow().siblings(tx).next().map(|item| match item {
            XmlNode::Element(el) => YXmlElement(RefCell::from(el)).into_value(),
            XmlNode::Fragment(fragment) => YXmlFragment(RefCell::from(fragment)).into_value(),
            XmlNode::Text(text) => YXmlText(RefCell::from(text)).into_value(),
        })
    }
    pub(crate) fn yxml_text_parent(&self) -> Option<Value> {
        self.0.borrow().parent().map(|item| match item {
            XmlNode::Element(el) => YXmlElement(RefCell::from(el)).into_value(),
            XmlNode::Fragment(fragment) => YXmlFragment(RefCell::from(fragment)).into_value(),
            XmlNode::Text(text) => YXmlText(RefCell::from(text)).into_value(),
        })
    }
    pub(crate) fn yxml_text_prev_sibling(&self, transaction: &YTransaction) -> Option<Value> {
        let tx = transaction.transaction();
        let tx = tx.as_ref().unwrap();

        self.0
            .borrow()
            .siblings(tx)
            .next_back()
            .map(|item| match item {
                XmlNode::Element(el) => YXmlElement(RefCell::from(el)).into_value(),
                XmlNode::Fragment(fragment) => YXmlFragment(RefCell::from(fragment)).into_value(),
                XmlNode::Text(text) => YXmlText(RefCell::from(text)).into_value(),
            })
    }
    pub(crate) fn yxml_text_push(&self, transaction: &YTransaction, content: String) {
        let mut tx = transaction.transaction();
        let tx = tx.as_mut().unwrap();

        self.0.borrow_mut().push(tx, content.as_str())
    }
    pub(crate) fn yxml_text_remove_range(
        &self,
        transaction: &YTransaction,
        index: u32,
        length: u32,
    ) {
        let mut tx = transaction.transaction();
        let tx = tx.as_mut().unwrap();

        self.0.borrow_mut().remove_range(tx, index, length)
    }
    pub(crate) fn yxml_text_to_s(&self, transaction: &YTransaction) -> String {
        let tx = transaction.transaction();
        let tx = tx.as_ref().unwrap();

        self.0.borrow().get_string(tx)
    }
}

impl From<XmlTextRef> for YXmlText {
    fn from(v: XmlTextRef) -> Self {
        YXmlText(RefCell::from(v))
    }
}
