use crate::ytransaction::YTransaction;
use crate::yxml_element::YXmlElement;
use crate::yxml_text::YXmlText;
use magnus::{IntoValue, RArray, Value};
use std::cell::RefCell;
use yrs::{GetString, XmlElementPrelim, XmlFragment, XmlFragmentRef, XmlNode};

#[magnus::wrap(class = "Y::XMLFragment")]
pub(crate) struct YXmlFragment(pub(crate) RefCell<XmlFragmentRef>);

/// SAFETY: This is safe because we only access this data when the GVL is held.
unsafe impl Send for YXmlFragment {}

impl YXmlFragment {
    pub(crate) fn yxml_fragment_first_child(&self) -> Option<Value> {
        self.0.borrow().first_child().map(|node| match node {
            XmlNode::Element(element) => YXmlElement::from(element).into_value(),
            XmlNode::Fragment(fragment) => YXmlFragment::from(fragment).into_value(),
            XmlNode::Text(text) => YXmlText::from(text).into_value(),
        })
    }

    pub(crate) fn yxml_fragment_get(
        &self,
        transaction: &YTransaction,
        index: u32,
    ) -> Option<Value> {
        let tx = transaction.transaction();
        let tx = tx.as_ref().unwrap();

        self.0.borrow().get(tx, index).map(|node| match node {
            XmlNode::Element(element) => YXmlElement::from(element).into_value(),
            XmlNode::Fragment(fragment) => YXmlFragment::from(fragment).into_value(),
            XmlNode::Text(text) => YXmlText::from(text).into_value(),
        })
    }

    pub(crate) fn yxml_fragment_insert(
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

    pub(crate) fn yxml_fragment_len(&self, transaction: &YTransaction) -> u32 {
        let tx = transaction.transaction();
        let tx = tx.as_ref().unwrap();

        self.0.borrow().len(tx)
    }

    pub(crate) fn yxml_fragment_parent(&self) -> Option<Value> {
        self.0.borrow().parent().map(|item| match item {
            XmlNode::Element(el) => YXmlElement::from(el).into_value(),
            XmlNode::Fragment(fragment) => YXmlFragment::from(fragment).into_value(),
            XmlNode::Text(text) => YXmlText::from(text).into_value(),
        })
    }

    pub(crate) fn yxml_fragment_push_back(
        &self,
        transaction: &YTransaction,
        tag: String,
    ) -> YXmlElement {
        let mut tx = transaction.transaction();
        let tx = tx.as_mut().unwrap();

        let node = XmlElementPrelim::empty(tag);
        YXmlElement::from(self.0.borrow_mut().push_back(tx, node))
    }

    pub(crate) fn yxml_fragment_push_front(
        &self,
        transaction: &YTransaction,
        tag: String,
    ) -> YXmlElement {
        let mut tx = transaction.transaction();
        let tx = tx.as_mut().unwrap();

        let node = XmlElementPrelim::empty(tag);
        YXmlElement::from(self.0.borrow_mut().push_front(tx, node))
    }

    pub(crate) fn yxml_fragment_remove_range(
        &self,
        transaction: &YTransaction,
        index: u32,
        length: u32,
    ) {
        let mut tx = transaction.transaction();
        let tx = tx.as_mut().unwrap();

        self.0.borrow_mut().remove_range(tx, index, length);
    }

    pub(crate) fn yxml_fragment_successors(&self, transaction: &YTransaction) -> RArray {
        let tx = transaction.transaction();
        let tx = tx.as_ref().unwrap();

        let fragment = self.0.borrow();

        let result = fragment.successors(tx).map(|item| match item {
            XmlNode::Element(el) => YXmlElement::from(el).into_value(),
            XmlNode::Fragment(fragment) => YXmlFragment::from(fragment).into_value(),
            XmlNode::Text(text) => YXmlText::from(text).into_value(),
        });

        RArray::from_iter(result)
    }

    pub(crate) fn yxml_fragment_to_s(&self, transaction: &YTransaction) -> String {
        let tx = transaction.transaction();
        let tx = tx.as_ref().unwrap();

        self.0.borrow().get_string(tx)
    }
}

impl From<XmlFragmentRef> for YXmlFragment {
    fn from(v: XmlFragmentRef) -> Self {
        YXmlFragment(RefCell::from(v))
    }
}
