#![allow(unused_variables)]

use crate::util::{convert_array_to_vecu8, convert_vecu8_to_array};
use crate::ytransaction::TRANSACTION_WRAPPER;
use rutie::{AnyObject, Array, Module, Object};
use yrs::updates::decoder::Decode;
use yrs::updates::encoder::Encode;
use yrs::{Doc, StateVector, Transaction};

wrappable_struct!(Doc, DocWrapper, DOC_WRAPPER);
class!(YDoc);

methods!(
    YDoc,
    rtself,
    fn ydoc_new() -> AnyObject {
        let doc = Doc::default();
        Module::from_existing("Y")
            .get_nested_class("Doc")
            .wrap_data(doc, &*DOC_WRAPPER)
    },
    fn ydoc_transact() -> AnyObject {
        let doc = rtself.get_data(&*DOC_WRAPPER);
        let transaction = doc.transact();

        Module::from_existing("Y")
            .get_nested_class("Transaction")
            .wrap_data(transaction, &*TRANSACTION_WRAPPER)
    },
    fn ydoc_state_vector() -> Array {
        let doc = rtself.get_data(&*DOC_WRAPPER);
        let mut transaction: Transaction = doc.transact();
        let sv = transaction.state_vector();
        let payload = sv.encode_v1();

        convert_vecu8_to_array(payload)
    },
    fn ydoc_encode_diff_v1(state_vector: Array) -> Array {
        let mut doc: &Doc = rtself.get_data_mut(&*DOC_WRAPPER);
        let state_vector_encoded: Vec<u8> =
            convert_array_to_vecu8(state_vector.unwrap());
        let sv = &StateVector::decode_v1(state_vector_encoded.as_slice());

        let update = doc.encode_state_as_update_v1(sv);

        convert_vecu8_to_array(update)
    }
);
