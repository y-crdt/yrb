# frozen_string_literal: true

RSpec.describe Y::Map do
  it "creates new map" do
    doc = Y::Doc.new
    map = doc.get_map("my map")

    expect(map).to be_instance_of(described_class)
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

    expect(map).to be_key(:hello)
  end

  it "returns false if key is missing" do
    doc = Y::Doc.new
    map = doc.get_map("my map")

    map[:hello] = "world"

    expect(map).not_to be_key(:nosuch)
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

    expect(map).not_to be_key(:hello)
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

    expected = { "hello" => "world" }

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

  context "when syncing documents" do
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

  context "when changing" do
    it "invokes callback" do
      local = Y::Doc.new
      map = local.get_map("my map")

      called = []
      listener = proc { |changes| called = changes }

      subscription_id = map.attach(listener)

      map[:hello] = "world"
      map[:say] = "goodbye"

      local.commit
      map.detach(subscription_id)

      expect(called).to match_array(
        [
          { inserted: { hello: "world" } },
          { inserted: { say: "goodbye" } }
        ]
      )
    end

    it "commits automatically" do
      local = Y::Doc.new

      changes = []

      map = local.get_map("my map")
      map.attach(->(delta) { changes << delta })

      local.transact do
        map[:hello] = "world"
        map[:say] = "goodbye"
      end

      local.transact do
        map.delete(:say)
      end

      local.transact do
        map[:say] = "hello again"
      end

      expect(map.to_h).to eq(
        {
          "hello" => "world",
          "say" => "hello again"
        }
      )

      expect(changes[0]).to match_array(
        [
          { inserted: { say: "goodbye" } },
          { inserted: { hello: "world" } }
        ]
      )
      expect(changes[1]).to match_array(
        [{ removed: { say: "goodbye" } }]
      )
      expect(changes[2]).to match_array(
        [{ inserted: { say: "hello again" } }]
      )
    end
  end
end
