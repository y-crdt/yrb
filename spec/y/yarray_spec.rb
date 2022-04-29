# frozen_string_literal: true

RSpec.describe Y::Array do
  context "when creating array type" do
    it "create array with name" do
      doc = Y::Doc.new
      transaction = doc.transact
      arr = transaction.get_array("my array")

      expect(arr.to_arr.size).to eq(0)
    end
  end

  context "when manipulating array" do
    it "returns size=1 when insert a single item" do
      doc = Y::Doc.new
      transaction = doc.transact
      arr = transaction.get_array("my array")
      arr.insert(transaction, 0, 1)

      expect(arr.length).to eq(1)
    end

    it "removes single element" do
      doc = Y::Doc.new
      transaction = doc.transact
      arr = transaction.get_array("my array")
      arr.insert(transaction, 0, 1)
      arr.insert(transaction, 1, 2)
      arr.remove(transaction, 0)

      expect(arr.length).to eq(1)
    end

    it "removes multiple elements" do
      doc = Y::Doc.new
      transaction = doc.transact
      arr = transaction.get_array("my array")
      arr.insert(transaction, 0, 1)
      arr.insert(transaction, 1, 2)
      arr.remove_range(transaction, 0, 2)

      expect(arr.length).to eq(0)
    end

    it "supports adding multiple types" do
      doc = Y::Doc.new
      transaction = doc.transact
      arr = transaction.get_array("my array")
      arr.insert(transaction, 0, 1)
      arr.insert(transaction, 1, "hello")
      arr.insert(transaction, 2, [1, 2, 3])

      expect(arr.to_arr).to eq([1, "hello", [1, 2, 3]])
    end
  end
end
