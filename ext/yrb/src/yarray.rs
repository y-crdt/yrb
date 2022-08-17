use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use lib0::any::Any;
use magnus::block::Proc;
use magnus::{Error, exception, QNIL, RArray, Value};
use magnus::exception::exception;
use magnus::value::Qnil;
use yrs::{Array};
use yrs::types::{Value as YrsValue};
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
                if let Err(e) = block.call::<(Value,), Qnil>(args) {
                    Error::from(e);
                }
            });
    }

    pub(crate) fn yarray_get(&self, index: u32) -> Result<Value, Error> {
        return if let Some(val) = self.0.borrow().get(index) {
            Ok(*YValue::from(val).0.borrow())
        } else {
            Err(
                Error::new(
                    exception::type_error(),
                    "cannot convert element into a Ruby type",
                )
            )
        };
    }
    pub(crate) fn yarray_insert(&self, transaction: &YTransaction, index: u32, value: Value) -> () {
        let yvalue = YValue::from(value);
        let avalue = Any::from(yvalue);
        self.0
            .borrow_mut()
            .insert(&mut *transaction.0.borrow_mut(), index, avalue);
    }
    pub(crate) fn yarray_insert_range(&self, transaction: &YTransaction, index: u32, values: RArray) -> () {
        let arr: Vec<Any> = values.each().into_iter()
            .map(|value| {
                if let Ok(v) = value {
                    YValue::from(v)
                } else {
                    YValue::from(Value::from(QNIL))
                }
            })
            .map(|value| value.into())
            .collect();

        self.0
            .borrow_mut()
            .insert_range(&mut *transaction.0.borrow_mut(), index, arr);
    }
    pub(crate) fn yarray_length(&self) -> u32 {
        return self.0.borrow().len();
    }
    pub(crate) fn yarray_push_back(&self, transaction: &YTransaction, value: Value) -> () {
        let yvalue = YValue::from(value);
        let avalue = Any::from(yvalue);
        self.0
            .borrow_mut()
            .push_back(&mut *transaction.0.borrow_mut(), avalue)
    }
    pub(crate) fn yarray_to_a(&self) -> RArray {
        let arr = self.0
            .borrow_mut()
            .iter()
            .map(|v| YValue::from(v).into())
            .collect::<Vec<Value>>();

        return RArray::from_vec(arr);
    }
}
