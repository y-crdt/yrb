use crate::yvalue::YValue;
use crate::yxml_text::YXmlText;
use crate::YTransaction;
use magnus::block::Proc;
use magnus::{Error, RArray, RHash, Symbol, Value};
use std::cell::RefCell;
use yrs::types::Change;
use yrs::{Xml, XmlElement};

#[magnus::wrap(class = "Y::XMLElement")]
pub(crate) struct YXmlElement(pub(crate) RefCell<XmlElement>);

/// SAFETY: This is safe because we only access this data when the GVL is held.
unsafe impl Send for YXmlElement {}

impl YXmlElement {
    pub(crate) fn yxml_element_attributes(&self) -> RHash {
        RHash::from_iter(self.0.borrow().attributes().into_iter())
    }
    pub(crate) fn yxml_element_first_child(&self) -> Option<Value> {
        self.yxml_element_get(0)
    }
    pub(crate) fn yxml_element_get(&self, index: u32) -> Option<Value> {
        self.0.borrow().get(index).map(|node| match node {
            Xml::Element(el) => Value::from(YXmlElement(RefCell::from(el))),
            Xml::Text(text) => Value::from(YXmlText(RefCell::from(text)))
        })
    }
    pub(crate) fn yxml_element_get_attribute(
        &self,
        name: String
    ) -> Option<String> {
        self.0.borrow().get_attribute(&*name)
    }
    pub(crate) fn yxml_element_insert_attribute(
        &self,
        transaction: &YTransaction,
        name: String,
        value: String
    ) {
        self.0.borrow_mut().insert_attribute(
            &mut *transaction.0.borrow_mut(),
            name,
            value
        );
    }
    pub(crate) fn yxml_element_insert_element(
        &self,
        transaction: &YTransaction,
        index: u32,
        name: String
    ) {
        self.0.borrow_mut().insert_elem(
            &mut *transaction.0.borrow_mut(),
            index,
            name
        );
    }
    pub(crate) fn yxml_element_insert_text(
        &self,
        transaction: &YTransaction,
        index: u32
    ) {
        self.0
            .borrow_mut()
            .insert_text(&mut *transaction.0.borrow_mut(), index);
    }
    pub(crate) fn yxml_element_next_sibling(&self) -> Option<Value> {
        self.0.borrow().next_sibling().map(|item| match item {
            Xml::Element(el) => Value::from(YXmlElement(RefCell::from(el))),
            Xml::Text(text) => Value::from(YXmlText(RefCell::from(text)))
        })
    }
    pub(crate) fn yxml_element_observe(
        &self,
        block: Proc
    ) -> Result<u32, Error> {
        let change_added = Symbol::new("added").to_static();
        let change_retain = Symbol::new("retain").to_static();
        let change_removed = Symbol::new("removed").to_static();

        let subscription_id = self.0.borrow_mut().observe(
            move |transaction, xml_element_event| {
                let delta = xml_element_event.delta(transaction);
                let changes = RArray::with_capacity(delta.len());

                for change in delta {
                    match change {
                        Change::Added(v) => {
                            let values = v
                                .iter()
                                .map(|o| YValue::from(o.clone()))
                                .map(|o| *o.0.borrow())
                                .collect::<Vec<_>>();

                            let payload = RHash::new();
                            payload
                                .aset(change_added, RArray::from_vec(values))
                                .expect("cannot create change::added payload");

                            changes.push(payload).expect(
                                "cannot push payload to list of changes"
                            );
                        }
                        Change::Retain(position) => {
                            let payload = RHash::new();
                            payload
                                .aset(change_retain, *position)
                                .expect("cannot create change::retain payload");

                            changes.push(payload).expect(
                                "cannot push payload to list of changes"
                            );
                        }
                        Change::Removed(position) => {
                            let payload = RHash::new();
                            payload.aset(change_removed, *position).expect(
                                "cannot create change::removed payload"
                            );

                            changes.push(payload).expect(
                                "cannot push payload to list of changes"
                            );
                        }
                    }
                }

                block
                    .call::<(RArray,), Value>((changes,))
                    .expect("cannot call block");
            }
        );

        Ok(subscription_id.into())
    }
    pub(crate) fn yxml_element_parent(&self) -> Option<Value> {
        self.0
            .borrow()
            .parent()
            .map(|item| Value::from(YXmlElement(RefCell::from(item))))
    }
    pub(crate) fn yxml_element_prev_sibling(&self) -> Option<Value> {
        self.0.borrow().prev_sibling().map(|item| match item {
            Xml::Element(el) => Value::from(YXmlElement(RefCell::from(el))),
            Xml::Text(text) => Value::from(YXmlText(RefCell::from(text)))
        })
    }
    pub(crate) fn yxml_element_push_element_back(
        &self,
        transaction: &YTransaction,
        name: String
    ) -> YXmlElement {
        let xml_element = self
            .0
            .borrow_mut()
            .push_elem_back(&mut *transaction.0.borrow_mut(), name);

        YXmlElement(RefCell::from(xml_element))
    }
    pub(crate) fn yxml_element_push_element_front(
        &self,
        transaction: &YTransaction,
        name: String
    ) -> YXmlElement {
        let xml_element = self
            .0
            .borrow_mut()
            .push_elem_front(&mut *transaction.0.borrow_mut(), name);

        YXmlElement(RefCell::from(xml_element))
    }
    pub(crate) fn yxml_element_push_text_back(
        &self,
        transaction: &YTransaction
    ) -> YXmlText {
        let xml_text = self
            .0
            .borrow_mut()
            .push_text_back(&mut *transaction.0.borrow_mut());

        YXmlText(RefCell::from(xml_text))
    }
    pub(crate) fn yxml_element_push_text_front(
        &self,
        transaction: &YTransaction
    ) -> YXmlText {
        let xml_text = self
            .0
            .borrow_mut()
            .push_text_front(&mut *transaction.0.borrow_mut());

        YXmlText(RefCell::from(xml_text))
    }
    pub(crate) fn yxml_element_remove_attribute(
        &self,
        transaction: &YTransaction,
        name: String
    ) {
        self.0.borrow_mut().remove_attribute::<&str>(
            &mut *transaction.0.borrow_mut(),
            &name.as_str()
        );
    }
    pub(crate) fn yxml_element_remove_range(
        &self,
        transaction: &YTransaction,
        index: u32,
        length: u32
    ) {
        self.0.borrow_mut().remove_range(
            &mut *transaction.0.borrow_mut(),
            index,
            length
        );
    }
    pub(crate) fn yxml_element_size(&self) -> u32 {
        self.0.borrow().len()
    }
    pub(crate) fn yxml_element_tag(&self) -> String {
        self.0.borrow().tag().to_string()
    }
    pub(crate) fn yxml_element_to_s(&self) -> String {
        self.0.borrow().to_string()
    }
    pub(crate) fn yxml_element_unobserve(&self, subscription_id: u32) {
        self.0.borrow_mut().unobserve(subscription_id);
    }
}
