# frozen_string_literal: true

RSpec.describe Y::Transaction do
  it "creates array type" do
    doc = Y::Doc.new
    doc.get_array("my array") # creates array

    arr = doc.current_transaction { |tx| tx.get_array("my array") }

    expect(arr).to be_instance_of(Y::Array)
  end

  it "creates map type" do
    doc = Y::Doc.new
    doc.get_map("my map") # creates map

    map = doc.current_transaction { |tx| tx.get_map("my map") }

    expect(map).to be_instance_of(Y::Map)
  end

  it "creates text type" do
    doc = Y::Doc.new
    doc.get_text("my text") # creates text

    text = doc.current_transaction { |tx| tx.get_text("my text") }

    expect(text).to be_instance_of(Y::Text)
  end

  it "creates XMLElement type" do
    doc = Y::Doc.new
    doc.get_xml_element("element") # creates text

    xml_element = doc.current_transaction { |tx| tx.get_xml_element("element") }

    expect(xml_element).to be_instance_of(Y::XMLElement)
  end

  it "creates XMLFragment type" do
    doc = Y::Doc.new
    doc.get_xml_fragment("fragment") # creates text

    xml_fragment = doc.current_transaction do |tx|
      tx.get_xml_fragment("fragment")
    end

    expect(xml_fragment).to be_instance_of(Y::XMLFragment)
  end

  it "creates XMLText type" do
    doc = Y::Doc.new
    doc.get_xml_text("text") # creates text

    xml_text = doc.current_transaction { |tx| tx.get_xml_text("text") }

    expect(xml_text).to be_instance_of(Y::XMLText)
  end
end
