use crate::yarray::YArray;
use crate::ymap::YMap;
use crate::ytext::YText;
use crate::yxml_element::YXmlElement;
use crate::yxml_fragment::YXmlFragment;
use crate::yxml_text::YXmlText;
use crate::YTransaction;
use magnus::block::Proc;
use magnus::{exception::runtime_error, Error, Integer, RArray, Value};
use std::borrow::Borrow;
use std::cell::RefCell;
use yrs::updates::decoder::Decode;
use yrs::updates::encoder::{Encoder, EncoderV2};
use yrs::{Doc, OffsetKind, Options, ReadTxn, StateVector, SubscriptionId, Transact};

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
        options.offset_kind = OffsetKind::Utf16;

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
            .map_err(|_e| Error::new(runtime_error(), "cannot encode diff"))
    }

    pub(crate) fn ydoc_encode_diff_v2(
        &self,
        transaction: &YTransaction,
        state_vector: Vec<u8>,
    ) -> Result<Vec<u8>, Error> {
        let mut tx = transaction.transaction();
        let tx = tx.as_mut().unwrap();
        let mut encoder = EncoderV2::new();

        StateVector::decode_v2(state_vector.borrow())
            .map(|sv| tx.encode_diff(&sv, &mut encoder))
            .map(|_| encoder.to_vec())
            .map_err(|_e| Error::new(runtime_error(), "cannot encode diff"))
    }

    pub(crate) fn ydoc_get_or_insert_array(&self, name: String) -> YArray {
        let array_ref = self.0.borrow().get_or_insert_array(name.as_str());
        YArray::from(array_ref)
    }

    pub(crate) fn ydoc_get_or_insert_map(&self, name: String) -> YMap {
        let map_ref = self.0.borrow().get_or_insert_map(name.as_str());
        YMap::from(map_ref)
    }

    pub(crate) fn ydoc_get_or_insert_text(&self, name: String) -> YText {
        let text_ref = self.0.borrow().get_or_insert_text(name.as_str());
        YText::from(text_ref)
    }

    pub(crate) fn ydoc_get_or_insert_xml_element(&self, name: String) -> YXmlElement {
        let xml_element_ref = self.0.borrow_mut().get_or_insert_xml_element(name.as_str());
        YXmlElement::from(xml_element_ref) // ::into() maps to YXmlFragment instead of YXmlElement :-(
    }

    pub(crate) fn ydoc_get_or_insert_xml_fragment(&self, name: String) -> YXmlFragment {
        let xml_fragment_ref = self.0.borrow().get_or_insert_xml_fragment(name.as_str());
        YXmlFragment::from(xml_fragment_ref)
    }

    pub(crate) fn ydoc_get_or_insert_xml_text(&self, name: String) -> YXmlText {
        let xml_text_ref = self.0.borrow().get_or_insert_xml_text(name.as_str());
        YXmlText::from(xml_text_ref)
    }

    pub(crate) fn ydoc_transact(&self) -> YTransaction {
        let doc = self.0.borrow();
        let transaction = doc.transact_mut();
        YTransaction::from(transaction)
    }

    pub(crate) fn ydoc_observe_update(&self, block: Proc) -> Result<SubscriptionId, Error> {
        self.0
            .borrow()
            .observe_update_v1(move |_tx, update_event| {
                let update = update_event.update.to_vec();
                let update = RArray::from_vec(update);

                let args: (RArray,) = (update,);
                block
                    .call::<(RArray,), Value>(args)
                    .expect("cannot call update block");
            })
            .map(|v| v.into())
            .map_err(|err| Error::new(runtime_error(), err.to_string()))
    }
}
