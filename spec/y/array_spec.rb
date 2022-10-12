# frozen_string_literal: true

RSpec.describe Y::Array do
  it "creates a new array" do
    doc = Y::Doc.new
    arr = doc.get_array("my array")

    expect(arr).to be_empty
  end

  it "adds element to the end" do
    doc = Y::Doc.new
    arr = doc.get_array("my array")

    arr << 1

    expect(arr.to_a).to eq([1])
  end

  it "adds element to the end with push alias" do
    doc = Y::Doc.new
    arr = doc.get_array("my array")

    arr.push(1)

    expect(arr.to_a).to eq([1])
  end

  it "appends all elements from array" do
    doc = Y::Doc.new
    arr = doc.get_array("my array")

    arr.concat([1, 2, 3])

    expect(arr.to_a).to eq([1, 2, 3])
  end

  it "insert different types" do
    doc = Y::Doc.new
    arr = doc.get_array("my array")

    arr << 42
    arr << 1.2
    arr << true
    arr << false
    arr << [1, 2, 3]
    arr << { format: "bold" }

    expect(arr.to_a).to eq([42, 1.2, true, false, [1, 2, 3],
                            { "format" => "bold" }])
  end

  it "retrieves element at position" do
    doc = Y::Doc.new
    arr = doc.get_array("my array")

    arr << 1

    expect(arr[0]).to eq(1)
  end

  it "removes element at position" do
    doc = Y::Doc.new
    arr = doc.get_array("my array")

    arr << 1
    arr.slice!(0)

    expect(arr).to be_empty
  end

  it "removes multiple elements" do
    doc = Y::Doc.new
    arr = doc.get_array("my array")

    arr << 1
    arr << 2
    arr.slice!(0..1)

    expect(arr).to be_empty
  end

  it "removes multiple elements" do
    doc = Y::Doc.new
    arr = doc.get_array("my array")

    arr << 1
    arr << 2
    arr.slice!(0...1)

    expect(arr.to_a).to eq([2])
  end

  it "removes number of elements from position" do
    doc = Y::Doc.new
    arr = doc.get_array("my array")

    arr << 1
    arr << 2
    arr.slice!(0, 2)

    expect(arr).to be_empty
  end

  it "returns first element" do
    doc = Y::Doc.new
    arr = doc.get_array("my array")

    arr << 1
    arr << 2
    arr << 3

    expect(arr.first).to eq(1)
  end

  it "returns last element" do
    doc = Y::Doc.new
    arr = doc.get_array("my array")

    arr << 1
    arr << 2
    arr << 3

    expect(arr.last).to eq(3)
  end

  it "adds element at the beginning" do
    doc = Y::Doc.new
    arr = doc.get_array("my array")

    arr << 1
    arr.unshift(2)

    expect(arr.first).to eq(2)
  end

  it "uses prepend as an alias for unshift" do
    doc = Y::Doc.new
    arr = doc.get_array("my array")

    arr << 1
    arr.prepend(2)

    expect(arr.first).to eq(2)
  end

  it "removes element from the end" do
    doc = Y::Doc.new
    arr = doc.get_array("my array")

    arr << 1
    arr.pop

    expect(arr).to be_empty
  end

  it "removes multiple elements from the end" do
    doc = Y::Doc.new
    arr = doc.get_array("my array")

    arr << 1
    arr << 2
    arr.pop(2)

    expect(arr).to be_empty
  end

  it "returns size of array" do
    doc = Y::Doc.new
    arr = doc.get_array("my array")

    arr << 1

    expect(arr.size).to eq(1)
  end

  it "uses length as an alias of size" do
    doc = Y::Doc.new
    arr = doc.get_array("my array")

    arr << 1

    expect(arr.length).to eq(1)
  end

  it "iterates over all elements" do
    doc = Y::Doc.new
    arr = doc.get_array("my array")

    arr << 1
    arr << 2

    arr.each do |element|
      expect(element).to eq(1)
    end
  end

  context "when syncing documents" do
    it "updates remote array from local array" do
      local = Y::Doc.new

      local_arr = local.get_array("my array")
      local_arr << "world"

      remote = Y::Doc.new
      diff = local.diff(remote.state)
      remote.sync(diff)

      remote_arr = remote.get_array("my array")

      expect(remote_arr.to_a).to match_array(["world"])
    end
  end

  context "when changing" do
    it "invokes callback" do
      local = Y::Doc.new
      arr = local.get_array("my array")

      called = nil
      listener = proc { |changes| called = changes }

      subscription_id = arr.attach(listener)

      arr << 1
      arr << 2
      arr.slice!(1)
      arr << 3

      local.commit
      arr.detach(subscription_id)

      expect(called).to eq(
        [
          { added: [1, 3] }
        ]
      )
    end

    it "commits automatically" do
      local = Y::Doc.new

      changes = []

      arr = local.get_array("my array")
      arr.attach(->(delta) { changes << delta })

      local.transact do
        arr << 1
        arr << 2
        arr << 3
      end

      local.transact do
        arr.slice!(1)
      end

      local.transact do
        arr[1] = 2
      end

      expect(arr.to_a).to match_array([1, 2, 3])
      expect(changes).to match_array(
        [
          [{ added: [1, 2, 3] }],
          [
            { retain: 1 },
            { removed: 1 }
          ],
          [
            { retain: 1 },
            { added: [2] }
          ]
        ]
      )
    end
  end
end
