use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use lib0::any::Any;
use magnus::block::Proc;
use magnus::{Error, exception, Integer, QNIL, RArray, RHash, Symbol, Value};
use magnus::exception::exception;
use magnus::value::Qnil;
use yrs::{Array, SubscriptionId};
use yrs::types::{Change, Value as YrsValue};
use crate::utils::map_magnus_value_to_lib0_any;
use crate::YTransaction;
use crate::yvalue::YValue;

#[magnus::wrap(class = "Y::Array")]
pub(crate) struct YArray(pub(crate) RefCell<Array>);

impl YArray {
    pub(crate) fn yarray_each(&self, block: Proc) -> () {
        self.0
            .borrow_mut()
            .iter()
            .for_each(|val| {
                let yvalue = YValue::from(val);
                let args = (yvalue.into(), );
                if let Err(e) = block.call::<(Value, ), Qnil>(args) {
                    Error::from(e);
                }
            });
    }

    pub(crate) fn yarray_get(&self, index: u32) -> Value {
        let v = self.0.borrow().get(index).unwrap();
        YValue::from(v).into()
    }
    pub(crate) fn yarray_insert(&self, transaction: &YTransaction, index: u32, value: Value) -> () {
        let yvalue = YValue::from(value);
        let avalue = Any::from(yvalue);
        self.0.borrow_mut()
            .insert(&mut *transaction.0.borrow_mut(), index, avalue);
    }
    pub(crate) fn yarray_insert_range(&self, transaction: &YTransaction, index: u32, values: RArray) -> () {
        let arr: Vec<Any> = values.each().into_iter()
            .map(|value| YValue::from(value.unwrap()).into())
            .collect();

        self.0.borrow_mut()
            .insert_range(&mut *transaction.0.borrow_mut(), index, arr);
    }
    pub(crate) fn yarray_length(&self) -> u32 {
        return self.0.borrow().len();
    }
    pub(crate) fn yarray_observe(&self, block: Proc) -> u32 {
        let subscription_id = self.0.borrow_mut()
            .observe(move |transaction, array_event| {
                let delta = array_event.delta(transaction);
                let mut changes = RArray::with_capacity(delta.len());

                for change in delta {
                    match change {
                        Change::Added(v) => {
                            let mut payload = RHash::new();
                            let values = v.iter()
                                .map(|v| YValue::from(v.clone()).into())
                                .collect::<Vec<Value>>();
                            payload.aset(Symbol::new("added"), RArray::from_vec(values));
                            changes.push(payload);
                        }
                        Change::Retain(position) => {
                            let mut payload = RHash::new();
                            payload.aset(Symbol::new("retain"), Value::from(*position));
                            changes.push(payload);
                        }
                        Change::Removed(position) => {
                            let mut payload = RHash::new();
                            payload.aset(Symbol::new("removed"), Value::from(*position));
                            changes.push(payload);
                        }
                    }
                }

                let args = (changes, );
                block.call::<(RArray, ), Qnil>(args);
            }).into();

        subscription_id
    }
    pub(crate) fn yarray_push_back(&self, transaction: &YTransaction, value: Value) -> () {
        let yvalue = YValue::from(value);
        let avalue = Any::from(yvalue);
        self.0.borrow_mut()
            .push_back(&mut *transaction.0.borrow_mut(), avalue)
    }
    pub(crate) fn yarray_push_front(&self, transaction: &YTransaction, value: Value) -> () {
        let yvalue = YValue::from(value);
        let avalue = Any::from(yvalue);
        self.0.borrow_mut()
            .push_front(&mut *transaction.0.borrow_mut(), avalue)
    }
    pub(crate) fn yarray_remove(&self, transaction: &YTransaction, index: u32) -> () {
        self.0.borrow_mut()
            .remove(&mut transaction.0.borrow_mut(), index)
    }
    pub(crate) fn yarray_remove_range(&self, transaction: &YTransaction, index: u32, len: u32) -> () {
        self.0.borrow_mut()
            .remove_range(&mut transaction.0.borrow_mut(), index, len)
    }
    pub(crate) fn yarray_to_a(&self) -> RArray {
        let arr = self.0.borrow_mut().iter()
            .map(|v| YValue::from(v).into())
            .collect::<Vec<Value>>();

        return RArray::from_vec(arr);
    }
    pub(crate) fn yarray_unobserve(&self, subscription_id: u32) -> () {
        self.0.borrow_mut()
            .unobserve(subscription_id);
    }
}
