use std::cell::RefCell;
use magnus::{Value};
use yrs::{Text};
use crate::YTransaction;

#[magnus::wrap(class = "Y::Text")]
pub(crate) struct YText(pub(crate) RefCell<Text>);

impl YText {
    pub(crate) fn ytext_new(text: Text) -> Self {
        Self(RefCell::new(text))
    }
    pub(crate) fn ytext_insert(&self, transaction: &YTransaction, index: u32, chunk: String) {
        self.0
            .borrow_mut()
            .insert(&mut *transaction.0.borrow_mut(), index, &*chunk);
    }
    pub(crate) fn ytext_push(&self, transaction: &YTransaction, chunk: String) {
        self.0
            .borrow_mut()
            .push(&mut *transaction.0.borrow_mut(), &*chunk);
    }
    pub(crate) fn ytext_to_s(&self) -> String {
        return self.0
            .borrow_mut()
            .to_string();
    }
}
