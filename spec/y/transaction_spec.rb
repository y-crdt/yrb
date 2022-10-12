# frozen_string_literal: true

RSpec.describe Y::Transaction do
  it "applies update" do
    local = Y::Doc.new
    local_txn = local.current_transaction

    remote = Y::Doc.new
    remote_text = remote.get_text("my text")
    remote_text << "some text"

    update = remote.diff(local_txn.state)
    local_txn.apply(update)

    local_text = local.get_text("my text")

    expect(local_text.to_s).to eq("some text")
  end

  it "commits transaction" do
    remote = Y::Doc.new
    remote_text = remote.get_text("my text")
    remote_text << "some text"

    txn = remote.current_transaction

    expect { txn.commit }.not_to raise_error
  end

  it "creates array type" do
    doc = Y::Doc.new
    transaction = doc.current_transaction

    arr = transaction.get_array("my array")

    expect(arr).to be_instance_of(Y::Array)
  end

  it "creates map type" do
    doc = Y::Doc.new
    transaction = doc.current_transaction

    map = transaction.get_map("my map")

    expect(map).to be_instance_of(Y::Map)
  end

  it "creates text type" do
    doc = Y::Doc.new
    transaction = doc.current_transaction

    text = transaction.get_text("my text")

    expect(text).to be_instance_of(Y::Text)
  end

  it "creates XMLElement type" do
    doc = Y::Doc.new
    transaction = doc.current_transaction

    xml_element = transaction.get_xml_element("my xml")

    expect(xml_element).to be_instance_of(Y::XMLElement)
  end

  it "creates XMLText type" do
    doc = Y::Doc.new
    transaction = doc.current_transaction

    xml_text = transaction.get_xml_text("my xml text")

    expect(xml_text).to be_instance_of(Y::XMLText)
  end
end
