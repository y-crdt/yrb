use std::cell::{RefCell};
use std::ops::{Deref, DerefMut};
use magnus::{Error, TryConvert, Value};
use yrs::{Transaction, Update};
use yrs::updates::decoder::Decode;
use yrs::updates::encoder::Encode;
use crate::yarray::YArray;
use crate::ymap::YMap;
use crate::ytext::YText;
use crate::yxml_element::YXmlElement;
use crate::yxml_text::YXmlText;

#[magnus::wrap(class = "Y::Transaction")]
pub(crate) struct YTransaction(pub(crate) RefCell<Transaction>);

impl YTransaction {
    pub(crate) fn ytransaction_apply_update(&self, update: Vec<u8>) -> Result<(), Error> {
        return Update::decode_v1(update.as_slice())
            .map(|u| self.0.borrow_mut()
                .apply_update(u)
            ).map_err(|_e| Error::runtime_error("cannot apply update"));
    }
    pub(crate) fn ytransaction_commit(&self) {
        self.0.borrow_mut()
            .commit();
    }
    pub(crate) fn ytransaction_get_array(&self, name: String) -> YArray {
        let a = self.0.borrow_mut()
            .get_array(&*name);

        return YArray(RefCell::from(a));
    }
    pub(crate) fn ytransaction_get_map(&self, name: String) -> YMap {
        let m = self.0.borrow_mut()
            .get_map(&*name);

        return YMap(RefCell::from(m));
    }
    pub(crate) fn ytransaction_get_text(&self, name: String) -> YText {
        let t = self.0.borrow_mut()
            .get_text(&*name);

        return YText(RefCell::new(t));
    }
    pub(crate) fn ytransaction_get_xml_element(&self, name: String) -> YXmlElement {
        let el = self.0.borrow_mut()
            .get_xml_element(&*name);

        return YXmlElement(RefCell::new(el));
    }
    pub(crate) fn ytransaction_get_xml_text(&self, name: String) -> YXmlText {
        let t = self.0.borrow_mut()
            .get_xml_text(&*name);

        return YXmlText(RefCell::new(t));
    }
    pub(crate) fn ytransaction_state_vector(&self) -> Vec<u8> {
        return self.0.borrow_mut()
            .state_vector()
            .encode_v1();
    }
}
