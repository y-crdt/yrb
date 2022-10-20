use crate::YTransaction;
use magnus::{Error, Integer, Value};
use std::cell::RefCell;
use yrs::updates::decoder::Decode;
use yrs::{Doc, OffsetKind, Options, StateVector};

#[magnus::wrap(class = "Y::Doc")]
pub(crate) struct YDoc(pub(crate) RefCell<Doc>);

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

    pub(crate) fn ydoc_transact(&self) -> YTransaction {
        let transaction = self.0.borrow().transact();

        YTransaction(RefCell::new(transaction))
    }
    pub(crate) fn ydoc_encode_diff_v1(&self, state_vector: Vec<u8>) -> Result<Vec<u8>, Error> {
        StateVector::decode_v1(&*state_vector)
            .map(|sv| self.0.borrow().encode_state_as_update_v1(&sv))
            .map_err(|_e| Error::runtime_error("cannot encode diff"))
    }
}
