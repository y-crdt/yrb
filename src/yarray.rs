use crate::ytransaction::{YTransaction, TRANSACTION_WRAPPER};
use crate::util::{map_ruby_type_to_rust, map_yrs_value_to_ruby};
use rutie::{AnyObject, Array as RArray, Fixnum, NilClass, Object, VM};
use yrs::{Array};
use yrs::types::Value;

wrappable_struct!(Array, ArrayWrapper, ARRAY_WRAPPER);
class!(YArray);

methods!(
    YArray,
    rtself,
    fn yarray_length() -> Fixnum {
        let arr: &Array = rtself.get_data(&*ARRAY_WRAPPER);
        Fixnum::new(i64::from(arr.len()))
    }
    fn yarray_insert(transaction: YTransaction, index: Fixnum, value: AnyObject) -> NilClass {
        let mut txn = transaction.map_err(|e| VM::raise_ex(e)).unwrap();
        let tx = txn.get_data_mut(&*TRANSACTION_WRAPPER);

        let i = index.map_err(|e| VM::raise_ex(e)).unwrap();

        let val = value.map_err(|e| VM::raise_ex(e)).unwrap();
        let v = map_ruby_type_to_rust(val).unwrap();

        let arr: &Array = rtself.get_data(&*ARRAY_WRAPPER);
        arr.insert(tx, i.to_u32(), v);

        NilClass::new()
    }
    fn yarray_remove(transaction: YTransaction, index: Fixnum) -> NilClass {
        let mut txn = transaction.map_err(|e| VM::raise_ex(e)).unwrap();
        let tx = txn.get_data_mut(&*TRANSACTION_WRAPPER);

        let i = index.map_err(|e| VM::raise_ex(e)).unwrap();

        let arr: &Array = rtself.get_data(&*ARRAY_WRAPPER);
        arr.remove(tx, i.to_u32());

        NilClass::new()
    }
    fn yarray_remove_range(transaction: YTransaction, index: Fixnum, length: Fixnum) -> NilClass {
        let mut txn = transaction.map_err(|e| VM::raise_ex(e)).unwrap();
        let tx = txn.get_data_mut(&*TRANSACTION_WRAPPER);

        let i = index.map_err(|e| VM::raise_ex(e)).unwrap();
        let l = length.map_err(|e| VM::raise_ex(e)).unwrap();

        let arr: &Array = rtself.get_data(&*ARRAY_WRAPPER);
        arr.remove_range(tx, i.to_u32(), l.to_u32());

        NilClass::new()
    }
    fn yarray_to_arr() -> RArray {
        let v: &Array = rtself.get_data(&*ARRAY_WRAPPER);
        map_yrs_value_to_ruby(Value::YArray(v.clone()))
            .try_convert_to::<RArray>()
            .unwrap()
    }
);
