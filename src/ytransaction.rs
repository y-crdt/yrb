use crate::util::{convert_array_to_vecu8, convert_vecu8_to_array};
use crate::yarray::ARRAY_WRAPPER;
use crate::ymap::MAP_WRAPPER;
use crate::ytext::TEXT_WRAPPER;
use crate::yxml::{XML_ELEMENT_WRAPPER, XML_TEXT_WRAPPER};
use rutie::{
    AnyObject, Array, Module, NilClass, Object, RString, VerifiedObject, VM,
};
use yrs::updates::decoder::Decode;
use yrs::updates::encoder::Encode;
use yrs::{Transaction, Update};

wrappable_struct!(Transaction, TransactionWrapper, TRANSACTION_WRAPPER);
class!(YTransaction);

impl VerifiedObject for YTransaction {
    fn is_correct_type<T: Object>(object: &T) -> bool {
        object.class()
            == Module::from_existing("Y").get_nested_class("Transaction")
    }

    fn error_message() -> &'static str {
        "Error converting to YTransaction"
    }
}

methods!(
    YTransaction,
    rtself,
    fn ytransaction_apply_update(update: Array) -> NilClass {
        let u = convert_array_to_vecu8(update.unwrap());

        let transaction = rtself.get_data_mut(&*TRANSACTION_WRAPPER);
        transaction.apply_update(Update::decode_v1(u.as_slice()));

        NilClass::new()
    },
    fn ytransaction_commit() -> NilClass {
        let transaction = rtself.get_data_mut(&*TRANSACTION_WRAPPER);
        transaction.commit();

        NilClass::new()
    },
    fn ytransaction_get_array(name: RString) -> AnyObject {
        let name_str = name.map_err(|e| VM::raise_ex(e)).unwrap().to_string();

        let transaction = rtself.get_data_mut(&*TRANSACTION_WRAPPER);
        let arr = transaction.get_array(&name_str);

        Module::from_existing("Y")
            .get_nested_class("Array")
            .wrap_data(arr, &*ARRAY_WRAPPER)
    },
    fn ytransaction_get_map(name: RString) -> AnyObject {
        let name_str = name.map_err(|e| VM::raise_ex(e)).unwrap().to_string();

        let transaction = rtself.get_data_mut(&*TRANSACTION_WRAPPER);
        let map = transaction.get_map(&name_str);

        Module::from_existing("Y")
            .get_nested_class("Map")
            .wrap_data(map, &*MAP_WRAPPER)
    },
    fn ytransaction_get_text(name: RString) -> AnyObject {
        let name_str = name.map_err(|e| VM::raise_ex(e)).unwrap().to_string();

        let transaction = rtself.get_data_mut(&*TRANSACTION_WRAPPER);
        let text = transaction.get_text(&name_str);

        Module::from_existing("Y")
            .get_nested_class("Text")
            .wrap_data(text, &*TEXT_WRAPPER)
    },
    fn ytransaction_get_xml_element(name: RString) -> AnyObject {
        let name_str = name.map_err(|e| VM::raise_ex(e)).unwrap().to_string();

        let transaction = rtself.get_data_mut(&*TRANSACTION_WRAPPER);
        let xml_element = transaction.get_xml_element(&name_str);

        Module::from_existing("Y")
            .get_nested_class("XMLElement")
            .wrap_data(xml_element, &*XML_ELEMENT_WRAPPER)
    },
    fn ytransaction_get_xml_text(name: RString) -> AnyObject {
        let name_str = name.map_err(|e| VM::raise_ex(e)).unwrap().to_string();

        let transaction = rtself.get_data_mut(&*TRANSACTION_WRAPPER);
        let xml_text = transaction.get_xml_text(&name_str);

        Module::from_existing("Y")
            .get_nested_class("XMLText")
            .wrap_data(xml_text, &*XML_TEXT_WRAPPER)
    },
    fn ytransaction_state_vector() -> Array {
        let transaction = rtself.get_data_mut(&*TRANSACTION_WRAPPER);
        let sv = transaction.state_vector();
        let payload = sv.encode_v1();

        convert_vecu8_to_array(payload)
    }
);
