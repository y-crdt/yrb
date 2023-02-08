use crate::yvalue::YValue;
use crate::yxml_fragment::YXmlFragment;
use crate::yxml_text::YXmlText;
use crate::YTransaction;
use magnus::block::Proc;
use magnus::{Error, RArray, RHash, Symbol, Value};
use std::cell::RefCell;
use yrs::types::Change;
use yrs::{
    GetString, Observable, Xml, XmlElementPrelim, XmlElementRef, XmlFragment, XmlNode,
    XmlTextPrelim,
};

#[magnus::wrap(class = "Y::XMLElement")]
pub(crate) struct YXmlElement(pub(crate) RefCell<XmlElementRef>);

/// SAFETY: This is safe because we only access this data when the GVL is held.
unsafe impl Send for YXmlElement {}

impl YXmlElement {
    pub(crate) fn yxml_element_attributes(&self, transaction: &YTransaction) -> RHash {
        let tx = transaction.transaction();
        let tx = tx.as_ref().unwrap();

        RHash::from_iter(self.0.borrow().attributes(tx))
    }
    pub(crate) fn yxml_element_first_child(&self, transaction: &YTransaction) -> Option<Value> {
        self.yxml_element_get(transaction, 0)
    }
    pub(crate) fn yxml_element_get(&self, transaction: &YTransaction, index: u32) -> Option<Value> {
        let tx = transaction.transaction();
        let tx = tx.as_ref().unwrap();

        self.0.borrow().get(tx, index).map(|node| match node {
            XmlNode::Element(element) => Value::from(YXmlElement::from(element)),
            XmlNode::Fragment(fragment) => Value::from(YXmlFragment::from(fragment)),
            XmlNode::Text(text) => Value::from(YXmlText::from(text)),
        })
    }
    pub(crate) fn yxml_element_get_attribute(
        &self,
        transaction: &YTransaction,
        name: String,
    ) -> Option<String> {
        let tx = transaction.transaction();
        let tx = tx.as_ref().unwrap();

        self.0.borrow().get_attribute(tx, name.as_str())
    }
    pub(crate) fn yxml_element_insert_attribute(
        &self,
        transaction: &YTransaction,
        name: String,
        value: String,
    ) {
        let mut tx = transaction.transaction();
        let tx = tx.as_mut().unwrap();

        self.0.borrow_mut().insert_attribute(tx, name, value)
    }
    pub(crate) fn yxml_element_insert_element(
        &self,
        transaction: &YTransaction,
        index: u32,
        tag: String,
    ) -> YXmlElement {
        let mut tx = transaction.transaction();
        let tx = tx.as_mut().unwrap();

        let node = XmlElementPrelim::empty(tag);
        YXmlElement::from(self.0.borrow_mut().insert(tx, index, node))
    }
    pub(crate) fn yxml_element_insert_text(
        &self,
        transaction: &YTransaction,
        index: u32,
        content: String,
    ) -> YXmlText {
        let text = XmlTextPrelim::new(content.as_str());
        let mut tx = transaction.transaction();
        let tx = tx.as_mut().unwrap();

        YXmlText::from(self.0.borrow_mut().insert(tx, index, text))
    }
    pub(crate) fn yxml_element_len(&self, transaction: &YTransaction) -> u32 {
        let mut tx = transaction.transaction();
        let tx = tx.as_mut().unwrap();

        self.0.borrow().len(tx)
    }
    pub(crate) fn yxml_element_next_sibling(&self, transaction: &YTransaction) -> Option<Value> {
        let tx = transaction.transaction();
        let tx = tx.as_ref().unwrap();

        self.0.borrow().siblings(tx).next().map(|item| match item {
            XmlNode::Element(el) => Value::from(YXmlElement::from(el)),
            XmlNode::Fragment(fragment) => Value::from(YXmlFragment::from(fragment)),
            XmlNode::Text(text) => Value::from(YXmlText::from(text)),
        })
    }
    pub(crate) fn yxml_element_observe(&self, block: Proc) -> Result<u32, Error> {
        let change_added = Symbol::new("added").to_static();
        let change_retain = Symbol::new("retain").to_static();
        let change_removed = Symbol::new("removed").to_static();

        let subscription_id = self
            .0
            .borrow_mut()
            .observe(move |transaction, xml_element_event| {
                let delta = xml_element_event.delta(transaction);
                let changes = RArray::with_capacity(delta.len());

                for change in delta {
                    match change {
                        Change::Added(v) => {
                            let values = RArray::new();
                            for value in v.iter() {
                                let value = YValue::from(value.clone());
                                let value = value.0.borrow().clone();
                                values.push(value).expect("cannot push value to array");
                            }

                            let payload = RHash::new();
                            payload
                                .aset(change_added, values)
                                .expect("cannot create change::added payload");

                            changes
                                .push(payload)
                                .expect("cannot push payload to list of changes");
                        }
                        Change::Retain(position) => {
                            let payload = RHash::new();
                            payload
                                .aset(change_retain, *position)
                                .expect("cannot create change::retain payload");

                            changes
                                .push(payload)
                                .expect("cannot push payload to list of changes");
                        }
                        Change::Removed(position) => {
                            let payload = RHash::new();
                            payload
                                .aset(change_removed, *position)
                                .expect("cannot create change::removed payload");

                            changes
                                .push(payload)
                                .expect("cannot push payload to list of changes");
                        }
                    }
                }

                block
                    .call::<(RArray,), Value>((changes,))
                    .expect("cannot call block");
            });

        Ok(subscription_id.into())
    }
    pub(crate) fn yxml_element_parent(&self) -> Option<Value> {
        self.0.borrow().parent().map(|item| match item {
            XmlNode::Element(el) => Value::from(YXmlElement::from(el)),
            XmlNode::Fragment(fragment) => Value::from(YXmlFragment::from(fragment)),
            XmlNode::Text(text) => Value::from(YXmlText::from(text)),
        })
    }
    pub(crate) fn yxml_element_prev_sibling(&self, transaction: &YTransaction) -> Option<Value> {
        let tx = transaction.transaction();
        let tx = tx.as_ref().unwrap();

        self.0
            .borrow()
            .siblings(tx)
            .next_back()
            .map(|item| match item {
                XmlNode::Element(el) => Value::from(YXmlElement::from(el)),
                XmlNode::Fragment(fragment) => Value::from(YXmlFragment::from(fragment)),
                XmlNode::Text(text) => Value::from(YXmlText::from(text)),
            })
    }
    pub(crate) fn yxml_element_push_element_back(
        &self,
        transaction: &YTransaction,
        tag: String,
    ) -> YXmlElement {
        let mut tx = transaction.transaction();
        let tx = tx.as_mut().unwrap();

        let node = XmlElementPrelim::empty(tag);
        YXmlElement::from(self.0.borrow_mut().push_back(tx, node))
    }
    pub(crate) fn yxml_element_push_element_front(
        &self,
        transaction: &YTransaction,
        tag: String,
    ) -> YXmlElement {
        let mut tx = transaction.transaction();
        let tx = tx.as_mut().unwrap();

        let node = XmlElementPrelim::empty(tag);
        YXmlElement::from(self.0.borrow_mut().push_front(tx, node))
    }
    pub(crate) fn yxml_element_push_text_back(
        &self,
        transaction: &YTransaction,
        content: String,
    ) -> YXmlText {
        let mut tx = transaction.transaction();
        let tx = tx.as_mut().unwrap();

        let text = XmlTextPrelim::new(content.as_str());
        YXmlText::from(self.0.borrow_mut().push_back(tx, text))
    }
    pub(crate) fn yxml_element_push_text_front(
        &self,
        transaction: &YTransaction,
        content: String,
    ) -> YXmlText {
        let mut tx = transaction.transaction();
        let tx = tx.as_mut().unwrap();

        let text = XmlTextPrelim::new(content.as_str());
        YXmlText::from(self.0.borrow_mut().push_front(tx, text))
    }
    pub(crate) fn yxml_element_remove_attribute(&self, transaction: &YTransaction, name: String) {
        let mut tx = transaction.transaction();
        let tx = tx.as_mut().unwrap();

        self.0.borrow_mut().remove_attribute(tx, &name)
    }
    pub(crate) fn yxml_element_remove_range(
        &self,
        transaction: &YTransaction,
        index: u32,
        length: u32,
    ) {
        let mut tx = transaction.transaction();
        let tx = tx.as_mut().unwrap();

        self.0.borrow_mut().remove_range(tx, index, length)
    }
    pub(crate) fn yxml_element_siblings(&self, transaction: &YTransaction) -> RArray {
        let tx = transaction.transaction();
        let tx = tx.as_ref().unwrap();

        let siblings = self.0.borrow().siblings(tx).map(|item| match item {
            XmlNode::Element(el) => Value::from(YXmlElement::from(el)),
            XmlNode::Fragment(fragment) => Value::from(YXmlFragment::from(fragment)),
            XmlNode::Text(text) => Value::from(YXmlText::from(text)),
        });

        RArray::from_iter(siblings)
    }
    pub(crate) fn yxml_element_size(&self, transaction: &YTransaction) -> u32 {
        let tx = transaction.transaction();
        let tx = tx.as_ref().unwrap();

        self.0.borrow().len(tx)
    }
    pub(crate) fn yxml_element_tag(&self) -> String {
        self.0.borrow().tag().to_string()
    }
    pub(crate) fn yxml_element_to_s(&self, transaction: &YTransaction) -> String {
        let tx = transaction.transaction();
        let tx = tx.as_ref().unwrap();

        self.0.borrow().get_string(tx)
    }
    pub(crate) fn yxml_element_unobserve(&self, subscription_id: u32) {
        self.0.borrow_mut().unobserve(subscription_id);
    }
}

impl From<XmlElementRef> for YXmlElement {
    fn from(v: XmlElementRef) -> Self {
        YXmlElement(RefCell::from(v))
    }
}
