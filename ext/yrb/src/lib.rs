use crate::yarray::YArray;
use crate::ydoc::YDoc;
use crate::ymap::YMap;
use crate::ytext::YText;
use crate::ytransaction::YTransaction;
use crate::yxml_element::YXmlElement;
use crate::yxml_text::YXmlText;
use magnus::{define_module, function, method, Error, Module, Object};

mod utils;
mod yany;
mod yarray;
mod yattrs;
mod ydoc;
mod ymap;
mod ytext;
mod ytransaction;
mod yvalue;
mod yxml_element;
mod yxml_text;

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("Y").expect("cannot define ::Y module");

    let yarray = module
        .define_class("Array", Default::default())
        .expect("cannot find class Y::Array");

    yarray
        .define_private_method("yarray_each", method!(YArray::yarray_each, 1))
        .expect("cannot define private method: yarray_each");
    yarray
        .define_private_method("yarray_get", method!(YArray::yarray_get, 1))
        .expect("cannot define private method: yarray_get");
    yarray
        .define_private_method(
            "yarray_insert",
            method!(YArray::yarray_insert, 3)
        )
        .expect("cannot define private method: yarray_insert");
    yarray
        .define_private_method(
            "yarray_insert_range",
            method!(YArray::yarray_insert_range, 3)
        )
        .expect("cannot define private method: yarray_insert_range");
    yarray
        .define_private_method(
            "yarray_length",
            method!(YArray::yarray_length, 0)
        )
        .expect("cannot define private method: yarray_length");
    yarray
        .define_private_method(
            "yarray_observe",
            method!(YArray::yarray_observe, 1)
        )
        .expect("cannot define private method: yarray_observe");
    yarray
        .define_private_method(
            "yarray_push_back",
            method!(YArray::yarray_push_back, 2)
        )
        .expect("cannot define private method: yarray_push_back");
    yarray
        .define_private_method(
            "yarray_push_front",
            method!(YArray::yarray_push_front, 2)
        )
        .expect("cannot define private method: yarray_push_front");
    yarray
        .define_private_method(
            "yarray_remove",
            method!(YArray::yarray_remove, 2)
        )
        .expect("cannot define private method: yarray_remove");
    yarray
        .define_private_method(
            "yarray_remove_range",
            method!(YArray::yarray_remove_range, 3)
        )
        .expect("cannot define private method: yarray_remove_range");
    yarray
        .define_private_method("yarray_to_a", method!(YArray::yarray_to_a, 0))
        .expect("cannot define private method: yarray_to_a");
    yarray
        .define_private_method(
            "yarray_unobserve",
            method!(YArray::yarray_unobserve, 1)
        )
        .expect("cannot define private method: yarray_unobserve");

    let ydoc = module
        .define_class("Doc", Default::default())
        .expect("cannot define class Y::Doc");
    ydoc.define_singleton_method("new", function!(YDoc::ydoc_new, -1))
        .expect("cannot define singleton method: ydoc_new");
    ydoc.define_private_method(
        "ydoc_transact",
        method!(YDoc::ydoc_transact, 0)
    )
    .expect("cannot define private method: ydoc_transact");
    ydoc.define_private_method(
        "ydoc_encode_diff_v1",
        method!(YDoc::ydoc_encode_diff_v1, 1)
    )
    .expect("cannot define private method: ydoc_encode_diff_v1");

    let ymap = module
        .define_class("Map", Default::default())
        .expect("cannot define class Y::Map");

    ymap.define_private_method("ymap_clear", method!(YMap::ymap_clear, 1))
        .expect("cannot define private method: ymap_clear");
    ymap.define_private_method(
        "ymap_contains",
        method!(YMap::ymap_contains, 1)
    )
    .expect("cannot define private method: ymap_contains");
    ymap.define_private_method("ymap_each", method!(YMap::ymap_each, 1))
        .expect("cannot define private method: ymap_each");
    ymap.define_private_method("ymap_get", method!(YMap::ymap_get, 1))
        .expect("cannot define private method: ymap_get");
    ymap.define_private_method("ymap_insert", method!(YMap::ymap_insert, 3))
        .expect("cannot define private method: ymap_insert");
    ymap.define_private_method("ymap_observe", method!(YMap::ymap_observe, 1))
        .expect("cannot define private method: ymap_observe");
    ymap.define_private_method("ymap_remove", method!(YMap::ymap_remove, 2))
        .expect("cannot define private method: ymap_remove");
    ymap.define_private_method("ymap_size", method!(YMap::ymap_size, 0))
        .expect("cannot define private method: ymap_size");
    ymap.define_private_method("ymap_to_h", method!(YMap::ymap_to_h, 0))
        .expect("cannot define private method: ymap_to_h");
    ymap.define_private_method(
        "ymap_unobserve",
        method!(YMap::ymap_unobserve, 1)
    )
    .expect("cannot define private method: ymap_unobserve");

    let ytransaction = module
        .define_class("Transaction", Default::default())
        .expect("cannot define class Y::Transaction");

    ytransaction
        .define_private_method(
            "ytransaction_apply_update",
            method!(YTransaction::ytransaction_apply_update, 1)
        )
        .expect("cannot define private method: ytransaction_apply_update");
    ytransaction
        .define_private_method(
            "ytransaction_commit",
            method!(YTransaction::ytransaction_commit, 0)
        )
        .expect("cannot define private method: ytransaction_commit");
    ytransaction
        .define_private_method(
            "ytransaction_get_array",
            method!(YTransaction::ytransaction_get_array, 1)
        )
        .expect("cannot define private method: ytransaction_get_array");
    ytransaction
        .define_private_method(
            "ytransaction_get_map",
            method!(YTransaction::ytransaction_get_map, 1)
        )
        .expect("cannot define private method: ytransaction_get_mao");
    ytransaction
        .define_private_method(
            "ytransaction_get_text",
            method!(YTransaction::ytransaction_get_text, 1)
        )
        .expect("cannot define private method: ytransaction_get_text");
    ytransaction
        .define_private_method(
            "ytransaction_get_xml_element",
            method!(YTransaction::ytransaction_get_xml_element, 1)
        )
        .expect("cannot define private method: ytransaction_get_xml_element");
    ytransaction
        .define_private_method(
            "ytransaction_get_xml_text",
            method!(YTransaction::ytransaction_get_xml_text, 1)
        )
        .expect("cannot define private method: ytransaction_get_xml_text");
    ytransaction
        .define_private_method(
            "ytransaction_state_vector",
            method!(YTransaction::ytransaction_state_vector, 0)
        )
        .expect("cannot define private method: ytransaction_state_vector");

    let ytext = module
        .define_class("Text", Default::default())
        .expect("cannot define class Y::Text");

    ytext
        .define_private_method("ytext_format", method!(YText::ytext_format, 4))
        .expect("cannot define private method: ytext_format");
    ytext
        .define_private_method("ytext_insert", method!(YText::ytext_insert, 3))
        .expect("cannot define private method: ytext_insert");
    ytext
        .define_private_method(
            "ytext_insert_embed",
            method!(YText::ytext_insert_embed, 3)
        )
        .expect("cannot define private method: ytext_insert_embed");
    ytext
        .define_private_method(
            "ytext_insert_embed_with_attributes",
            method!(YText::ytext_insert_embed_with_attributes, 4)
        )
        .expect(
            "cannot define private method: ytext_insert_embed_with_attributes"
        );
    ytext
        .define_private_method(
            "ytext_insert_with_attributes",
            method!(YText::ytext_insert_with_attributes, 4)
        )
        .expect("cannot define private method: ytext_insert_with_attributes");
    ytext
        .define_private_method("ytext_length", method!(YText::ytext_length, 0))
        .expect("cannot define private method: ytext_length");
    ytext
        .define_private_method(
            "ytext_observe",
            method!(YText::ytext_observe, 1)
        )
        .expect("cannot define private method: ytext_observe");
    ytext
        .define_private_method("ytext_push", method!(YText::ytext_push, 2))
        .expect("cannot define private method: ytext_push");
    ytext
        .define_private_method(
            "ytext_remove_range",
            method!(YText::ytext_remove_range, 3)
        )
        .expect("cannot define private method: ytext_remove_range");
    ytext
        .define_private_method("ytext_to_s", method!(YText::ytext_to_s, 0))
        .expect("cannot define private method: ytext_to_s");
    ytext
        .define_private_method(
            "ytext_unobserve",
            method!(YText::ytext_unobserve, 1)
        )
        .expect("cannot define private method: ytext_unobserve");

    let yxml_element = module
        .define_class("XMLElement", Default::default())
        .expect("cannot define class Y::XMLElement");

    yxml_element
        .define_private_method(
            "yxml_element_attributes",
            method!(YXmlElement::yxml_element_attributes, 0)
        )
        .expect("cannot define private method: yxml_element_attributes");
    yxml_element
        .define_private_method(
            "yxml_element_first_child",
            method!(YXmlElement::yxml_element_first_child, 0)
        )
        .expect("cannot define private method: yxml_element_first_child");
    yxml_element
        .define_private_method(
            "yxml_element_get",
            method!(YXmlElement::yxml_element_get, 1)
        )
        .expect("cannot define private method: yxml_element_get");
    yxml_element
        .define_private_method(
            "yxml_element_get_attribute",
            method!(YXmlElement::yxml_element_get_attribute, 1)
        )
        .expect("cannot define private method: yxml_element_get_attribute");
    yxml_element
        .define_private_method(
            "yxml_element_insert_attribute",
            method!(YXmlElement::yxml_element_insert_attribute, 3)
        )
        .expect("cannot define private method: yxml_element_insert_attribute");
    yxml_element
        .define_private_method(
            "yxml_element_insert_element",
            method!(YXmlElement::yxml_element_insert_element, 3)
        )
        .expect("cannot define private method: yxml_element_insert_element");
    yxml_element
        .define_private_method(
            "yxml_element_insert_text",
            method!(YXmlElement::yxml_element_insert_text, 2)
        )
        .expect("cannot define private method: yxml_element_insert_text");
    yxml_element
        .define_private_method(
            "yxml_element_next_sibling",
            method!(YXmlElement::yxml_element_next_sibling, 0)
        )
        .expect("cannot define private method: yxml_element_next_sibling");
    yxml_element
        .define_private_method(
            "yxml_element_observe",
            method!(YXmlElement::yxml_element_observe, 1)
        )
        .expect("cannot define private method: yxml_element_observe");
    yxml_element
        .define_private_method(
            "yxml_element_parent",
            method!(YXmlElement::yxml_element_parent, 0)
        )
        .expect("cannot define private method: yxml_element_parent");
    yxml_element
        .define_private_method(
            "yxml_element_prev_sibling",
            method!(YXmlElement::yxml_element_prev_sibling, 0)
        )
        .expect("cannot define private method: yxml_element_prev_sibling");
    yxml_element
        .define_private_method(
            "yxml_element_push_elem_back",
            method!(YXmlElement::yxml_element_push_element_back, 2)
        )
        .expect("cannot define private method: yxml_element_push_elem_back");
    yxml_element
        .define_private_method(
            "yxml_element_push_elem_front",
            method!(YXmlElement::yxml_element_push_element_front, 2)
        )
        .expect("cannot define private method: yxml_element_push_elem_front");
    yxml_element
        .define_private_method(
            "yxml_element_push_text_back",
            method!(YXmlElement::yxml_element_push_text_back, 1)
        )
        .expect("cannot define private method: yxml_element_push_text_back");
    yxml_element
        .define_private_method(
            "yxml_element_push_text_front",
            method!(YXmlElement::yxml_element_push_text_front, 1)
        )
        .expect("cannot define private method: yxml_element_push_text_front");
    yxml_element
        .define_private_method(
            "yxml_element_remove_attribute",
            method!(YXmlElement::yxml_element_remove_attribute, 2)
        )
        .expect("cannot define private method: yxml_element_remove_range");
    yxml_element
        .define_private_method(
            "yxml_element_remove_range",
            method!(YXmlElement::yxml_element_remove_range, 3)
        )
        .expect("cannot define private method: yxml_element_remove_range");
    yxml_element
        .define_private_method(
            "yxml_element_size",
            method!(YXmlElement::yxml_element_size, 0)
        )
        .expect("cannot define private method: yxml_element_size");
    yxml_element
        .define_private_method(
            "yxml_element_tag",
            method!(YXmlElement::yxml_element_tag, 0)
        )
        .expect("cannot define private method: yxml_element_tag");
    yxml_element
        .define_private_method(
            "yxml_element_to_s",
            method!(YXmlElement::yxml_element_to_s, 0)
        )
        .expect("cannot define private method: yxml_element_to_s");
    yxml_element
        .define_private_method(
            "yxml_element_unobserve",
            method!(YXmlElement::yxml_element_unobserve, 1)
        )
        .expect("cannot define private method: yxml_element_unobserve");

    let yxml_text = module
        .define_class("XMLText", Default::default())
        .expect("cannot define class Y::XMLText");

    yxml_text
        .define_private_method(
            "yxml_text_attributes",
            method!(YXmlText::yxml_text_attributes, 0)
        )
        .expect("cannot define private method: yxml_text_attributes");
    yxml_text
        .define_private_method(
            "yxml_text_format",
            method!(YXmlText::yxml_text_format, 4)
        )
        .expect("cannot define private method: yxml_text_format");
    yxml_text
        .define_private_method(
            "yxml_text_get_attribute",
            method!(YXmlText::yxml_text_get_attribute, 1)
        )
        .expect("cannot define private method: yxml_text_get_attribute");
    yxml_text
        .define_private_method(
            "yxml_text_insert",
            method!(YXmlText::yxml_text_insert, 3)
        )
        .expect("cannot define private method: yxml_text_insert");
    yxml_text
        .define_private_method(
            "yxml_text_insert_attribute",
            method!(YXmlText::yxml_text_insert_attribute, 3)
        )
        .expect("cannot define private method: yxml_text_insert_attribute");
    yxml_text.define_private_method("yxml_text_insert_embed_with_attrs", method!(YXmlText::yxml_text_insert_embed_with_attributes, 4)).expect("cannot define private method: yxml_text_insert_embed_with_attributes");
    yxml_text
        .define_private_method(
            "yxml_text_insert_with_attrs",
            method!(YXmlText::yxml_text_insert_with_attributes, 4)
        )
        .expect(
            "cannot define private method: yxml_text_insert_with_attributes"
        );
    yxml_text
        .define_private_method(
            "yxml_text_insert_embed",
            method!(YXmlText::yxml_text_insert_embed, 3)
        )
        .expect("cannot define private method: yxml_text_insert_embed");
    yxml_text
        .define_private_method(
            "yxml_text_length",
            method!(YXmlText::yxml_text_length, 0)
        )
        .expect("cannot define private method: yxml_text_length");
    yxml_text
        .define_private_method(
            "yxml_text_next_sibling",
            method!(YXmlText::yxml_text_next_sibling, 0)
        )
        .expect("cannot define private method: yxml_text_next_sibling");
    yxml_text
        .define_private_method(
            "yxml_text_parent",
            method!(YXmlText::yxml_text_parent, 0)
        )
        .expect("cannot define private method: yxml_text_parent");
    yxml_text
        .define_private_method(
            "yxml_text_prev_sibling",
            method!(YXmlText::yxml_text_prev_sibling, 0)
        )
        .expect("cannot define private method: yxml_text_prev_sibling");
    yxml_text
        .define_private_method(
            "yxml_text_push",
            method!(YXmlText::yxml_text_push, 2)
        )
        .expect("cannot define private method: yxml_text_push");
    yxml_text
        .define_private_method(
            "yxml_text_remove_range",
            method!(YXmlText::yxml_text_remove_range, 3)
        )
        .expect("cannot define private method: yxml_text_remove_range");
    yxml_text
        .define_private_method(
            "yxml_text_to_s",
            method!(YXmlText::yxml_text_to_s, 0)
        )
        .expect("cannot define private method: yxml_text_to_s");

    Ok(())
}
