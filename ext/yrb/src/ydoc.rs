use std::cell::RefCell;
use magnus::{DataType, Error, RClass, TypedData};
use yrs::{Doc, StateVector};
use yrs::updates::decoder::Decode;
use crate::YTransaction;

#[magnus::wrap(class = "Y::Doc")]
pub(crate) struct YDoc(pub(crate) RefCell<Doc>);

impl YDoc {
    pub(crate) fn ydoc_new() -> Self {
        Self(RefCell::new(Doc::new()))
    }

    pub(crate) fn ydoc_transact(&self) -> YTransaction {
        let transaction = self.0
            .borrow()
            .transact();

        return YTransaction(RefCell::new(transaction));
    }
    pub(crate) fn ydoc_encode_diff_v1(&self, state_vector: Vec<u8>) -> Result<Vec<u8>, Error> {
        return StateVector::decode_v1(&*state_vector)
            .map(|sv| self.0
                .borrow()
                .encode_state_as_update_v1(&sv)
            )
            .map_err(|_e| Error::runtime_error("cannot encode diff"));
    }
}
