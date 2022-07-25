use std::cell::{RefCell};
use std::marker::Destruct;
use magnus::{DataType, Error, RClass, TryConvert, TypedData, Value};
use yrs::{Transaction, Update};
use yrs::updates::decoder::Decode;
use yrs::updates::encoder::Encode;
use crate::yarray::YArray;
use crate::ytext::YText;

#[magnus::wrap(class = "Y::Transaction")]
pub(crate) struct YTransaction(pub(crate) RefCell<Transaction>);

impl YTransaction {
    pub(crate) fn ytransaction_apply_update(&self, update: Vec<u8>) -> Result<(), Error> {
        return Update::decode_v1(&*update)
            .map(|u| self.0
                .borrow_mut()
                .apply_update(u)
            ).map_err(|e| Error::runtime_error("cannot apply update"));
    }
    pub(crate) fn ytransaction_commit(&self) {
        self.0
            .borrow_mut()
            .commit();
    }
    pub(crate) fn ytransaction_get_array(&self, name: String) -> YArray {
        let a = self.0
            .borrow_mut()
            .get_array(&*name);

        return YArray(RefCell::from(a));
    }
    pub(crate) fn ytransaction_get_text(&self, name: String) -> YText {
        let t = self.0
            .borrow_mut()
            .get_text(&*name);

        return YText(RefCell::new(t));
    }
    pub(crate) fn ytransaction_state_vector(&self) -> Vec<u8> {
        return self.0
            .borrow_mut()
            .state_vector()
            .encode_v1();
    }
}

impl TryConvert for YTransaction {
    fn try_convert(val: &Value) -> Result<Self, Error> {
        todo!()
    }
}
