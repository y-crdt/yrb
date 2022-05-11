use crate::util::{map_ruby_type_to_rust, map_yrs_value_to_ruby};
use crate::ytransaction::{YTransaction, TRANSACTION_WRAPPER};
use rutie::{
    AnyObject, Array as RArray, Fixnum, Hash, Integer, NilClass, Object, Proc,
    Symbol, VM,
};
use yrs::types::{Change, Value};
use yrs::{Array, SubscriptionId};

wrappable_struct!(Array, ArrayWrapper, ARRAY_WRAPPER);
class!(YArray);

#[rustfmt::skip]
methods!(
    YArray,
    rtself,
    fn yarray_each(block: Proc) -> NilClass {
        let b = block.map_err(|e| VM::raise_ex(e)).unwrap();

        let a: &Array = rtself.get_data(&*ARRAY_WRAPPER);

        a
            .iter()
            .for_each(|val| {
                let args = [map_yrs_value_to_ruby(val)];
                b.call(&args);
            });

        NilClass::new()
    },
    fn yarray_get(index: Fixnum) -> AnyObject {
        let i = index.map_err(|e| VM::raise_ex(e)).unwrap();

        let arr: &Array = rtself.get_data(&*ARRAY_WRAPPER);
        let val = arr.get(i.to_u32());

        map_yrs_value_to_ruby(val.unwrap())
    },
    fn yarray_insert(
        transaction: YTransaction,
        index: Fixnum,
        value: AnyObject) -> NilClass {
        let mut txn = transaction.map_err(|e| VM::raise_ex(e)).unwrap();
        let tx = txn.get_data_mut(&*TRANSACTION_WRAPPER);

        let i = index.map_err(|e| VM::raise_ex(e)).unwrap();

        let val = value.map_err(|e| VM::raise_ex(e)).unwrap();
        let v = map_ruby_type_to_rust(val).unwrap();

        let arr: &Array = rtself.get_data(&*ARRAY_WRAPPER);
        arr.insert(tx, i.to_u32(), v);

        NilClass::new()
    },
    fn yarray_insert_range(
        transaction: YTransaction,
        index: Fixnum,
        values: RArray) -> NilClass {
        let mut txn = transaction.map_err(|e| VM::raise_ex(e)).unwrap();
        let tx = txn.get_data_mut(&*TRANSACTION_WRAPPER);

        let i = index.map_err(|e| VM::raise_ex(e)).unwrap();

        let values = values.map_err(|e| VM::raise_ex(e)).unwrap();
        let mapped_values = values
            .into_iter()
            .map(|value| map_ruby_type_to_rust(value).unwrap() )
            .collect::<Vec<_>>();

        let arr: &Array = rtself.get_data(&*ARRAY_WRAPPER);
        arr.insert_range(tx, i.to_u32(), mapped_values);

        NilClass::new()
    },
    fn yarray_length() -> Fixnum {
        let arr: &Array = rtself.get_data(&*ARRAY_WRAPPER);
        Fixnum::new(i64::from(arr.len()))
    },
    fn yarray_observe(callback: Proc) -> Integer {
        let c = callback.map_err(|e| VM::raise_ex(e)).unwrap();

        let arr: &mut Array = rtself.get_data_mut(&*ARRAY_WRAPPER);
        let subscription_id: SubscriptionId = arr
            .observe(move |transaction, array_event| {
                let delta = array_event.delta(transaction);
                let mut changes: Vec<AnyObject> = Vec::new();

                for change in delta {
                    match change {
                        Change::Added(v) => {
                            let mut payload = Hash::new();
                            let values = v.iter().map(|v| map_yrs_value_to_ruby(v.clone()) ).collect::<Vec<_>>();
                            payload.store(Symbol::new("added"), RArray::from_iter(values));

                            changes.push(payload.to_any_object());
                        },
                        Change::Retain(position) => {
                            let mut payload = Hash::new();
                            payload.store(Symbol::new("retain"), Integer::from(*position));

                            changes.push(payload.to_any_object());
                        },
                        Change::Removed(position) => {
                            let mut payload = Hash::new();
                            payload.store(Symbol::new("removed"), Integer::from(*position));

                            changes.push(payload.to_any_object());
                        }
                    }
                }

                let args = &[RArray::from_iter(changes).to_any_object()];
                c.call(args);
            })
            .into();

        Integer::from(subscription_id)
    },
    fn yarray_push_back(transaction: YTransaction, value: AnyObject) -> NilClass {
        let mut txn = transaction.map_err(|e| VM::raise_ex(e)).unwrap();
        let tx = txn.get_data_mut(&*TRANSACTION_WRAPPER);

        let val = value.map_err(|e| VM::raise_ex(e)).unwrap();
        let v = map_ruby_type_to_rust(val).unwrap();

        let arr: &Array = rtself.get_data(&*ARRAY_WRAPPER);
        arr.push_back(tx, v);

        NilClass::new()
    },
    fn yarray_push_front(transaction: YTransaction, value: AnyObject) -> NilClass {
        let mut txn = transaction.map_err(|e| VM::raise_ex(e)).unwrap();
        let tx = txn.get_data_mut(&*TRANSACTION_WRAPPER);

        let val = value.map_err(|e| VM::raise_ex(e)).unwrap();
        let v = map_ruby_type_to_rust(val).unwrap();

        let arr: &Array = rtself.get_data(&*ARRAY_WRAPPER);
        arr.push_front(tx, v);

        NilClass::new()
    },
    fn yarray_remove(transaction: YTransaction, index: Fixnum) -> NilClass {
        let mut txn = transaction.map_err(|e| VM::raise_ex(e)).unwrap();
        let tx = txn.get_data_mut(&*TRANSACTION_WRAPPER);

        let i = index.map_err(|e| VM::raise_ex(e)).unwrap();

        let arr: &Array = rtself.get_data(&*ARRAY_WRAPPER);
        arr.remove(tx, i.to_u32());

        NilClass::new()
    },
    fn yarray_remove_range(
        transaction: YTransaction,
        index: Fixnum,
        length: Fixnum) -> NilClass {
        let mut txn = transaction.map_err(|e| VM::raise_ex(e)).unwrap();
        let tx = txn.get_data_mut(&*TRANSACTION_WRAPPER);

        let i = index.map_err(|e| VM::raise_ex(e)).unwrap();
        let l = length.map_err(|e| VM::raise_ex(e)).unwrap();

        let arr: &Array = rtself.get_data(&*ARRAY_WRAPPER);
        arr.remove_range(tx, i.to_u32(), l.to_u32());

        NilClass::new()
    },
    fn yarray_to_a() -> RArray {
        let v: &Array = rtself.get_data(&*ARRAY_WRAPPER);
        map_yrs_value_to_ruby(Value::YArray(v.clone()))
            .try_convert_to::<RArray>()
            .unwrap()
    }
    fn yarray_unobserve(subscription_id: Integer) -> NilClass {
        let s = subscription_id.map_err(|e| VM::raise_ex(e)).unwrap();

        let arr: &mut Array = rtself.get_data_mut(&*ARRAY_WRAPPER);
        arr.unobserve(s.into());

        NilClass::new()
    }
);
