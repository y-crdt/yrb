extern crate core;

use crate::yarray::YArray;
use crate::yawareness::{YAwareness, YAwarenessEvent};
use crate::ydiff::YDiff;
use crate::ydoc::YDoc;
use crate::ymap::YMap;
use crate::ytext::YText;
use crate::ytransaction::YTransaction;
use crate::yxml_element::YXmlElement;
use crate::yxml_fragment::YXmlFragment;
use crate::yxml_text::YXmlText;

use magnus::{function, method, Error, Module, Object, Ruby};

mod utils;
mod yany;
mod yarray;
mod yattrs;
mod yawareness;
mod ydiff;
mod ydoc;
mod ymap;
mod ytext;
mod ytransaction;
mod yvalue;
mod yxml_element;
mod yxml_fragment;
mod yxml_text;

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("Y").expect("cannot define ::Y module");

    let yarray = module
        .define_class("Array", ruby.class_object())
        .expect("cannot find class Y::Array");

    yarray
        .define_private_method("yarray_each", method!(YArray::yarray_each, 2))
        .expect("cannot define private method: yarray_each");
    yarray
        .define_private_method("yarray_get", method!(YArray::yarray_get, 2))
        .expect("cannot define private method: yarray_get");
    yarray
        .define_private_method("yarray_insert", method!(YArray::yarray_insert, 3))
        .expect("cannot define private method: yarray_insert");
    yarray
        .define_private_method(
            "yarray_insert_range",
            method!(YArray::yarray_insert_range, 3),
        )
        .expect("cannot define private method: yarray_insert_range");
    yarray
        .define_private_method("yarray_length", method!(YArray::yarray_length, 1))
        .expect("cannot define private method: yarray_length");
    yarray
        .define_private_method("yarray_observe", method!(YArray::yarray_observe, 1))
        .expect("cannot define private method: yarray_observe");
    yarray
        .define_private_method("yarray_push_back", method!(YArray::yarray_push_back, 2))
        .expect("cannot define private method: yarray_push_back");
    yarray
        .define_private_method("yarray_push_front", method!(YArray::yarray_push_front, 2))
        .expect("cannot define private method: yarray_push_front");
    yarray
        .define_private_method("yarray_remove", method!(YArray::yarray_remove, 2))
        .expect("cannot define private method: yarray_remove");
    yarray
        .define_private_method(
            "yarray_remove_range",
            method!(YArray::yarray_remove_range, 3),
        )
        .expect("cannot define private method: yarray_remove_range");
    yarray
        .define_private_method("yarray_to_a", method!(YArray::yarray_to_a, 1))
        .expect("cannot define private method: yarray_to_a");
    yarray
        .define_private_method("yarray_unobserve", method!(YArray::yarray_unobserve, 1))
        .expect("cannot define private method: yarray_unobserve");

    let ydoc = module
        .define_class("Doc", ruby.class_object())
        .expect("cannot define class Y::Doc");
    ydoc.define_singleton_method("new", function!(YDoc::ydoc_new, -1))
        .expect("cannot define singleton method: ydoc_new");
    ydoc.define_private_method("ydoc_encode_diff_v1", method!(YDoc::ydoc_encode_diff_v1, 2))
        .expect("cannot define private method: ydoc_encode_diff_v1");
    ydoc.define_private_method("ydoc_encode_diff_v2", method!(YDoc::ydoc_encode_diff_v2, 2))
        .expect("cannot define private method: ydoc_encode_diff_v2");
    ydoc.define_private_method(
        "ydoc_get_or_insert_array",
        method!(YDoc::ydoc_get_or_insert_array, 1),
    )
    .expect("cannot define private method: ydoc_get_or_insert_array");
    ydoc.define_private_method(
        "ydoc_get_or_insert_map",
        method!(YDoc::ydoc_get_or_insert_map, 1),
    )
    .expect("cannot define private method: ydoc_get_or_insert_map");
    ydoc.define_private_method(
        "ydoc_get_or_insert_text",
        method!(YDoc::ydoc_get_or_insert_text, 1),
    )
    .expect("cannot define private method: ydoc_get_or_insert_text");
    ydoc.define_private_method(
        "ydoc_get_or_insert_xml_element",
        method!(YDoc::ydoc_get_or_insert_xml_element, 1),
    )
    .expect("cannot define private method: ydoc_get_or_insert_xml_element");
    ydoc.define_private_method(
        "ydoc_get_or_insert_xml_fragment",
        method!(YDoc::ydoc_get_or_insert_xml_fragment, 1),
    )
    .expect("cannot define private method: ydoc_get_or_insert_xml_fragment");
    ydoc.define_private_method(
        "ydoc_get_or_insert_xml_text",
        method!(YDoc::ydoc_get_or_insert_xml_text, 1),
    )
    .expect("cannot define private method: ydoc_get_or_insert_xml_text");
    ydoc.define_private_method("ydoc_transact", method!(YDoc::ydoc_transact, 0))
        .expect("cannot define private method: ydoc_transact");

    ydoc.define_private_method("ydoc_observe_update", method!(YDoc::ydoc_observe_update, 1))
        .expect("cannot define private method: ydoc_observe_update");

    let ymap = module
        .define_class("Map", ruby.class_object())
        .expect("cannot define class Y::Map");

    ymap.define_private_method("ymap_clear", method!(YMap::ymap_clear, 1))
        .expect("cannot define private method: ymap_clear");
    ymap.define_private_method("ymap_contains", method!(YMap::ymap_contains, 2))
        .expect("cannot define private method: ymap_contains");
    ymap.define_private_method("ymap_each", method!(YMap::ymap_each, 2))
        .expect("cannot define private method: ymap_each");
    ymap.define_private_method("ymap_get", method!(YMap::ymap_get, 2))
        .expect("cannot define private method: ymap_get");
    ymap.define_private_method("ymap_insert", method!(YMap::ymap_insert, 3))
        .expect("cannot define private method: ymap_insert");
    ymap.define_private_method("ymap_observe", method!(YMap::ymap_observe, 1))
        .expect("cannot define private method: ymap_observe");
    ymap.define_private_method("ymap_remove", method!(YMap::ymap_remove, 2))
        .expect("cannot define private method: ymap_remove");
    ymap.define_private_method("ymap_size", method!(YMap::ymap_size, 1))
        .expect("cannot define private method: ymap_size");
    ymap.define_private_method("ymap_to_h", method!(YMap::ymap_to_h, 1))
        .expect("cannot define private method: ymap_to_h");
    ymap.define_private_method("ymap_unobserve", method!(YMap::ymap_unobserve, 1))
        .expect("cannot define private method: ymap_unobserve");

    let ytransaction = module
        .define_class("Transaction", ruby.class_object())
        .expect("cannot define class Y::Transaction");

    ytransaction
        .define_private_method(
            "ytransaction_apply_update",
            method!(YTransaction::ytransaction_apply_update, 1),
        )
        .expect("cannot define private method: ytransaction_apply_update");
    ytransaction
        .define_private_method(
            "ytransaction_apply_update_v2",
            method!(YTransaction::ytransaction_apply_update_v2, 1),
        )
        .expect("cannot define private method: ytransaction_apply_update_v2");
    ytransaction
        .define_private_method(
            "ytransaction_commit",
            method!(YTransaction::ytransaction_commit, 0),
        )
        .expect("cannot define private method: ytransaction_commit");
    ytransaction
        .define_method("free", method!(YTransaction::ytransaction_free, 0))
        .expect("");
    ytransaction
        .define_private_method(
            "ytransaction_get_array",
            method!(YTransaction::ytransaction_get_array, 1),
        )
        .expect("cannot define private method: ytransaction_get_array");
    ytransaction
        .define_private_method(
            "ytransaction_get_map",
            method!(YTransaction::ytransaction_get_map, 1),
        )
        .expect("cannot define private method: ytransaction_get_map");
    ytransaction
        .define_private_method(
            "ytransaction_get_text",
            method!(YTransaction::ytransaction_get_text, 1),
        )
        .expect("cannot define private method: ytransaction_get_text");
    ytransaction
        .define_private_method(
            "ytransaction_get_xml_element",
            method!(YTransaction::ytransaction_get_xml_element, 1),
        )
        .expect("cannot define private method: ytransaction_get_xml_element");
    ytransaction
        .define_private_method(
            "ytransaction_get_xml_fragment",
            method!(YTransaction::ytransaction_get_xml_fragment, 1),
        )
        .expect("cannot define private method: ytransaction_get_xml_fragment");
    ytransaction
        .define_private_method(
            "ytransaction_get_xml_text",
            method!(YTransaction::ytransaction_get_xml_text, 1),
        )
        .expect("cannot define private method: ytransaction_get_xml_text");
    ytransaction
        .define_private_method(
            "ytransaction_state_vector",
            method!(YTransaction::ytransaction_state_vector, 0),
        )
        .expect("cannot define private method: ytransaction_state_vector");
    ytransaction
        .define_private_method(
            "ytransaction_state_vector_v2",
            method!(YTransaction::ytransaction_state_vector_v2, 0),
        )
        .expect("cannot define private method: ytransaction_state_vector_v2");

    let ytext = module
        .define_class("Text", ruby.class_object())
        .expect("cannot define class Y::Text");

    ytext
        .define_private_method("ytext_diff", method!(YText::ytext_diff, 1))
        .expect("cannot define private method: ytext_diff");
    ytext
        .define_private_method("ytext_format", method!(YText::ytext_format, 4))
        .expect("cannot define private method: ytext_format");
    ytext
        .define_private_method("ytext_insert", method!(YText::ytext_insert, 3))
        .expect("cannot define private method: ytext_insert");
    ytext
        .define_private_method("ytext_insert_embed", method!(YText::ytext_insert_embed, 3))
        .expect("cannot define private method: ytext_insert_embed");
    ytext
        .define_private_method(
            "ytext_insert_embed_with_attributes",
            method!(YText::ytext_insert_embed_with_attributes, 4),
        )
        .expect("cannot define private method: ytext_insert_embed_with_attributes");
    ytext
        .define_private_method(
            "ytext_insert_with_attributes",
            method!(YText::ytext_insert_with_attributes, 4),
        )
        .expect("cannot define private method: ytext_insert_with_attributes");
    ytext
        .define_private_method("ytext_length", method!(YText::ytext_length, 1))
        .expect("cannot define private method: ytext_length");
    ytext
        .define_private_method("ytext_observe", method!(YText::ytext_observe, 1))
        .expect("cannot define private method: ytext_observe");
    ytext
        .define_private_method("ytext_push", method!(YText::ytext_push, 2))
        .expect("cannot define private method: ytext_push");
    ytext
        .define_private_method("ytext_remove_range", method!(YText::ytext_remove_range, 3))
        .expect("cannot define private method: ytext_remove_range");
    ytext
        .define_private_method("ytext_to_s", method!(YText::ytext_to_s, 1))
        .expect("cannot define private method: ytext_to_s");
    ytext
        .define_private_method("ytext_unobserve", method!(YText::ytext_unobserve, 1))
        .expect("cannot define private method: ytext_unobserve");

    let yxml_element = module
        .define_class("XMLElement", ruby.class_object())
        .expect("cannot define class Y::XMLElement");

    yxml_element
        .define_private_method(
            "yxml_element_attributes",
            method!(YXmlElement::yxml_element_attributes, 1),
        )
        .expect("cannot define private method: yxml_element_attributes");
    yxml_element
        .define_private_method(
            "yxml_element_first_child",
            method!(YXmlElement::yxml_element_first_child, 1),
        )
        .expect("cannot define private method: yxml_element_first_child");
    yxml_element
        .define_private_method(
            "yxml_element_get",
            method!(YXmlElement::yxml_element_get, 2),
        )
        .expect("cannot define private method: yxml_element_get");
    yxml_element
        .define_private_method(
            "yxml_element_get_attribute",
            method!(YXmlElement::yxml_element_get_attribute, 2),
        )
        .expect("cannot define private method: yxml_element_get_attribute");
    yxml_element
        .define_private_method(
            "yxml_element_insert_attribute",
            method!(YXmlElement::yxml_element_insert_attribute, 3),
        )
        .expect("cannot define private method: yxml_element_insert_attribute");
    yxml_element
        .define_private_method(
            "yxml_element_insert_element",
            method!(YXmlElement::yxml_element_insert_element, 3),
        )
        .expect("cannot define private method: yxml_element_insert_element");
    yxml_element
        .define_private_method(
            "yxml_element_insert_text",
            method!(YXmlElement::yxml_element_insert_text, 3),
        )
        .expect("cannot define private method: yxml_element_insert_text");
    yxml_element
        .define_private_method(
            "yxml_element_len",
            method!(YXmlElement::yxml_element_len, 1),
        )
        .expect("cannot define private method: yxml_element_len");
    yxml_element
        .define_private_method(
            "yxml_element_next_sibling",
            method!(YXmlElement::yxml_element_next_sibling, 1),
        )
        .expect("cannot define private method: yxml_element_next_sibling");
    yxml_element
        .define_private_method(
            "yxml_element_observe",
            method!(YXmlElement::yxml_element_observe, 1),
        )
        .expect("cannot define private method: yxml_element_observe");
    yxml_element
        .define_private_method(
            "yxml_element_parent",
            method!(YXmlElement::yxml_element_parent, 0),
        )
        .expect("cannot define private method: yxml_element_parent");
    yxml_element
        .define_private_method(
            "yxml_element_prev_sibling",
            method!(YXmlElement::yxml_element_prev_sibling, 1),
        )
        .expect("cannot define private method: yxml_element_prev_sibling");
    yxml_element
        .define_private_method(
            "yxml_element_push_element_back",
            method!(YXmlElement::yxml_element_push_element_back, 2),
        )
        .expect("cannot define private method: yxml_element_push_element_back");
    yxml_element
        .define_private_method(
            "yxml_element_push_element_front",
            method!(YXmlElement::yxml_element_push_element_front, 2),
        )
        .expect("cannot define private method: yxml_element_push_element_front");
    yxml_element
        .define_private_method(
            "yxml_element_push_text_back",
            method!(YXmlElement::yxml_element_push_text_back, 2),
        )
        .expect("cannot define private method: yxml_element_push_text_back");
    yxml_element
        .define_private_method(
            "yxml_element_push_text_front",
            method!(YXmlElement::yxml_element_push_text_front, 2),
        )
        .expect("cannot define private method: yxml_element_push_text_front");
    yxml_element
        .define_private_method(
            "yxml_element_remove_attribute",
            method!(YXmlElement::yxml_element_remove_attribute, 2),
        )
        .expect("cannot define private method: yxml_element_remove_range");
    yxml_element
        .define_private_method(
            "yxml_element_remove_range",
            method!(YXmlElement::yxml_element_remove_range, 3),
        )
        .expect("cannot define private method: yxml_element_remove_range");
    yxml_element
        .define_private_method(
            "yxml_element_siblings",
            method!(YXmlElement::yxml_element_siblings, 1),
        )
        .expect("cannot define private method: yxml_element_siblings");
    yxml_element
        .define_private_method(
            "yxml_element_size",
            method!(YXmlElement::yxml_element_size, 1),
        )
        .expect("cannot define private method: yxml_element_size");
    yxml_element
        .define_private_method(
            "yxml_element_tag",
            method!(YXmlElement::yxml_element_tag, 0),
        )
        .expect("cannot define private method: yxml_element_tag");
    yxml_element
        .define_private_method(
            "yxml_element_to_s",
            method!(YXmlElement::yxml_element_to_s, 1),
        )
        .expect("cannot define private method: yxml_element_to_s");
    yxml_element
        .define_private_method(
            "yxml_element_unobserve",
            method!(YXmlElement::yxml_element_unobserve, 1),
        )
        .expect("cannot define private method: yxml_element_unobserve");

    let yxml_fragment = module
        .define_class("XMLFragment", ruby.class_object())
        .expect("cannot define class: Y::XMLFragment");

    yxml_fragment
        .define_private_method(
            "yxml_fragment_first_child",
            method!(YXmlFragment::yxml_fragment_first_child, 0),
        )
        .expect("cannot define private method: yxml_fragment_first_child");
    yxml_fragment
        .define_private_method(
            "yxml_fragment_get",
            method!(YXmlFragment::yxml_fragment_get, 2),
        )
        .expect("cannot define private method: yxml_fragment_get");
    yxml_fragment
        .define_private_method(
            "yxml_fragment_insert",
            method!(YXmlFragment::yxml_fragment_insert, 3),
        )
        .expect("cannot define private method: yxml_fragment_insert");
    yxml_fragment
        .define_private_method(
            "yxml_fragment_len",
            method!(YXmlFragment::yxml_fragment_len, 1),
        )
        .expect("cannot define private method: yxml_fragment_len");
    yxml_fragment
        .define_private_method(
            "yxml_fragment_parent",
            method!(YXmlFragment::yxml_fragment_parent, 0),
        )
        .expect("cannot define private method: yxml_fragment_parent");
    yxml_fragment
        .define_private_method(
            "yxml_fragment_push_back",
            method!(YXmlFragment::yxml_fragment_push_back, 2),
        )
        .expect("cannot define private method: yxml_fragment_push_back");
    yxml_fragment
        .define_private_method(
            "yxml_fragment_push_front",
            method!(YXmlFragment::yxml_fragment_push_front, 2),
        )
        .expect("cannot define private method: yxml_fragment_push_front");
    yxml_fragment
        .define_private_method(
            "yxml_fragment_remove_range",
            method!(YXmlFragment::yxml_fragment_remove_range, 3),
        )
        .expect("cannot define private method: yxml_fragment_remove_range");
    yxml_fragment
        .define_private_method(
            "yxml_fragment_successors",
            method!(YXmlFragment::yxml_fragment_successors, 1),
        )
        .expect("cannot define private method: yxml_fragment_successors");
    yxml_fragment
        .define_private_method(
            "yxml_fragment_to_s",
            method!(YXmlFragment::yxml_fragment_to_s, 1),
        )
        .expect("cannot define private method: yxml_fragment_to_s");

    let yxml_text = module
        .define_class("XMLText", ruby.class_object())
        .expect("cannot define class Y::XMLText");

    yxml_text
        .define_private_method(
            "yxml_text_attributes",
            method!(YXmlText::yxml_text_attributes, 1),
        )
        .expect("cannot define private method: yxml_text_attributes");
    yxml_text
        .define_private_method("yxml_text_format", method!(YXmlText::yxml_text_format, 4))
        .expect("cannot define private method: yxml_text_format");
    yxml_text
        .define_private_method(
            "yxml_text_get_attribute",
            method!(YXmlText::yxml_text_get_attribute, 2),
        )
        .expect("cannot define private method: yxml_text_get_attribute");
    yxml_text
        .define_private_method("yxml_text_insert", method!(YXmlText::yxml_text_insert, 3))
        .expect("cannot define private method: yxml_text_insert");
    yxml_text
        .define_private_method(
            "yxml_text_insert_attribute",
            method!(YXmlText::yxml_text_insert_attribute, 3),
        )
        .expect("cannot define private method: yxml_text_insert_attribute");
    yxml_text
        .define_private_method(
            "yxml_text_insert_embed_with_attrs",
            method!(YXmlText::yxml_text_insert_embed_with_attributes, 4),
        )
        .expect("cannot define private method: yxml_text_insert_embed_with_attributes");
    yxml_text
        .define_private_method(
            "yxml_text_insert_with_attrs",
            method!(YXmlText::yxml_text_insert_with_attributes, 4),
        )
        .expect("cannot define private method: yxml_text_insert_with_attributes");
    yxml_text
        .define_private_method(
            "yxml_text_insert_embed",
            method!(YXmlText::yxml_text_insert_embed, 3),
        )
        .expect("cannot define private method: yxml_text_insert_embed");
    yxml_text
        .define_private_method("yxml_text_length", method!(YXmlText::yxml_text_length, 1))
        .expect("cannot define private method: yxml_text_length");
    yxml_text
        .define_private_method(
            "yxml_text_next_sibling",
            method!(YXmlText::yxml_text_next_sibling, 1),
        )
        .expect("cannot define private method: yxml_text_next_sibling");
    yxml_text
        .define_private_method("yxml_text_parent", method!(YXmlText::yxml_text_parent, 0))
        .expect("cannot define private method: yxml_text_parent");
    yxml_text
        .define_private_method(
            "yxml_text_prev_sibling",
            method!(YXmlText::yxml_text_prev_sibling, 1),
        )
        .expect("cannot define private method: yxml_text_prev_sibling");
    yxml_text
        .define_private_method("yxml_text_push", method!(YXmlText::yxml_text_push, 2))
        .expect("cannot define private method: yxml_text_push");
    yxml_text
        .define_private_method(
            "yxml_text_remove_range",
            method!(YXmlText::yxml_text_remove_range, 3),
        )
        .expect("cannot define private method: yxml_text_remove_range");
    yxml_text
        .define_private_method("yxml_text_to_s", method!(YXmlText::yxml_text_to_s, 1))
        .expect("cannot define private method: yxml_text_to_s");

    let yawareness = module
        .define_class("Awareness", ruby.class_object())
        .expect("cannot define class Y::Awareness");
    yawareness
        .define_singleton_method("new", function!(YAwareness::yawareness_new, 0))
        .expect("cannot define singleton method: yawareness_new");
    yawareness
        .define_private_method(
            "yawareness_apply_update",
            method!(YAwareness::yawareness_apply_update, 1),
        )
        .expect("cannot define private method: yawareness_apply_update");
    yawareness
        .define_private_method(
            "yawareness_clean_local_state",
            method!(YAwareness::yawareness_clean_local_state, 0),
        )
        .expect("cannot define private method: yawareness_clean_local_state");
    yawareness
        .define_private_method(
            "yawareness_clients",
            method!(YAwareness::yawareness_clients, 0),
        )
        .expect("cannot define private method: yawareness_clients");
    yawareness
        .define_private_method(
            "yawareness_client_id",
            method!(YAwareness::yawareness_client_id, 0),
        )
        .expect("cannot define private method: yawareness_client_id");
    yawareness
        .define_private_method(
            "yawareness_local_state",
            method!(YAwareness::yawareness_local_state, 0),
        )
        .expect("cannot define private method: yawareness_local_state");
    yawareness
        .define_private_method(
            "yawareness_on_update",
            method!(YAwareness::yawareness_on_update, 1),
        )
        .expect("cannot define private method: yawareness_on_update");
    yawareness
        .define_private_method(
            "yawareness_remove_state",
            method!(YAwareness::yawareness_remove_state, 1),
        )
        .expect("cannot define private method: yawareness_remove_state");
    yawareness
        .define_private_method(
            "yawareness_set_local_state",
            method!(YAwareness::yawareness_set_local_state, 1),
        )
        .expect("cannot define private method: yawareness_set_local_state");
    yawareness
        .define_private_method(
            "yawareness_update",
            method!(YAwareness::yawareness_update, 0),
        )
        .expect("cannot define private method: yawareness_update");
    yawareness
        .define_private_method(
            "yawareness_update_with_clients",
            method!(YAwareness::yawareness_update_with_clients, 1),
        )
        .expect("cannot define private method: yawareness_update_with_clients");

    let yawareness_event = module
        .define_class("AwarenessEvent", ruby.class_object())
        .expect("cannot define class Y::AwarenessEvent");
    yawareness_event
        .define_method("added", method!(YAwarenessEvent::added, 0))
        .expect("cannot define private method: added");
    yawareness_event
        .define_method("updated", method!(YAwarenessEvent::updated, 0))
        .expect("cannot define private method: updated");
    yawareness_event
        .define_method("removed", method!(YAwarenessEvent::removed, 0))
        .expect("cannot define private method: removed");

    let ydiff = module
        .define_class("Diff", ruby.class_object())
        .expect("cannot define class Y::Diff");
    ydiff
        .define_private_method("ydiff_insert", method!(YDiff::ydiff_insert, 0))
        .expect("cannot define private method: insert");
    ydiff
        .define_private_method("ydiff_attrs", method!(YDiff::ydiff_attrs, 0))
        .expect("cannot define private method: attrs");

    Ok(())
}
