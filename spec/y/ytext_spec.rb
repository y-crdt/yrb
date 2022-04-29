# frozen_string_literal: true

RSpec.describe Y::Text do
  context "when creating text type" do
    it "create text with name" do
      doc = Y::Doc.new
      transaction = doc.transact
      text = transaction.get_text("name")
      text.push(transaction, "name")

      expect(text.to_s).to eq("name")
    end
  end

  context "when manipulating text" do
    it "pushes text to the end" do
      doc = Y::Doc.new
      transaction = doc.transact
      text = transaction.get_text("name")
      text.push(transaction, "hello")
      text.push(transaction, "world")

      expect(text.to_s).to eq("helloworld")
    end

    it "inserts text at position" do
      doc = Y::Doc.new
      transaction = doc.transact
      text = transaction.get_text("name")
      text.push(transaction, "abd")
      text.insert(transaction, 2, "c")

      expect(text.to_s).to eq("abcd")
    end

    it "inserts text with attributes" do
      doc = Y::Doc.new
      transaction = doc.transact
      text = transaction.get_text("name")

      attrs = { format: "bold" }
      text.insert_with_attrs(transaction, 2, "hello", attrs)

      expect(text.to_s).to eq("hello")
    end
  end
end
