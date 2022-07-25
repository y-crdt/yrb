use std::cell::RefCell;
use magnus::block::Proc;
use magnus::{Error};
use yrs::{Array};

#[magnus::wrap(class = "Y::Array")]
pub(crate) struct YArray(pub(crate) RefCell<Array>);

impl YArray {
    pub(crate) fn yarray_each(&self, block: Proc) -> () {
        self.0
            .borrow_mut()
            .iter()
            .for_each(|val| {
                let args = (1,);
                // if let Err(e) = block.call(args) { Error::from(e); }
            });
    }
}
