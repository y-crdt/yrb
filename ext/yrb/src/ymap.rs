use crate::utils::indifferent_hash_key;
use crate::yvalue::YValue;
use crate::YTransaction;
use lib0::any::Any;
use magnus::block::Proc;
use magnus::{Error, RArray, RHash, Symbol, Value};
use std::cell::RefCell;
use yrs::types::{EntryChange, Value as YrsValue};
use yrs::Map;

#[magnus::wrap(class = "Y::Map")]
pub(crate) struct YMap(pub(crate) RefCell<Map>);

/// SAFETY: This is safe because we only access this data when the GVL is held.
unsafe impl Send for YMap {}

impl YMap {
    pub(crate) fn ymap_clear(&self, transaction: &YTransaction) {
        self.0.borrow_mut().clear(&mut *transaction.0.borrow_mut());
    }
    pub(crate) fn ymap_contains(&self, key: Value) -> bool {
        match indifferent_hash_key(key) {
            None => false,
            Some(k) => self.0.borrow().contains(&*k)
        }
    }
    pub(crate) fn ymap_each(&self, proc: Proc) {
        self.0.borrow().iter().for_each(|(key, val)| {
            let k = key.to_string();
            let v = *YValue::from(val).0.borrow();
            proc.call::<(String, Value), Value>((k, v))
                .expect("cannot iterate map");
        });
    }
    pub(crate) fn ymap_get(&self, key: Value) -> Option<Value> {
        indifferent_hash_key(key)
            .map(|k| self.0.borrow().get(&*k))
            .map(|v| v.unwrap_or(YrsValue::Any(Any::Undefined)))
            .map(|v| *YValue::from(v).0.borrow())
    }
    pub(crate) fn ymap_insert(
        &self,
        transaction: &YTransaction,
        key: Value,
        value: Value
    ) -> Result<(), Error> {
        match indifferent_hash_key(key) {
            None => Err(Error::runtime_error(
                "invalid key type, make sure it is either a Symbol or a String"
            )),
            Some(k) => {
                let v = Any::from(YValue::from(value));
                self.0.borrow_mut().insert(
                    &mut *transaction.0.borrow_mut(),
                    k,
                    v
                );

                Ok(())
            }
        }
    }
    pub(crate) fn ymap_observe(&self, block: Proc) -> u32 {
        let change_inserted = Symbol::new("inserted").as_static();
        let change_updated = Symbol::new("updated").as_static();
        let change_removed = Symbol::new("removed").as_static();

        self.0
            .borrow_mut()
            .observe(move |transaction, map_event| {
                let delta = map_event.keys(transaction);
                let changes = RArray::with_capacity(delta.len());

                for (key, change) in delta {
                    match change {
                        EntryChange::Inserted(v) => {
                            let h = RHash::new();
                            h.aset(
                                Symbol::new(&key.to_string()),
                                *YValue::from(v.clone()).0.borrow()
                            )
                            .expect("cannot add change::inserted");

                            let payload = RHash::new();
                            payload
                                .aset(change_inserted, h)
                                .expect("cannot add change::inserted");

                            changes
                                .push(payload)
                                .expect("cannot push changes::payload");
                        }
                        EntryChange::Updated(old, new) => {
                            let values = RArray::with_capacity(2);
                            values
                                .push(*YValue::from(old.clone()).0.borrow())
                                .expect("cannot push change::updated");
                            values
                                .push(*YValue::from(new.clone()).0.borrow())
                                .expect("cannot push change::updated");

                            let h = RHash::new();
                            h.aset(Symbol::new(&key.to_string()), values)
                                .expect("cannot push change::updated");

                            let payload = RHash::new();
                            payload
                                .aset(change_updated, h)
                                .expect("cannot push change::updated");

                            changes
                                .push(payload)
                                .expect("cannot push changes::payload");
                        }
                        EntryChange::Removed(v) => {
                            let h = RHash::new();
                            h.aset(
                                Symbol::new(&key.to_string()),
                                *YValue::from(v.clone()).0.borrow()
                            )
                            .expect("cannot push change::removed");

                            let payload = RHash::new();
                            payload
                                .aset(change_removed, h)
                                .expect("cannot push change::removed");

                            changes
                                .push(payload)
                                .expect("cannot push changes::payload");
                        }
                    }
                }

                block
                    .call::<(RArray,), Value>((changes,))
                    .expect("cannot call block");
            })
            .into()
    }
    pub(crate) fn ymap_remove(
        &self,
        transaction: &YTransaction,
        key: Value
    ) -> Option<Value> {
        indifferent_hash_key(key)
            .map(|k| {
                self.0
                    .borrow()
                    .remove(&mut *transaction.0.borrow_mut(), &*k)
            })
            .map(|v| v.unwrap_or(YrsValue::Any(Any::Undefined)))
            .map(|v| *YValue::from(v).0.borrow())
    }
    pub(crate) fn ymap_size(&self) -> u32 {
        self.0.borrow().len()
    }
    pub(crate) fn ymap_to_h(&self) -> RHash {
        RHash::from_iter(
            self.0.borrow().iter().map(move |(k, v)| {
                (k.to_string(), *YValue::from(v).0.borrow())
            })
        )
    }
    pub(crate) fn ymap_unobserve(&self, subscription_id: u32) {
        self.0.borrow_mut().unobserve(subscription_id);
    }
}
