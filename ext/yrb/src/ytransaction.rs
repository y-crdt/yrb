use crate::yarray::YArray;
use crate::ymap::YMap;
use crate::ytext::YText;
use crate::yxml_element::YXmlElement;
use crate::yxml_fragment::YXmlFragment;
use crate::yxml_text::YXmlText;
use magnus::{exception, Error};
use std::cell::{RefCell, RefMut};
use yrs::updates::decoder::Decode;
use yrs::updates::encoder::Encode;
use yrs::{ReadTxn, TransactionMut, Update};

#[magnus::wrap(class = "Y::Transaction")]
pub(crate) struct YTransaction(pub(crate) RefCell<Option<TransactionMut<'static>>>);

/// SAFETY: This is safe because we only access this data when the GVL is held.
unsafe impl Send for YTransaction {}

impl YTransaction {}

impl<'doc> From<TransactionMut<'doc>> for YTransaction {
    fn from(txn: TransactionMut<'doc>) -> Self {
        let txn: TransactionMut<'static> = unsafe { std::mem::transmute(txn) };
        YTransaction(RefCell::from(Some(txn)))
    }
}

// API which is eventually publicly exposed
impl YTransaction {
    pub(crate) fn ytransaction_apply_update(&self, update: Vec<u8>) -> Result<(), Error> {
        Update::decode_v1(update.as_slice())
            .map_err(|error| {
                Error::new(
                    exception::runtime_error(),
                    format!("cannot decode update: {:?}", error),
                )
            })
            .map(|u| self.transaction().as_mut().unwrap().apply_update(u))
    }

    pub(crate) fn ytransaction_commit(&self) {
        self.transaction().as_mut().unwrap().commit();
    }

    pub(crate) fn ytransaction_get_array(&self, name: String) -> Option<YArray> {
        self.transaction()
            .as_ref()
            .unwrap()
            .get_array(name.as_str())
            .map(YArray::from)
    }

    pub(crate) fn ytransaction_get_map(&self, name: String) -> Option<YMap> {
        self.transaction()
            .as_ref()
            .unwrap()
            .get_map(name.as_str())
            .map(YMap::from)
    }

    pub(crate) fn ytransaction_get_text(&self, name: String) -> Option<YText> {
        self.transaction()
            .as_ref()
            .unwrap()
            .get_text(name.as_str())
            .map(YText::from)
    }

    pub(crate) fn ytransaction_get_xml_element(&self, name: String) -> Option<YXmlElement> {
        self.transaction()
            .as_ref()
            .unwrap()
            .get_xml_element(name.as_str())
            .map(YXmlElement::from)
    }

    pub(crate) fn ytransaction_get_xml_fragment(&self, name: String) -> Option<YXmlFragment> {
        self.transaction()
            .as_ref()
            .unwrap()
            .get_xml_fragment(name.as_str())
            .map(YXmlFragment::from)
    }

    pub(crate) fn ytransaction_get_xml_text(&self, name: String) -> Option<YXmlText> {
        self.transaction()
            .as_ref()
            .unwrap()
            .get_xml_text(name.as_str())
            .map(YXmlText::from)
    }

    pub(crate) fn ytransaction_state_vector(&self) -> Vec<u8> {
        self.transaction()
            .as_ref()
            .unwrap()
            .state_vector()
            .encode_v1()
    }

    pub(crate) fn ytransaction_free(&self) {
        self.0.replace(None);
    }

    pub(crate) fn transaction(&self) -> RefMut<'_, Option<TransactionMut<'static>>> {
        self.0.borrow_mut()
    }
}
