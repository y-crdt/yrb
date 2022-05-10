# frozen_string_literal: true

RSpec.describe Y::Doc do
  it "returns a transaction" do
    doc = Y::Doc.new

    expect(doc.current_transaction).to be_instance_of(Y::Transaction)
  end

  it "returns diff" do
    local_doc = Y::Doc.new

    remote_doc = Y::Doc.new
    remote_text = remote_doc.get_text("my text")
    remote_text << "Hello, World!"

    expect(local_doc.diff(local_doc.state)).to_not be_empty
  end

  it "returns array" do
    doc = Y::Doc.new
    arr = doc.get_array("my array")

    expect(arr).to be_instance_of(Y::Array)
  end

  it "returns map" do
    doc = Y::Doc.new
    map = doc.get_map("my map")

    expect(map).to be_instance_of(Y::Map)
  end

  it "returns text" do
    doc = Y::Doc.new
    text = doc.get_text("my text")

    expect(text).to be_instance_of(Y::Text)
  end

  it "returns XMLElement" do
    doc = Y::Doc.new
    xml_element = doc.get_xml_element("my xml element")

    expect(xml_element).to be_instance_of(Y::XMLElement)
  end

  it "returns XMLText" do
    doc = Y::Doc.new
    xml_text = doc.get_xml_text("my xml text")

    expect(xml_text).to be_instance_of(Y::XMLText)
  end

  it "returns state" do
    doc = Y::Doc.new

    expect(doc.state).to match_array([0])
  end

  context "when syncing documents" do
    it "sync changes of a local document to a remote doc" do
      local_doc = Y::Doc.new
      local_text = local_doc.get_text("my text")
      local_text << "hello "

      remote_doc = Y::Doc.new
      remote_text = remote_doc.get_text("my text")

      remote_state = remote_doc.state
      update_remote = local_doc.diff(remote_state)

      remote_doc.sync(update_remote)

      expect(remote_text.to_s).to eq(local_text.to_s)
    end
  end
end
