# frozen_string_literal: true

RSpec.describe Y::YText do
  it "appends string to text type" do
    doc = Y::YDoc.new
    text = doc.get_text("my text")

    doc.with_transaction do |tx|
      text.push(tx, "foo")
    end

    pp text.inspect
    expect(doc).to_not be_nil
  end

  it "reads text to string" do
    doc = Y::YDoc.new
    text = doc.get_text("my text")
    doc.with_transaction { |t| text.push(t, "hello") }
    doc.with_transaction { |t| text.push(t, " world") }

    pp text.to_s
  end
end
