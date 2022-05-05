# frozen_string_literal: true

RSpec.describe Y::XMLElement do
  context "when creating XMLElement type" do
    it "create XMLElement by name" do
      doc = Y::Doc.new
      transaction = doc.transact
      xml_element = transaction.get_xml_element("my xml")

      expect(xml_element.size).to eq(0)
    end

    it "adds nested XMLElement at the end" do
      doc = Y::Doc.new
      transaction = doc.transact
      xml_element = transaction.get_xml_element("my xml")
      new_element = xml_element.push_elem_back(transaction, "node")

      expect(new_element.tag).to eq("node")
    end

    it "adds nested XMLElement at the front" do
      doc = Y::Doc.new
      transaction = doc.transact
      xml_element = transaction.get_xml_element("my xml")
      new_element = xml_element.push_elem_front(transaction, "node")

      expect(new_element.tag).to eq("node")
    end

    it "adds XMLText at the end" do
      doc = Y::Doc.new
      transaction = doc.transact
      xml_element = transaction.get_xml_element("my xml")
      new_text = xml_element.push_text_back(transaction, "text")

      pp new_text
    end

    it "adds XMLText at the front" do
      doc = Y::Doc.new
      transaction = doc.transact
      xml_element = transaction.get_xml_element("my xml")
      new_text = xml_element.push_text_front(transaction, "text")

      pp new_text
    end

    it "creates nested XMLElement" do
      doc = Y::Doc.new
      transaction = doc.transact
      xml_element = transaction.get_xml_element("my xml")
      new_element = xml_element.insert_element(transaction, 0, "node")

      expect(new_element.tag).to eq("node")
    end

    it "creates nested XMLText" do
      doc = Y::Doc.new
      transaction = doc.transact
      xml_element = transaction.get_xml_element("my xml")
      new_text = xml_element.insert_text(transaction, 0, "mytext")

      pp new_text
    end
  end

  context "when manipulating XMLElement type" do
    it "inserts attribute" do
      doc = Y::Doc.new
      transaction = doc.transact
      xml_element = transaction.get_xml_element("my xml")
      xml_element.insert_attribute(transaction, "hello", "world")

      expect(xml_element.get_attribute("hello")).to_not be_nil
    end

    it "removes attribute" do
      doc = Y::Doc.new
      transaction = doc.transact
      xml_element = transaction.get_xml_element("my xml")
      xml_element.insert_attribute(transaction, "hello", "world")
      xml_element.remove_attribute(transaction, "hello")

      expect(xml_element.get_attribute("hello")).to be_nil
    end
  end

  context "when retrieving elements" do
    it "returns attributes" do
      doc = Y::Doc.new
      transaction = doc.transact
      xml_element = transaction.get_xml_element("my xml")
      xml_element.insert_attribute(transaction, "a", "1")
      xml_element.insert_attribute(transaction, "b", "2")

      attrs = xml_element.attributes(transaction, "hello", "world")

      expect(attrs).to match_array([%w[a 1], %w[b 2]])
    end

    it "returns attribute" do
      doc = Y::Doc.new
      transaction = doc.transact
      xml_element = transaction.get_xml_element("my xml")
      xml_element.insert_attribute(transaction, "hello", "world")
      attr = xml_element.get_attribute("hello")

      expect(attr).to eq("world")
    end

    it "returns attribute" do
      doc = Y::Doc.new
      transaction = doc.transact
      xml_element = transaction.get_xml_element("my xml")
      actual = xml_element.insert_element(transaction, 0, "A")

      expected = xml_element.get(0)

      expect(expected.tag).to eq(actual.tag)
    end
  end

  context "when introspecting XMLElement type" do
    it "returns tag name of current element" do
      doc = Y::Doc.new
      transaction = doc.transact
      xml_element = transaction.get_xml_element("my xml")

      expect(xml_element.tag).to eq("UNDEFINED")
    end
  end
end
