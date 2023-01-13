use crate::yarray::YArray;
use crate::ymap::YMap;
use crate::ytext::YText;
use crate::yxml_element::YXmlElement;
use crate::yxml_fragment::YXmlFragment;
use crate::yxml_text::YXmlText;
use crate::YTransaction;
use magnus::{exception, Error, Integer, Value};
use std::borrow::Borrow;
use std::cell::RefCell;
use yrs::updates::decoder::Decode;
use yrs::{Doc, OffsetKind, Options, ReadTxn, StateVector, Transact};

#[magnus::wrap(class = "Y::Doc")]
pub(crate) struct YDoc(pub(crate) RefCell<Doc>);

unsafe impl Send for YDoc {}

impl YDoc {
    pub(crate) fn ydoc_new(client_id: &[Value]) -> Self {
        let mut options = Options::default();
        if client_id.len() == 1 {
            let value = client_id.first().unwrap();
            options.client_id = Integer::from_value(*value).unwrap().to_u64().unwrap();
        }
        options.offset_kind = OffsetKind::Utf32;

        let doc = Doc::with_options(options);
        Self(RefCell::new(doc))
    }

    pub(crate) fn ydoc_encode_diff_v1(
        &self,
        transaction: &YTransaction,
        state_vector: Vec<u8>,
    ) -> Result<Vec<u8>, Error> {
        let mut tx = transaction.transaction();
        let tx = tx.as_mut().unwrap();

        StateVector::decode_v1(state_vector.borrow())
            .map(|sv| tx.encode_diff_v1(&sv))
            .map_err(|_e| Error::new(exception::runtime_error(), "cannot encode diff"))
    }

    pub(crate) fn ydoc_get_or_insert_array(&self, name: String) -> YArray {
        self.0.borrow().get_or_insert_array(name.as_str()).into()
    }

    pub(crate) fn ydoc_get_or_insert_map(&self, name: String) -> YMap {
        self.0.borrow().get_or_insert_map(name.as_str()).into()
    }

    pub(crate) fn ydoc_get_or_insert_text(&self, name: String) -> YText {
        self.0.borrow().get_or_insert_text(name.as_str()).into()
    }

    pub(crate) fn ydoc_get_or_insert_xml_element(&self, name: String) -> YXmlElement {
        let xml_element_ref = self.0.borrow_mut().get_or_insert_xml_element(name.as_str());
        YXmlElement::from(xml_element_ref) // ::into() maps to YXmlFragment instead of YXmlElement :-(
    }

    pub(crate) fn ydoc_get_or_insert_xml_fragment(&self, name: String) -> YXmlFragment {
        self.0
            .borrow()
            .get_or_insert_xml_fragment(name.as_str())
            .into()
    }

    pub(crate) fn ydoc_get_or_insert_xml_text(&self, name: String) -> YXmlText {
        self.0.borrow().get_or_insert_xml_text(name.as_str()).into()
    }

    pub(crate) fn ydoc_transact<'doc>(&self) -> YTransaction {
        let doc = self.0.borrow();
        let transaction = doc.transact_mut();
        YTransaction::from(transaction)
    }
}
