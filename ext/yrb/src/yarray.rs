use crate::yvalue::YValue;
use crate::YTransaction;
use lib0::any::Any;
use magnus::block::Proc;
use magnus::value::Qnil;
use magnus::{Error, RArray, RHash, Symbol, Value};
use std::cell::RefCell;
use yrs::types::Change;
use yrs::Array;

#[magnus::wrap(class = "Y::Array")]
pub(crate) struct YArray(pub(crate) RefCell<Array>);

/// SAFETY: This is safe because we only access this data when the GVL is held.
unsafe impl Send for YArray {}

impl YArray {
    pub(crate) fn yarray_each(&self, block: Proc) {
        self.0.borrow_mut().iter().for_each(|val| {
            let yvalue = YValue::from(val);
            let args = (yvalue.into(),);
            let _ = block.call::<(Value,), Qnil>(args);
        });
    }
    pub(crate) fn yarray_get(&self, index: u32) -> Value {
        let v = self.0.borrow().get(index).unwrap();
        YValue::from(v).into()
    }
    pub(crate) fn yarray_insert(&self, transaction: &YTransaction, index: u32, value: Value) {
        let yvalue = YValue::from(value);
        let avalue = Any::from(yvalue);
        self.0
            .borrow_mut()
            .insert(&mut *transaction.0.borrow_mut(), index, avalue);
    }
    pub(crate) fn yarray_insert_range(
        &self,
        transaction: &YTransaction,
        index: u32,
        values: RArray,
    ) {
        let arr: Vec<Any> = values
            .each()
            .into_iter()
            .map(|value| YValue::from(value.unwrap()).into())
            .collect();

        self.0
            .borrow_mut()
            .insert_range(&mut *transaction.0.borrow_mut(), index, arr);
    }
    pub(crate) fn yarray_length(&self) -> u32 {
        return self.0.borrow().len();
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
                                payload.aset(change_retain, Value::from(*position))
                            }
                            Change::Removed(position) => {
                                payload.aset(change_removed, Value::from(*position))
                            }
                        };

                        match result {
                            Ok(()) => Ok(payload),
                            Err(e) => Err(e),
                        }
                    })
                    .partition(Result::is_ok);

                if errors.is_empty() {
                    let args = (RArray::from_vec(
                        changes.into_iter().map(Result::unwrap).collect(),
                    ),);
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
        self.0
            .borrow_mut()
            .push_back(&mut *transaction.0.borrow_mut(), avalue)
    }
    pub(crate) fn yarray_push_front(&self, transaction: &YTransaction, value: Value) {
        let yvalue = YValue::from(value);
        let avalue = Any::from(yvalue);
        self.0
            .borrow_mut()
            .push_front(&mut *transaction.0.borrow_mut(), avalue)
    }
    pub(crate) fn yarray_remove(&self, transaction: &YTransaction, index: u32) {
        self.0
            .borrow_mut()
            .remove(&mut transaction.0.borrow_mut(), index)
    }
    pub(crate) fn yarray_remove_range(&self, transaction: &YTransaction, index: u32, len: u32) {
        self.0
            .borrow_mut()
            .remove_range(&mut transaction.0.borrow_mut(), index, len)
    }
    pub(crate) fn yarray_to_a(&self) -> RArray {
        let arr = self
            .0
            .borrow_mut()
            .iter()
            .map(|v| YValue::from(v).into())
            .collect::<Vec<Value>>();

        RArray::from_vec(arr)
    }
    pub(crate) fn yarray_unobserve(&self, subscription_id: u32) {
        self.0.borrow_mut().unobserve(subscription_id);
    }
}
