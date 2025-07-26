# frozen_string_literal: true

RSpec.describe Y::XMLText do
  it "create XMLElement by name" do
    doc = Y::Doc.new
    xml_text = doc.get_xml_text("my text")

    expect(xml_text.attrs).to eq({})
  end

  it "appends a string to the end of the XMLText" do
    doc = Y::Doc.new
    xml_text = doc.get_xml_text("my text")

    xml_text << "Hello, World!"

    expect(xml_text.to_s).to eq("Hello, World!")
  end

  it "uses push as alias for <<" do
    doc = Y::Doc.new
    xml_text = doc.get_xml_text("my text")

    xml_text.push "Hello, World!"

    expect(xml_text.to_s).to eq("Hello, World!")
  end

  it "returns text attributes" do
    doc = Y::Doc.new
    xml_text = doc.get_xml_text("my text")

    xml_text.attr_name = "Hello"

    expect(xml_text.attrs).to eq({ "name" => "Hello" })
  end

  it "formats text" do
    doc = Y::Doc.new
    xml_text = doc.get_xml_text("my text")
    xml_text << "Hello, World!"

    attrs = { "format" => "bold" }
    xml_text.format(7, 6, attrs)

    expect(xml_text.to_s).to eq("Hello, <format>World!</format>")
  end

  it "get a list of changes" do
    doc = Y::Doc.new
    xml_text = doc.get_xml_text("my text")

    xml_text.insert(0, "Hello, World!", { format: "bold" })
    xml_text.insert(13, " From Hannes.", { format: "italic" })

    expect(xml_text.diff.map(&:to_h)).to eq([
                                              { insert: "Hello, World!",
                                                attrs: { "format" => "bold" } },
                                              { insert: " From Hannes.",
                                                attrs: { "format" => "italic" } }
                                            ])
  end

  it "inserts string at position" do
    doc = Y::Doc.new
    xml_text = doc.get_xml_text("my xml text")
    xml_text.insert(0, "Hello, World!")

    expect(xml_text.to_s).to eq("Hello, World!")
  end

  it "inserts string with attributes at position" do
    doc = Y::Doc.new
    xml_text = doc.get_xml_text("my xml text")

    attrs = { format: "bold" }
    xml_text.insert(0, "Hello, World!", attrs)

    expect(xml_text.to_s).to eq("<format>Hello, World!</format>")
  end

  it "inserts Boolean at position" do
    doc = Y::Doc.new
    xml_text = doc.get_xml_text("my xml text")

    xml_text.insert(0, true)

    expect(xml_text.to_s).to eq("true")
  end

  it "inserts Integer at position" do
    doc = Y::Doc.new
    xml_text = doc.get_xml_text("my xml text")

    xml_text.insert(0, 42)

    expect(xml_text.to_s).to eq("42")
  end

  it "inserts Float at position" do
    doc = Y::Doc.new
    xml_text = doc.get_xml_text("my xml text")

    xml_text.insert(0, 1.2)

    expect(xml_text.to_s).to eq("1.2")
  end

  it "inserts Array at position" do
    doc = Y::Doc.new
    xml_text = doc.get_xml_text("my xml text")

    xml_text.insert(0, [1, 2, 3])

    expect(xml_text.to_s).to eq("[1, 2, 3]")
  end

  it "inserts Hash at position" do
    doc = Y::Doc.new
    xml_text = doc.get_xml_text("my xml text")

    xml_text.insert(0, { hello: "World" })

    expect(xml_text.to_s).to eq("{hello: World}")
  end

  it "inserts embed at position with attributes" do
    doc = Y::Doc.new
    xml_text = doc.get_xml_text("my xml text")

    attrs = { format: "bold" }
    xml_text.insert(0, { hello: "World" }, attrs)

    expect(xml_text.to_s).to eq("<format>{hello: World}</format>")
  end

  it "returns length of text" do
    doc = Y::Doc.new
    xml_text = doc.get_xml_text("my xml text")

    xml_text << "Hello, World!"

    expect(xml_text.length).to eq(13)
  end

  it "uses size as alias of length" do
    doc = Y::Doc.new
    xml_text = doc.get_xml_text("my xml text")

    xml_text << "Hello, World!"

    expect(xml_text.size).to eq(13)
  end

  it "retrieves next node (text or element)" do
    doc = Y::Doc.new
    root = doc.get_xml_element("my xml element")
    a = root.push_text
    b = root << "b"

    expect(a.next_sibling.tag).to eq(b.tag)
  end

  it "retrieves parent node" do
    doc = Y::Doc.new
    root = doc.get_xml_element("my xml element")
    a = root.push_text

    expect(a.parent.tag).to eq(root.tag)
  end

  it "retrieves prev node (text or element)" do
    doc = Y::Doc.new
    root = doc.get_xml_element("my xml element")
    a = root << "a"
    b = root.push_text

    expect(b.prev_sibling.tag).to eq(a.tag)
  end

  it "removes string from text at position" do
    doc = Y::Doc.new
    xml_text = doc.get_xml_text("my xml text")
    xml_text << "Hello, World!"

    xml_text.slice!(0)

    expect(xml_text.to_s).to eq("ello, World!")
  end

  it "removes string from text staring at position and given length" do
    doc = Y::Doc.new
    xml_text = doc.get_xml_text("my xml text")
    xml_text << "Hello, World!"

    xml_text.slice!(0, 3)

    expect(xml_text.to_s).to eq("lo, World!")
  end

  it "returns string representation of text" do
    doc = Y::Doc.new
    xml_text = doc.get_xml_text("my xml text")
    xml_text << "Hello, World!"

    expect(xml_text.to_s).to eq("Hello, World!")
  end

  context "when traversing elements" do
    let(:local) { Y::Doc.new }
    let(:remote) do
      doc = Y::Doc.new
      doc.sync(local.diff)
      doc
    end

    before do
      local_xml = local.get_xml_element("my xml")
      local_xml << "A"
      local_xml.insert_text(1, "my text")
      local_xml << "B"
    end

    it "retrieves next sibling with document reference set" do
      remote_xml = remote.get_xml_element("my xml")
      first_text = remote_xml[1]
      next_sibling = first_text.next_sibling

      expect(next_sibling.document).to eq(remote)
    end

    it "retrieves previous sibling with document reference set" do
      remote_xml = remote.get_xml_element("my xml")
      first_text = remote_xml[1]
      prev_sibling = first_text.prev_sibling

      expect(prev_sibling.document).to eq(remote)
    end
  end

  context "when syncing documents" do
    it "updates remote XMLText from local XMLText" do
      local = Y::Doc.new
      local_xml_text = local.get_xml_text("my xml text")
      local_xml_text << "hello"

      remote = Y::Doc.new
      remote_xml_text = remote.get_xml_text("my xml text")

      update = local.diff(remote.state)
      remote.sync(update)

      expect(remote_xml_text.to_s).to eq("hello")
    end
  end
end
