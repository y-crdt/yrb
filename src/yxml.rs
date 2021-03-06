use crate::util::{
    map_attrs_to_hash, map_hash_to_attrs, map_ruby_type_to_rust,
    map_yrs_value_to_ruby
};
use crate::ytransaction::{YTransaction, TRANSACTION_WRAPPER};
use rutie::{
    AnyObject, Array as RArray, Fixnum, Hash, Integer, Module, NilClass,
    Object, Proc, RString, Symbol, VM
};
use yrs::types::xml::Attributes;
use yrs::types::{Change, Delta};
use yrs::{SubscriptionId, Xml, XmlElement, XmlText};

wrappable_struct!(XmlElement, XmlElementWrapper, XML_ELEMENT_WRAPPER);
class!(YXmlElement);

#[rustfmt::skip]
methods!(
    YXmlElement,
    rtself,
    fn yxml_element_attributes() -> Hash {
        let xml_element = rtself.get_data(&*XML_ELEMENT_WRAPPER);

        let mut h = Hash::new();

        let attrs: Attributes = xml_element.attributes();
        for (key, val) in attrs {
            h.store(
                Symbol::new(key),
                RString::new_utf8(&val)
            );
        }

        h
    },
    fn yxml_element_first_child() -> AnyObject {
       let index = 0;

        let xml_element = rtself.get_data(&*XML_ELEMENT_WRAPPER);
        let node = xml_element.get(index);

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
    },
    fn yxml_element_next_sibling() -> AnyObject {
        let xml_element = rtself.get_data(&*XML_ELEMENT_WRAPPER);
        let node = xml_element.next_sibling();

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
    fn yxml_element_observe(callback: Proc) -> Integer {
        let c = callback.map_err(|e| VM::raise_ex(e)).unwrap();

        let xml_element: &mut XmlElement = rtself.get_data_mut(&*XML_ELEMENT_WRAPPER);
        let subscription_id: SubscriptionId = xml_element
            .observe(move |transaction, xml_element_event| {
                // TODO: emit attribute changes
                let delta = xml_element_event.delta(transaction);
                let mut changes: Vec<AnyObject> = Vec::with_capacity(delta.len());

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
    fn yxml_element_parent() -> AnyObject {
        let xml_element = rtself.get_data(&*XML_ELEMENT_WRAPPER);
        let node = xml_element.parent();

        match node {
            Some(v) => Module::from_existing("Y")
                .get_nested_class("XMLElement")
                .wrap_data(v, &*XML_ELEMENT_WRAPPER),
            None => NilClass::new().to_any_object()
        }
    },
    fn yxml_element_prev_sibling() -> AnyObject {
        let xml_element = rtself.get_data(&*XML_ELEMENT_WRAPPER);
        let node = xml_element.prev_sibling();

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
        let xml_element = rtself.get_data(&*XML_ELEMENT_WRAPPER);

        Fixnum::new(i64::from(xml_element.len()))
    },
    fn yxml_element_tag() -> RString {
        let xml_element = rtself.get_data(&*XML_ELEMENT_WRAPPER);

        RString::new_utf8(xml_element.tag())
    },
    fn yxml_element_to_string() -> RString {
        let xml_element = rtself.get_data(&*XML_ELEMENT_WRAPPER);

        RString::new_utf8(&xml_element.to_string())
    },
    fn yxml_element_unobserve(subscription_id: Integer) -> NilClass {
        let s = subscription_id.map_err(|e| VM::raise_ex(e)).unwrap();

        let xml_element: &mut XmlElement = rtself.get_data_mut(&*XML_ELEMENT_WRAPPER);
        xml_element.unobserve(s.into());

        NilClass::new()
    }
);

wrappable_struct!(XmlText, XmlTextWrapper, XML_TEXT_WRAPPER);
class!(YXmlText);

#[rustfmt::skip]
methods!(
    YXmlText,
    rtself,
    fn yxml_text_attributes() -> Hash {
        let xml_element = rtself.get_data(&*XML_TEXT_WRAPPER);

        let mut h = Hash::new();

        let attrs: Attributes = xml_element.attributes();
        for (key, val) in attrs {
            h.store(
                Symbol::new(key),
                RString::new_utf8(&val)
            );
        }

        h
    },
    fn yxml_text_format(transaction: YTransaction, index: Fixnum, length: Fixnum, attrs: Hash) -> NilClass {
        let mut t = transaction.map_err(|e| VM::raise_ex(e)).unwrap();
        let tx = t.get_data_mut(&*TRANSACTION_WRAPPER);

        let i = index.map_err(|e| VM::raise_ex(e)).unwrap();
        let l = length.map_err(|e| VM::raise_ex(e)).unwrap();
        let a = attrs.map_err(|e| VM::raise_ex(e)).unwrap();
        let mapped_attrs = map_hash_to_attrs(a);

        let xml_text = rtself.get_data(&*XML_TEXT_WRAPPER);
        xml_text.format(tx, i.to_u32(), l.to_u32(), mapped_attrs);

        NilClass::new()
    },
    fn yxml_text_get_attribute(name: RString) -> AnyObject {
        let n = name.map_err(|e| VM::raise_ex(e)).unwrap();

        let xml_text = rtself.get_data(&*XML_TEXT_WRAPPER);
        let attr = xml_text.get_attribute(&n.to_string());

        match attr {
            Some(v) => RString::new_utf8(&v).to_any_object(),
            None => NilClass::new().to_any_object()
        }
    },
    fn yxml_text_insert(transaction: YTransaction, index: Fixnum, content: RString) -> NilClass {
        let mut t = transaction.map_err(|e| VM::raise_ex(e)).unwrap();
        let tx = t.get_data_mut(&*TRANSACTION_WRAPPER);

        let i = index.map_err(|e| VM::raise_ex(e)).unwrap();
        let c = content.map_err(|e| VM::raise_ex(e)).unwrap();

        let xml_text = rtself.get_data(&*XML_TEXT_WRAPPER);
        xml_text.insert(tx, i.to_u32(),&c.to_string());

        NilClass::new()
    },
    fn yxml_text_insert_attribute(transaction: YTransaction, name: RString, value: RString) -> NilClass {
        let mut t = transaction.map_err(|e| VM::raise_ex(e)).unwrap();
        let tx = t.get_data_mut(&*TRANSACTION_WRAPPER);

        let n = name.map_err(|e| VM::raise_ex(e)).unwrap();
        let v = value.map_err(|e| VM::raise_ex(e)).unwrap();

        let xml_text = rtself.get_data(&*XML_TEXT_WRAPPER);
        xml_text.insert_attribute(tx, n.to_string(), v.to_string());

        NilClass::new()
    },
    fn yxml_text_insert_embed(transaction: YTransaction, index: Fixnum, content: AnyObject) -> NilClass {
        let mut t = transaction.map_err(|e| VM::raise_ex(e)).unwrap();
        let tx = t.get_data_mut(&*TRANSACTION_WRAPPER);

        let i = index.map_err(|e| VM::raise_ex(e)).unwrap();
        let c = content.map_err(|e| VM::raise_ex(e)).unwrap();
        let mapped_content = map_ruby_type_to_rust(c).map_err(|e| VM::raise_ex(e)).unwrap();

        let xml_text = rtself.get_data(&*XML_TEXT_WRAPPER);
        xml_text.insert_embed(tx, i.to_u32(), mapped_content);

        NilClass::new()
    },
    fn yxml_text_insert_embed_with_attributes(transaction: YTransaction, index: Fixnum, content: AnyObject, attrs: Hash) -> NilClass {
        let mut t = transaction.map_err(|e| VM::raise_ex(e)).unwrap();
        let tx = t.get_data_mut(&*TRANSACTION_WRAPPER);

        let i = index.map_err(|e| VM::raise_ex(e)).unwrap();
        let c = content.map_err(|e| VM::raise_ex(e)).unwrap();
        let mapped_content = map_ruby_type_to_rust(c).map_err(|e| VM::raise_ex(e)).unwrap();
        let a = attrs.map_err(|e| VM::raise_ex(e)).unwrap();
        let mapped_attrs = map_hash_to_attrs(a);

        let xml_text = rtself.get_data(&*XML_TEXT_WRAPPER);
        xml_text.insert_embed_with_attributes(tx, i.to_u32(), mapped_content, mapped_attrs);

        NilClass::new()
    },
    fn yxml_text_insert_with_attributes(transaction: YTransaction, index: Fixnum, content: RString, attrs: Hash) -> NilClass {
        let mut t = transaction.map_err(|e| VM::raise_ex(e)).unwrap();
        let tx = t.get_data_mut(&*TRANSACTION_WRAPPER);

        let i = index.map_err(|e| VM::raise_ex(e)).unwrap();
        let c = content.map_err(|e| VM::raise_ex(e)).unwrap();
        let a = attrs.map_err(|e| VM::raise_ex(e)).unwrap();

        let mapped_attrs = map_hash_to_attrs(a);

        let xml_text = rtself.get_data(&*XML_TEXT_WRAPPER);
        xml_text.insert_with_attributes(tx, i.to_u32(), &c.to_string(), mapped_attrs);

        NilClass::new()
    },
    fn yxml_text_length() -> Fixnum {
        let xml_text = rtself.get_data(&*XML_TEXT_WRAPPER);
        Fixnum::new(i64::from(xml_text.len()))
    },
    fn yxml_text_next_sibling() -> AnyObject {
        let xml_text = rtself.get_data(&*XML_TEXT_WRAPPER);
        let xml = xml_text.next_sibling();

        match xml {
            Some(Xml::Element(v)) => Module::from_existing("Y")
                .get_nested_class("XMLElement")
                .wrap_data(v, &*XML_ELEMENT_WRAPPER),
            Some(Xml::Text(v)) => Module::from_existing("Y")
                .get_nested_class("XMLText")
                .wrap_data(v, &*XML_TEXT_WRAPPER),
            None => NilClass::new().to_any_object()
        }
    },
    fn yxml_text_observe(callback: Proc) -> Integer {
        let c = callback.map_err(|e| VM::raise_ex(e)).unwrap();

        let xml_text: &mut XmlText = rtself.get_data_mut(&*XML_TEXT_WRAPPER);
        let subscription_id: SubscriptionId = xml_text
            .observe(move |transaction, xml_text_event| {
                // TODO: add node changes
                let delta = xml_text_event.delta(transaction);
                for event in delta {
                    match event {
                        Delta::Inserted(v, attrs) => {
                            let mut payload = Hash::new();
                            payload.store(Symbol::new("insert"), map_yrs_value_to_ruby(v.clone()));

                            match attrs {
                                Some(a) => {
                                    let copy = a.clone();
                                    let result = map_attrs_to_hash(*copy).to_any_object();
                                    payload.store(Symbol::new("attributes"), result);
                                },
                                None => ()
                            }

                            let args = &[payload.to_any_object()];
                            c.call(args);
                        },
                        Delta::Retain(position, attrs) => {
                            let mut payload = Hash::new();
                            payload.store(Symbol::new("retain"), Integer::from(*position));

                            match attrs {
                                Some(a) => {
                                    let copy = a.clone();
                                    let result = map_attrs_to_hash(*copy).to_any_object();
                                    payload.store(Symbol::new("attributes"), result);
                                },
                                None => ()
                            }

                            let args = &[payload.to_any_object()];
                            c.call(args);
                        },
                        Delta::Deleted(position) => {
                            let mut payload = Hash::new();
                            payload.store(Symbol::new("delete"), Integer::from(*position));

                            let args = &[payload.to_any_object()];
                            c.call(args);
                        }
                    }
                }
            })
            .into();

        Integer::from(subscription_id)
    },
    fn yxml_text_parent() -> AnyObject {
        let xml_text = rtself.get_data(&*XML_TEXT_WRAPPER);
        let xml_element = xml_text.parent();

        match xml_element {
            Some(v) => Module::from_existing("Y")
                .get_nested_class("XMLElement")
                .wrap_data(v, &*XML_ELEMENT_WRAPPER),
            None => NilClass::new().to_any_object()
        }
    },
    fn yxml_text_prev_sibling() -> AnyObject {
        let xml_text = rtself.get_data(&*XML_TEXT_WRAPPER);
        let xml = xml_text.prev_sibling();

        match xml {
            Some(Xml::Element(v)) => Module::from_existing("Y")
                .get_nested_class("XMLElement")
                .wrap_data(v, &*XML_ELEMENT_WRAPPER),
            Some(Xml::Text(v)) => Module::from_existing("Y")
                .get_nested_class("XMLText")
                .wrap_data(v, &*XML_TEXT_WRAPPER),
            None => NilClass::new().to_any_object()
        }
    },
    fn yxml_text_push(transaction: YTransaction, content: RString) -> NilClass {
        let mut t = transaction.map_err(|e| VM::raise_ex(e)).unwrap();
        let tx = t.get_data_mut(&*TRANSACTION_WRAPPER);

        let c = content.map_err(|e| VM::raise_ex(e)).unwrap();

        let xml_text = rtself.get_data(&*XML_TEXT_WRAPPER);
        xml_text.push(tx, &c.to_string());

        NilClass::new()
    },
    fn yxml_text_remove_range(transaction: YTransaction, index: Fixnum, length: Fixnum) -> NilClass {
        let mut t = transaction.map_err(|e| VM::raise_ex(e)).unwrap();
        let tx = t.get_data_mut(&*TRANSACTION_WRAPPER);

        let i = index.map_err(|e| VM::raise_ex(e)).unwrap();
        let l = length.map_err(|e| VM::raise_ex(e)).unwrap();

        let xml_text = rtself.get_data(&*XML_TEXT_WRAPPER);
        xml_text.remove_range(tx, i.to_u32(), l.to_u32());

        NilClass::new()
    },
    fn yxml_text_to_string() -> RString {
        let xml_text = rtself.get_data(&*XML_TEXT_WRAPPER);
        RString::new_utf8(&xml_text.to_string())
    },
    fn yxml_text_unobserve(subscription_id: Integer) -> NilClass {
        let s = subscription_id.map_err(|e| VM::raise_ex(e)).unwrap();

        let xml_text: &mut XmlText = rtself.get_data_mut(&*XML_TEXT_WRAPPER);
        xml_text.unobserve(s.into());

        NilClass::new()
    }
);
