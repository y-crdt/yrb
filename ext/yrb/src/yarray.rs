use crate::utils::convert_yvalue_to_ruby_value;
use crate::ytransaction::YTransaction;
use crate::yvalue::YValue;
use magnus::block::Proc;
use magnus::value::Qnil;
use magnus::{Error, IntoValue, RArray, RHash, Symbol, Value};
use std::cell::RefCell;
use yrs::types::Change;
use yrs::{Any, Array, ArrayRef, Observable};

#[magnus::wrap(class = "Y::Array")]
pub(crate) struct YArray(pub(crate) RefCell<ArrayRef>);

/// SAFETY: This is safe because we only access this data when the GVL is held.
unsafe impl Send for YArray {}

impl YArray {
    pub(crate) fn yarray_each(&self, transaction: &YTransaction, block: Proc) -> Result<(), Error> {
        let tx = transaction.transaction();
        let tx = tx.as_ref().unwrap();

        let arr = self.0.borrow();
        arr.iter(tx).for_each(|val| {
            let yvalue = *convert_yvalue_to_ruby_value(val, tx).0.borrow();
            let args = (yvalue,);
            let _ = block.call::<(Value,), Qnil>(args);
        });

        Ok(())
    }

    pub(crate) fn yarray_get(&self, transaction: &YTransaction, index: u32) -> Value {
        let tx = transaction.transaction();
        let tx = tx.as_ref().unwrap();

        let arr = self.0.borrow();
        let v = arr.get(tx, index).unwrap();
        *convert_yvalue_to_ruby_value(v, tx).0.borrow()
    }
    pub(crate) fn yarray_insert(&self, transaction: &YTransaction, index: u32, value: Value) {
        let yvalue = YValue::from(value);
        let avalue = Any::from(yvalue);

        let mut tx = transaction.transaction();
        let tx = tx.as_mut().unwrap();

        let arr = self.0.borrow_mut();
        arr.insert(tx, index, avalue);
    }
    pub(crate) fn yarray_insert_range(
        &self,
        transaction: &YTransaction,
        index: u32,
        values: RArray,
    ) {
        let arr = self.0.borrow_mut();
        let mut tx = transaction.transaction();
        let tx = tx.as_mut().unwrap();

        let add_values: Vec<Any> = values
            .each()
            .into_iter()
            .map(|value| YValue::from(value.unwrap()).into())
            .collect();

        arr.insert_range(tx, index, add_values)
    }
    pub(crate) fn yarray_length(&self, transaction: &YTransaction) -> u32 {
        let arr = self.0.borrow();
        let tx = transaction.transaction();
        let tx = tx.as_ref().unwrap();

        arr.len(tx)
    }
    pub(crate) fn yarray_observe(&self, block: Proc) -> Result<u32, Error> {
        let change_added = Symbol::new("added").to_static();
        let change_retain = Symbol::new("retain").to_static();
        let change_removed = Symbol::new("removed").to_static();

        // let mut error: Option<Error> = None;

        let subscription_id = self
            .0
            .borrow_mut()
            .observe(move |transaction, array_event| {
                let delta = array_event.delta(transaction);
                // let mut changes = RArray::with_capacity(delta.len());
                let (changes, errors): (Vec<_>, Vec<_>) = delta
                    .iter()
                    .map(|change| {
                        let payload = RHash::new();
                        let result = match change {
                            Change::Added(v) => {
                                let values = v
                                    .iter()
                                    .map(|v| <YValue as Into<Value>>::into(YValue::from(v.clone())))
                                    .collect::<RArray>();
                                payload.aset(change_added, values)
                            }
                            Change::Retain(position) => {
                                payload.aset(change_retain, (*position).into_value())
                            }
                            Change::Removed(position) => {
                                payload.aset(change_removed, (*position).into_value())
                            }
                        };

                        match result {
                            Ok(()) => Ok(payload),
                            Err(e) => Err(e),
                        }
                    })
                    .partition(Result::is_ok);

                if errors.is_empty() {
                    let args_changes = RArray::new();
                    for change in changes.iter() {
                        let c = *change.as_ref().unwrap();
                        args_changes
                            .push(c)
                            .expect("cannot push change event to args");
                    }

                    let args = (args_changes,);
                    let _ = block.call::<(RArray,), Qnil>(args);
                    // todo: make sure we respect the result and bubble up the
                    //  error so that we can return as part of the Result
                }

                // todo: make sure we respect errors and let the method fail by
                //  by returning a Result containing an Error
            })
            .into();

        Ok(subscription_id)
    }
    pub(crate) fn yarray_push_back(&self, transaction: &YTransaction, value: Value) {
        let yvalue = YValue::from(value);
        let avalue = Any::from(yvalue);
        let mut tx = transaction.transaction();
        let tx = tx.as_mut().unwrap();

        self.0.borrow_mut().push_back(tx, avalue);
    }
    pub(crate) fn yarray_push_front(&self, transaction: &YTransaction, value: Value) {
        let yvalue = YValue::from(value);
        let avalue = Any::from(yvalue);

        let mut tx = transaction.transaction();
        let tx = tx.as_mut().unwrap();

        let arr = self.0.borrow_mut();
        arr.push_front(tx, avalue);
    }
    pub(crate) fn yarray_remove(&self, transaction: &YTransaction, index: u32) {
        let mut tx = transaction.transaction();
        let tx = tx.as_mut().unwrap();

        let arr = self.0.borrow_mut();
        arr.remove(tx, index)
    }
    pub(crate) fn yarray_remove_range(&self, transaction: &YTransaction, index: u32, len: u32) {
        let mut tx = transaction.transaction();
        let tx = tx.as_mut().unwrap();

        let arr = self.0.borrow_mut();
        arr.remove_range(tx, index, len)
    }
    pub(crate) fn yarray_to_a(&self, transaction: &YTransaction) -> RArray {
        let arr = self.0.borrow();
        let tx = transaction.transaction();
        let tx = tx.as_ref().unwrap();

        let r_arr = RArray::new();
        for item in arr.iter(tx) {
            let r_val = YValue::from(item);
            let r_val = *r_val.0.borrow();
            r_arr.push(r_val).expect("cannot push item event to array");
        }
        r_arr
    }
    pub(crate) fn yarray_unobserve(&self, subscription_id: u32) {
        self.0.borrow_mut().unobserve(subscription_id);
    }
}

impl From<ArrayRef> for YArray {
    fn from(v: ArrayRef) -> Self {
        YArray(RefCell::from(v))
    }
}
