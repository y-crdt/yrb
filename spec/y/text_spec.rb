# frozen_string_literal: true

RSpec.describe Y::Text do
  it "appends string to text" do
    doc = Y::Doc.new
    text = doc.get_text("my text")
    text << "Hello, World!"

    expect(text.size).to eq("Hello, World!".size)
  end

  it "formats text" do
    doc = Y::Doc.new
    text = doc.get_text("my text")
    text << "Hello, World!"

    attrs = { format: "bold" }
    text.format(0, 5, attrs)

    expect(text.to_s).to eq("Hello, World!")
  end

  it "get a list of changes" do
    doc = Y::Doc.new
    text = doc.get_text("my text")

    text.insert(0, "Hello, World!", { format: "bold" })
    text.insert(13, " From Hannes.", { format: "italic" })

    expect(text.diff.map(&:to_h)).to eq([
                                          { insert: "Hello, World!",
                                            attrs: { "format" => "bold" } },
                                          { insert: " From Hannes.",
                                            attrs: { "format" => "italic" } }
                                        ])
  end

  it "inserts string at position" do
    doc = Y::Doc.new
    text = doc.get_text("my text")
    text.insert(0, "Hello, World!")

    expect(text.to_s).to eq("Hello, World!")
  end

  it "inserts string with attributes at position" do
    doc = Y::Doc.new
    text = doc.get_text("my text")

    attrs = { format: "bold" }
    text.insert(0, "Hello, World!", attrs)

    expect(text.to_s).to eq("Hello, World!")
  end

  it "inserts Boolean at position" do
    doc = Y::Doc.new
    text = doc.get_text("my text")
    text.insert(0, "something")

    expect(text.to_s).to eq("something")
  end

  it "inserts Integer at position" do
    doc = Y::Doc.new
    text = doc.get_text("my text")
    text.insert(0, 42)

    expect(text.to_s).to eq("")
  end

  it "inserts Float at position" do
    doc = Y::Doc.new
    text = doc.get_text("my text")
    text.insert(0, 1.2)

    expect(text.to_s).to eq("")
  end

  it "inserts Array at position" do
    doc = Y::Doc.new
    text = doc.get_text("my text")
    text.insert(0, [1, 2, 3])

    expect(text.to_s).to eq("")
  end

  it "inserts Hash at position" do
    doc = Y::Doc.new
    text = doc.get_text("my text")
    text.insert(0, { hello: "World" })

    expect(text.to_s).to eq("")
  end

  it "inserts embed at position with attributes" do
    doc = Y::Doc.new
    text = doc.get_text("my text")

    attrs = { format: "bold" }
    text.insert(0, { hello: "World" }, attrs)

    expect(text.to_s).to eq("")
  end

  it "removes string from text at position" do
    doc = Y::Doc.new
    text = doc.get_text("my text")
    text << "Hello, World!"

    text.slice!(0)

    expect(text.to_s).to eq("ello, World!")
  end

  it "removes string from text staring at position and given length" do
    doc = Y::Doc.new
    text = doc.get_text("my text")
    text << "Hello, World!"

    text.slice!(0, 3)

    expect(text.to_s).to eq("lo, World!")
  end

  it "returns length of text" do
    doc = Y::Doc.new
    text = doc.get_text("my text")
    text << "123"

    expect(text.length).to eq(3)
  end

  it "returns size of text" do
    doc = Y::Doc.new
    text = doc.get_text("my text")
    text << "123"

    expect(text.size).to eq(3)
  end

  it "returns string representation of text" do
    doc = Y::Doc.new
    text = doc.get_text("my text")
    text << "Hello, World!"

    expect(text.to_s).to eq("Hello, World!")
  end

  context "when syncing documents" do
    it "updates remote text from local text" do
      local = Y::Doc.new

      local_text = local.get_text("my text")
      local_text << "Hello, World!"

      remote = Y::Doc.new
      diff = local.diff(remote.state)
      remote.sync(diff)

      remote_text = remote.get_text("my text")

      expect(remote_text.to_s).to eq("Hello, World!")
    end
  end

  # rubocop:disable RSpec/ExampleLength
  context "when changing" do
    it "invokes callback" do
      local = Y::Doc.new
      text = local.get_text("my text")

      called = nil
      listener = proc { |changes| called = changes }

      subscription_id = text.attach(listener)

      local.transact do
        text << "Hello, Wörld!"
        text.slice! 8, 1
        text.insert 8, "o"
      end

      text.detach(subscription_id)

      expect(called).to eq({ insert: "Hello, World!" })
    end

    # rubocop:disable RSpec/MultipleExpectations
    it "commits automatically" do
      local = Y::Doc.new

      changes = []

      text = local.get_text("my text")
      text.attach(proc { |delta| changes << delta })

      local.transact do
        text << "Hello, Wörld!"
      end

      local.transact do
        text.slice!(8)
      end

      local.transact do
        text.insert(8, "o")
      end

      expect(text.to_s).to eq("Hello, World!")
      expect(changes).to contain_exactly({ insert: "Hello, Wörld!" },
                                         { retain: 8 }, { delete: 1 }, { retain: 8 }, { insert: "o" })
    end
    # rubocop:enable RSpec/MultipleExpectations
  end
  # rubocop:enable RSpec/ExampleLength
end
