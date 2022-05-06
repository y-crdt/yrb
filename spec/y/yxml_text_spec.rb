# frozen_string_literal: true

RSpec.describe Y::XMLText do
  context "when creating XMLText type" do
    it "create XMLElement by name" do
      doc = Y::Doc.new
      transaction = doc.transact
      xml_text = transaction.get_xml_text("my text")

      expect(xml_text.length).to eq(0)
    end
  end

  context "when manipulating XMLText type" do
    it "inserts plain text" do
      doc = Y::Doc.new
      transaction = doc.transact
      xml_text = transaction.get_xml_text("my text")
      xml_text.insert(transaction, 0, "Hello, World!")

      expect(xml_text.to_s).to eq("Hello, World!")
    end

    it "inserts text with attributes" do
      doc = Y::Doc.new
      transaction = doc.transact
      xml_text = transaction.get_xml_text("my text")
      xml_text.insert_with_attributes(transaction, 0, "Hello",
                                      { format: "bold" })

      expect(xml_text.to_s).to eq("Hello")
    end

    it "inserts content" do
      doc = Y::Doc.new
      transaction = doc.transact
      xml_text = transaction.get_xml_text("my text")
      xml_text.insert_embed(transaction, 0, "Hello")

      expect(xml_text.to_s).to eq("")
    end

    it "inserts content with attributes" do
      doc = Y::Doc.new
      transaction = doc.transact
      xml_text = transaction.get_xml_text("my text")
      xml_text.insert_embed_with_attributes(transaction, 0, "Hello",
                                            { format: "bold" })

      expect(xml_text.to_s).to eq("")
    end

    it "pushes plain text to the end" do
      doc = Y::Doc.new
      transaction = doc.transact
      xml_text = transaction.get_xml_text("my text")
      xml_text.insert(transaction, 0, "Hello")
      xml_text.push(transaction, ", World!")

      expect(xml_text.to_s).to eq("Hello, World!")
    end

    it "formats" do
      doc = Y::Doc.new
      transaction = doc.transact
      xml_text = transaction.get_xml_text("my text")
      xml_text.insert(transaction, 0, "Hello")
      xml_text.format(transaction, 0, 5, { format: "bold" })

      expect(xml_text.to_s).to eq("Hello")
    end

    it "removes range" do
      doc = Y::Doc.new
      transaction = doc.transact
      xml_text = transaction.get_xml_text("my text")
      xml_text.insert(transaction, 0, "Hello, World!")
      xml_text.remove_range(transaction, 5, 8)

      expect(xml_text.to_s).to eq("Hello")
    end

    it "removes attribute" do
      doc = Y::Doc.new
      transaction = doc.transact
      xml_text = transaction.get_xml_text("my text")
      xml_text.insert(transaction, 0, "Hello, World!")
      xml_text.insert_attribute(transaction, "format", "bold")
      xml_text.remove_attribute(transaction, "format")

      expect(xml_text.get_attribute("format")).to be_nil
    end
  end

  context "when traversing XMLText type" do
    it "returns previous element" do
      doc = Y::Doc.new
      transaction = doc.transact
      root = transaction.get_xml_element("my element")
      first_child = root.insert_element(transaction, 0, "A")
      xml_text = root.insert_text(transaction, 1)

      expect(xml_text.prev_sibling.tag).to eq(first_child.tag)
    end

    it "returns next element" do
      doc = Y::Doc.new
      transaction = doc.transact
      root = transaction.get_xml_element("my element")
      first_child = root.insert_element(transaction, 0, "A")
      xml_text = root.insert_text(transaction, 0)

      expect(xml_text.next_sibling.tag).to eq(first_child.tag)
    end

    it "returns parent" do
      doc = Y::Doc.new
      transaction = doc.transact
      root = transaction.get_xml_element("my element")
      xml_text = root.insert_text(transaction, 0)

      expect(xml_text.parent.tag).to eq(root.tag)
    end

    it "returns attribute value" do
      doc = Y::Doc.new
      transaction = doc.transact
      xml_text = transaction.get_xml_text("my text")
      xml_text.insert(transaction, 0, "Hello")
      xml_text.insert_attribute(transaction, "format", "bold")

      expect(xml_text.get_attribute("format")).to eq("bold")
    end
  end

  context "when introspecting XMLText type" do
    it "returns string representation" do
      doc = Y::Doc.new
      transaction = doc.transact
      xml_text = transaction.get_xml_text("my text")

      expect(xml_text.to_s).to eq("")
    end
  end
end
