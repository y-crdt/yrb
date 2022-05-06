# frozen_string_literal: true

RSpec.describe Y::Map do
  context "when using public API" do
    it "create map with name" do
      skip "this is just to test the public API"
      doc = Y::Doc.new
      map = doc.get_map("my map")

      map[:hello] = "world"
      expect(map[:hello]).to eq("world")

      map.each do |key, val|
        expect(key).to eq("hello")
        expect(val).to eq("world")
      end

      map.delete(:hello)
      expect(map.size).to eq(0)

      map.delete(:hello) { |key| expect(key).to eq(:hello) }

      map[:hello] = "world"
      m = map.clear
      expect(m).to eq(map)
      expect(m.size).to eq(0)

      d2 = Y::Doc.new
      diff = doc.diff(d2.state)
      d2.sync(diff)

      m = d2.get_map("my map")
      pp m.to_h
    end
  end

  context "when creating map type" do
    it "create map with name" do
      doc = Y::Doc.new
      transaction = doc.transact
      map = transaction.get_map("name")

      expect(map.to_h).to eq({})
    end
  end

  context "when inserting into map" do
    it "adds key-value pair to map" do
      doc = Y::Doc.new
      transaction = doc.transact
      map = transaction.get_map("name")

      map.insert(transaction, :hello, "world")

      expect(map.to_h).to eq({ hello: "world" })
    end

    it "supports string and symbol keys" do
      doc = Y::Doc.new
      transaction = doc.transact
      map = transaction.get_map("name")

      map.insert(transaction, :hello1, "world")
      map.insert(transaction, "hello2", "world")

      expect(map.to_h).to eq({ hello1: "world", hello2: "world" })
    end
  end

  context "when retrieving entries" do
    it "returns value by key" do
      doc = Y::Doc.new
      transaction = doc.transact
      map = transaction.get_map("name")

      map.insert(transaction, :hello, "world")
      value = map.get(:hello)

      expect(value).to eq("world")
    end
  end

  context "when manipulating entries" do
    it "clears map" do
      doc = Y::Doc.new
      map = doc.get_map("name")

      map[:hello] = "world"
      map.clear

      expect(map.size).to eq(0)
    end
  end

  context "when introspecting map properties" do
    it "returns JSON representation of map" do
      doc = Y::Doc.new
      transaction = doc.transact
      map = transaction.get_map("name")

      map.insert(transaction, :hello, "world")

      expect(map.to_json).to eq("{\"hello\":\"world\"}")
    end

    it "returns size of map" do
      doc = Y::Doc.new
      transaction = doc.transact
      map = transaction.get_map("name")

      map.insert(transaction, :hello, "world")

      expect(map.size).to eq(1)
    end

    it "returns true for existing key" do
      doc = Y::Doc.new
      transaction = doc.transact
      map = transaction.get_map("name")

      map.insert(transaction, :hello, "world")

      expect(map.contains(:hello)).to eq(true)
    end

    it "returns false for non-existing key" do
      doc = Y::Doc.new
      transaction = doc.transact
      map = transaction.get_map("name")

      expect(map.contains(:hello)).to eq(false)
    end
  end
end
