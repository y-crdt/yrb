mod util;
mod yarray;
mod ydoc;
mod ymap;
mod ytext;
mod ytransaction;
mod yxml;

#[macro_use]
extern crate rutie;

use rutie::{Module, Object};

module!(Y);

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_yrb() {
    Module::new("Y").define(|module| {
        module.define_nested_class("Array", None).define(|klass| {
            klass.def("length", yarray::yarray_length);
            klass.def("insert", yarray::yarray_insert);
            klass.def("remove", yarray::yarray_remove);
            klass.def("remove_range", yarray::yarray_remove_range);
            klass.def("to_arr", yarray::yarray_to_arr);
        });

        module.define_nested_class("Doc", None).define(|klass| {
            klass.def_self("new", ydoc::ydoc_new);
            klass.def("transact", ydoc::ydoc_transact);
            klass.def("state_vector", ydoc::ydoc_state_vector);
            klass.def("encode_diff_v1", ydoc::ydoc_encode_diff_v1);
        });

        module.define_nested_class("Map", None).define(|klass| {
            klass.def_private("ymap_clear", ymap::ymap_clear);
            klass.def_private("ymap_contains", ymap::ymap_contains);
            klass.def_private("ymap_each", ymap::ymap_each);
            klass.def_private("ymap_get", ymap::ymap_get);
            klass.def_private("ymap_insert", ymap::ymap_insert);
            klass.def_private("ymap_remove", ymap::ymap_remove);
            klass.def_private("ymap_size", ymap::ymap_size);
            klass.def_private("ymap_to_h", ymap::ymap_to_hash);
        });

        module.define_nested_class("Text", None).define(|klass| {
            klass.def_private("ytext_insert", ytext::ytext_insert);
            klass.def_private("ytext_insert_embed", ytext::ytext_insert_embed);
            klass.def_private(
                "ytext_insert_embed_with_attrs",
                ytext::ytext_insert_embed_with_attributes,
            );
            klass.def_private(
                "ytext_insert_with_attrs",
                ytext::ytext_insert_with_attributes,
            );
            klass.def_private("ytext_remove_range", ytext::ytext_remove_range);
            klass.def_private("ytext_format", ytext::ytext_format);
            klass.def_private("ytext_length", ytext::ytext_length);
            klass.def_private("ytext_push", ytext::ytext_push);
            klass.def_private("ytext_changes", ytext::ytext_changes);
            klass.def_private("ytext_to_s", ytext::ytext_to_string);
        });

        module
            .define_nested_class("Transaction", None)
            .define(|klass| {
                klass.def(
                    "apply_update",
                    ytransaction::ytransaction_apply_update,
                );
                klass.def("commit", ytransaction::ytransaction_commit);
                klass.def("get_array", ytransaction::ytransaction_get_array);
                klass.def("get_map", ytransaction::ytransaction_get_map);
                klass.def("get_text", ytransaction::ytransaction_get_text);
                klass.def(
                    "get_xml_element",
                    ytransaction::ytransaction_get_xml_element,
                );
                klass.def(
                    "get_xml_text",
                    ytransaction::ytransaction_get_xml_text,
                );
                klass.def(
                    "state_vector",
                    ytransaction::ytransaction_state_vector,
                );
            });

        module
            .define_nested_class("XMLElement", None)
            .define(|klass| {
                klass.def("attributes", yxml::yxml_element_attributes);
                klass.def("first_child", yxml::yxml_element_first_child);
                klass.def("get", yxml::yxml_element_get);
                klass.def("get_attribute", yxml::yxml_element_get_attribute);
                klass.def(
                    "insert_attribute",
                    yxml::yxml_element_insert_attribute,
                );
                klass.def("insert_element", yxml::yxml_element_insert_element);
                klass.def("insert_text", yxml::yxml_element_insert_text);
                klass.def("next_sibling", yxml::yxml_element_next_sibling);
                klass.def("parent", yxml::yxml_element_parent);
                klass.def("prev_sibling", yxml::yxml_element_prev_sibling);
                klass.def("push_elem_back", yxml::yxml_element_push_elem_back);
                klass
                    .def("push_elem_front", yxml::yxml_element_push_elem_front);
                klass.def("push_text_back", yxml::yxml_element_push_text_back);
                klass
                    .def("push_text_front", yxml::yxml_element_push_text_front);
                klass.def(
                    "remove_attribute",
                    yxml::yxml_element_remove_attribute,
                );
                klass.def("remove_range", yxml::yxml_element_remove_range);
                klass.def("size", yxml::yxml_element_size);
                klass.def("tag", yxml::yxml_element_tag);
                klass.def("to_s", yxml::yxml_element_to_string);
            });

        module.define_nested_class("XMLText", None).define(|klass| {
            klass.def("attributes", yxml::yxml_text_attributes);
            klass.def("format", yxml::yxml_text_format);
            klass.def("get_attribute", yxml::yxml_text_get_attribute);
            klass.def("insert", yxml::yxml_text_insert);
            klass.def("insert_attribute", yxml::yxml_text_insert_attribute);
            klass.def("insert_embed", yxml::yxml_text_insert_embed);
            klass.def(
                "insert_embed_with_attributes",
                yxml::yxml_text_insert_embed_with_attributes,
            );
            klass.def(
                "insert_with_attributes",
                yxml::yxml_text_insert_with_attributes,
            );
            klass.def("length", yxml::yxml_text_length);
            klass.def("next_sibling", yxml::yxml_text_next_sibling);
            klass.def("parent", yxml::yxml_text_parent);
            klass.def("prev_sibling", yxml::yxml_text_prev_sibling);
            klass.def("push", yxml::yxml_text_push);
            klass.def("remove_attribute", yxml::yxml_text_remove_attribute);
            klass.def("remove_range", yxml::yxml_text_remove_range);
            klass.def("to_s", yxml::yxml_text_to_string);
        });
    });
}
