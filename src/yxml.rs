use crate::ytransaction::{YTransaction, TRANSACTION_WRAPPER};
use rutie::{AnyObject, Array, Fixnum, Module, NilClass, Object, RString, VM};
use yrs::types::xml::Attributes;
use yrs::{Xml, XmlElement, XmlText};

wrappable_struct!(XmlElement, XmlElementWrapper, XML_ELEMENT_WRAPPER);
class!(YXmlElement);

#[rustfmt::skip]
methods!(
    YXmlElement,
    rtself,
    fn yxml_element_attributes() -> Array {
        let xml_element = rtself.get_data(&*XML_ELEMENT_WRAPPER);

        let mut arr = Array::new();

        let attrs: Attributes = xml_element.attributes();
        for (key, val) in attrs {
            let mut pair = Array::with_capacity(2);
            pair.push(RString::new_utf8(key));
            pair.push(RString::new_utf8(&val));

            arr.push(pair);
        }

        arr
    },
    fn yxml_element_get(index: Fixnum) -> AnyObject {
        let i = index.map_err(|e| VM::raise_ex(e)).unwrap();

        let xml_element = rtself.get_data(&*XML_ELEMENT_WRAPPER);
        let node = xml_element.get(i.to_u32());

        match node {
            Some(Xml::Element(v)) => Module::from_existing("Y")
                .get_nested_class("XMLElement")
                .wrap_data(v, &*XML_ELEMENT_WRAPPER),
            Some(Xml::Text(v)) => Module::from_existing("Y")
                .get_nested_class("XMLText")
                .wrap_data(v, &*XML_TEXT_WRAPPER),
            None => NilClass::new().to_any_object()
        }
    },
    fn yxml_element_get_attribute(name: RString) -> AnyObject {
        let n = name.map_err(|e| VM::raise_ex(e)).unwrap();

        let xml_element = rtself.get_data(&*XML_ELEMENT_WRAPPER);
        let attr_name = xml_element.get_attribute(&n.to_string());

        match attr_name {
            Some(v) => RString::new_utf8(&v).to_any_object(),
            None => NilClass::new().to_any_object()
        }
    },
    fn yxml_element_insert_attribute(
        transaction: YTransaction,
        name: RString,
        value: RString) -> NilClass {
        let mut t = transaction.map_err(|e| VM::raise_ex(e)).unwrap();
        let n = name.map_err(|e| VM::raise_ex(e)).unwrap();
        let v = value.map_err(|e| VM::raise_ex(e)).unwrap();

        let tx = t.get_data_mut(&*TRANSACTION_WRAPPER);

        let xml_element = rtself.get_data(&*XML_ELEMENT_WRAPPER);
        xml_element.insert_attribute(tx, n.to_string(), v.to_string());

        NilClass::new()
    },
    fn yxml_element_insert_element(transaction: YTransaction, index: Fixnum, name: RString) -> AnyObject {
        let mut t = transaction.map_err(|e| VM::raise_ex(e)).unwrap();
        let i = index.map_err(|e| VM::raise_ex(e)).unwrap();
        let n = name.map_err(|e| VM::raise_ex(e)).unwrap();

        let tx = t.get_data_mut(&*TRANSACTION_WRAPPER);

        let xml_element = rtself.get_data(&*XML_ELEMENT_WRAPPER);
        let new_element: XmlElement = xml_element.insert_elem(tx, i.to_u32(), n.to_string());

        Module::from_existing("Y")
            .get_nested_class("XMLElement")
            .wrap_data(new_element, &*XML_ELEMENT_WRAPPER)
    },
    fn yxml_element_insert_text(transaction: YTransaction, index: Fixnum) -> AnyObject {
        let mut t = transaction.map_err(|e| VM::raise_ex(e)).unwrap();
        let i = index.map_err(|e| VM::raise_ex(e)).unwrap();

        let tx = t.get_data_mut(&*TRANSACTION_WRAPPER);

        let xml_element = rtself.get_data(&*XML_ELEMENT_WRAPPER);
        let new_text = xml_element.insert_text(tx, i.to_u32());

        Module::from_existing("Y")
            .get_nested_class("XMLText")
            .wrap_data(new_text, &*XML_TEXT_WRAPPER)
    }
    fn yxml_element_push_elem_back(transaction: YTransaction, name: RString) -> AnyObject {
        let mut t = transaction.map_err(|e| VM::raise_ex(e)).unwrap();
        let n = name.map_err(|e| VM::raise_ex(e)).unwrap();

        let tx = t.get_data_mut(&*TRANSACTION_WRAPPER);

        let xml_element = rtself.get_data(&*XML_ELEMENT_WRAPPER);
        let new_element: XmlElement = xml_element.push_elem_back(tx, n.to_string());

        Module::from_existing("Y")
            .get_nested_class("XMLElement")
            .wrap_data(new_element, &*XML_ELEMENT_WRAPPER)
    },
    fn yxml_element_push_elem_front(transaction: YTransaction, name: RString) -> AnyObject {
        let mut t = transaction.map_err(|e| VM::raise_ex(e)).unwrap();
        let n = name.map_err(|e| VM::raise_ex(e)).unwrap();

        let tx = t.get_data_mut(&*TRANSACTION_WRAPPER);

        let xml_element = rtself.get_data(&*XML_ELEMENT_WRAPPER);
        let new_element: XmlElement = xml_element.push_elem_front(tx, n.to_string());

        Module::from_existing("Y")
            .get_nested_class("XMLElement")
            .wrap_data(new_element, &*XML_ELEMENT_WRAPPER)
    },
    fn yxml_element_push_text_back(transaction: YTransaction) -> AnyObject {
        let mut t = transaction.map_err(|e| VM::raise_ex(e)).unwrap();

        let tx = t.get_data_mut(&*TRANSACTION_WRAPPER);

        let xml_element = rtself.get_data(&*XML_ELEMENT_WRAPPER);
        let new_text = xml_element.push_text_back(tx);

        Module::from_existing("Y")
            .get_nested_class("XMLText")
            .wrap_data(new_text, &*XML_TEXT_WRAPPER)
    },
    fn yxml_element_push_text_front(transaction: YTransaction) -> AnyObject {
        let mut t = transaction.map_err(|e| VM::raise_ex(e)).unwrap();

        let tx = t.get_data_mut(&*TRANSACTION_WRAPPER);

        let xml_element = rtself.get_data(&*XML_ELEMENT_WRAPPER);
        let new_text = xml_element.push_text_front(tx);

        Module::from_existing("Y")
            .get_nested_class("XMLText")
            .wrap_data(new_text, &*XML_TEXT_WRAPPER)
    },
    fn yxml_element_remove_attribute(
        transaction: YTransaction,
        name: RString) -> NilClass {

        let mut t = transaction.map_err(|e| VM::raise_ex(e)).unwrap();
        let n = name.map_err(|e| VM::raise_ex(e)).unwrap();

        let tx = t.get_data_mut(&*TRANSACTION_WRAPPER);

        let xml_element = rtself.get_data(&*XML_ELEMENT_WRAPPER);
        xml_element.remove_attribute(tx, &n.to_string());

        NilClass::new()
    },
    fn yxml_element_remove_range(transaction: YTransaction, index: Fixnum, length: Fixnum) -> NilClass {
        let mut t = transaction.map_err(|e| VM::raise_ex(e)).unwrap();
        let i = index.map_err(|e| VM::raise_ex(e)).unwrap();
        let l = length.map_err(|e| VM::raise_ex(e)).unwrap();

        let tx = t.get_data_mut(&*TRANSACTION_WRAPPER);

        let xml_element = rtself.get_data(&*XML_ELEMENT_WRAPPER);
        xml_element.remove_range(tx, i.to_u32(), l.to_u32());

        NilClass::new()
    },
    fn yxml_element_size() -> Fixnum {
        Fixnum::new(0)
    },
    fn yxml_element_tag() -> RString {
        let xml_element = rtself.get_data(&*XML_ELEMENT_WRAPPER);

        RString::new_utf8(xml_element.tag())
    }
);

wrappable_struct!(XmlText, XmlTextWrapper, XML_TEXT_WRAPPER);
class!(YXmlText);

#[rustfmt::skip]
methods!(
    YXmlText,
    rtself,

);
