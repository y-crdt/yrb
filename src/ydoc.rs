use crate::util::{convert_array_to_vecu8, convert_vecu8_to_array};
use crate::ytransaction::TRANSACTION_WRAPPER;
use rutie::rubysys::class;
use rutie::types::{Argc, Value};
use rutie::util::str_to_cstring;
use rutie::{AnyObject, Array, Integer, Module, Object};
use std::mem;
use yrs::updates::decoder::Decode;
use yrs::{Doc, OffsetKind, Options, StateVector};

wrappable_struct!(Doc, DocWrapper, DOC_WRAPPER);
class!(YDoc);

methods!(
    YDoc,
    rtself,
    fn ydoc_transact() -> AnyObject {
        let doc = rtself.get_data(&*DOC_WRAPPER);
        let transaction = doc.transact();

        Module::from_existing("Y")
            .get_nested_class("Transaction")
            .wrap_data(transaction, &*TRANSACTION_WRAPPER)
    },
    fn ydoc_encode_diff_v1(state_vector: Array) -> Array {
        let mut doc: &Doc = rtself.get_data_mut(&*DOC_WRAPPER);
        let state_vector_encoded: Vec<u8> =
            convert_array_to_vecu8(state_vector.unwrap());

        let result = &StateVector::decode_v1(state_vector_encoded.as_slice());
        let sv = match result {
            Ok(sv) => sv,
            Err(error) => {
                panic!("decoding the state vector failed: {:?}", error)
            }
        };

        let update = doc.encode_state_as_update_v1(sv);
        convert_vecu8_to_array(update)
    }
);

pub extern "C" fn ydoc_new(
    argc: Argc,
    argv: *const AnyObject,
    _rtself: AnyObject,
) -> AnyObject {
    let args = Value::from(0);

    unsafe {
        let p_argv: *const Value = mem::transmute(argv);

        class::rb_scan_args(argc, p_argv, str_to_cstring("*").as_ptr(), &args)
    };

    let arguments = Array::from(args);
    let client_id = arguments.at(0).try_convert_to::<Integer>();

    let mut options = Options::default();
    if let Ok(c_id) = client_id {
        options.client_id = c_id.into();
    };
    // make sure we treat offsets for codepoints not bytes
    options.offset_kind = OffsetKind::Utf32;

    let doc = Doc::with_options(options);

    Module::from_existing("Y")
        .get_nested_class("Doc")
        .wrap_data(doc, &*DOC_WRAPPER)
}
