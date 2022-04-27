# frozen_string_literal: true

RSpec.describe Y::YDoc do
  it "creates document" do
    doc = Y::YDoc.new

    expect(doc).to_not be_nil
  end

  it "adds a text type to document" do
    doc = Y::YDoc.new
    text = doc.get_text("my text")

    expect(text).to_not be_nil
  end

  it "syncs local document with diff from remote" do
    d1 = Y::YDoc.new
    t1 = d1.transact
    text = d1.get_text("name")
    text.push(t1, "hello ")

    d2 = Y::YDoc.new
    t2 = d2.transact
    remote = d2.get_text("name")
    remote.push(t2, "world!")

    state_vector = d2.state_vector(t2, remote.length)
    update = d1.encode_delta(t1, state_vector)

    d2.apply(t2, update)

    puts t2.to_s
  end
end
