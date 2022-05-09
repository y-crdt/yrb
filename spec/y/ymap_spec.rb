# frozen_string_literal: true

RSpec.describe Y::Map do
  it "creates new map" do
    doc = Y::Doc.new
    map = doc.get_map("my map")

    expect(map).to be_instance_of(Y::Map)
  end

  it "inserts element to map" do
    doc = Y::Doc.new
    map = doc.get_map("my map")

    map[:hello] = "world"

    expect(map).to have_key(:hello)
  end

  it "returns true if key exists" do
    doc = Y::Doc.new
    map = doc.get_map("my map")

    map[:hello] = "world"

    expect(map.key?(:hello)).to be_truthy
  end

  it "returns false if key is missing" do
    doc = Y::Doc.new
    map = doc.get_map("my map")

    map[:hello] = "world"

    expect(map.key?(:nosuch)).to be_falsey
  end

  it "retrieves value" do
    doc = Y::Doc.new
    map = doc.get_map("my map")

    map[:hello] = "world"

    expect(map[:hello]).to eq("world")
  end

  it "removes element" do
    doc = Y::Doc.new
    map = doc.get_map("my map")

    map[:hello] = "world"
    map.delete(:hello)

    expect(map.key?(:hello)).to be_falsey
  end

  it "provides non deleted key as argument in block" do
    doc = Y::Doc.new
    map = doc.get_map("my map")

    map.delete(:hello) { |key| expect(key).to eq(:hello) }
  end

  it "iterates over key-value pairs" do
    doc = Y::Doc.new
    map = doc.get_map("my map")

    map[:hello] = "world"
    map.each do |key, value|
      expect(key).to eq("hello")
      expect(value).to eq("world")
    end
  end

  it "clears map" do
    doc = Y::Doc.new
    map = doc.get_map("my map")

    map[:hello] = "world"
    map.clear

    expect(map.size).to eq(0)
  end

  it "returns hash" do
    doc = Y::Doc.new
    map = doc.get_map("my map")

    map[:hello] = "world"
    actual = map.to_h

    expected = { hello: "world" }

    expect(expected).to eq(actual)
  end

  it "returns size" do
    doc = Y::Doc.new
    map = doc.get_map("my map")

    map[:hello] = "world"
    expect(map.size).to eq(1)
  end

  it "returns the same structure for same name" do
    doc = Y::Doc.new
    map1 = doc.get_map("my map")
    map1[:hello] = "world"

    map2 = doc.get_map("my map")

    expect(map1.to_h).to eq(map2.to_h)
  end

  it "returns JSON representation of map" do
    doc = Y::Doc.new
    map = doc.get_map("my map")
    map[:hello] = "world"

    actual = map.to_json
    expected = "{\"hello\":\"world\"}"

    expect(expected).to eq(actual)
  end

  context "syncing documents" do
    it "updates remote map from local map" do
      local = Y::Doc.new

      local_map = local.get_map("my map")
      local_map[:hello] = "world"

      remote = Y::Doc.new
      diff = local.diff(remote.state)
      remote.sync(diff)

      remote_map = remote.get_map("my map")

      expect(remote_map).to have_key(:hello)
    end
  end
end
