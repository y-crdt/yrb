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
            klass.def("insert", ymap::ymap_insert);
            klass.def("clear", ymap::ymap_clear);
            klass.def("contains", ymap::ymap_contains);
            klass.def("get", ymap::ymap_get);
            klass.def("remove", ymap::ymap_remove);
            klass.def("size", ymap::ymap_size);
            klass.def("to_h", ymap::ymap_to_hash);
        });

        module.define_nested_class("Text", None).define(|klass| {
            klass.def("insert", ytext::ytext_insert);
            klass.def("insert_embed", ytext::ytext_insert_embed);
            klass.def(
                "insert_embed_with_attrs",
                ytext::ytext_insert_embed_with_attributes,
            );
            klass.def("insert_with_attrs", ytext::ytext_insert_with_attributes);
            klass.def("remove_range", ytext::ytext_remove_range);
            klass.def("format", ytext::ytext_format);
            klass.def("length", ytext::ytext_length);
            klass.def("push", ytext::ytext_push);
            klass.def("changes", ytext::ytext_changes);
            klass.def("to_s", ytext::ytext_to_string);
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
            });

        module
            .define_nested_class("XMLElement", None)
            .define(|klass| {
                klass.def("attributes", yxml::yxml_element_attributes);
                klass.def("get", yxml::yxml_element_get);
                klass.def("get_attribute", yxml::yxml_element_get_attribute);
                klass.def(
                    "insert_attribute",
                    yxml::yxml_element_insert_attribute,
                );
                klass.def("insert_element", yxml::yxml_element_insert_element);
                klass.def("insert_text", yxml::yxml_element_insert_text);
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
            });
    });
}
