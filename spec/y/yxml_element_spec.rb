# frozen_string_literal: true

RSpec.describe Y::XMLElement do
  it "creates new XMLElement" do
    doc = Y::Doc.new
    xml_element = doc.get_xml_element("my xml")

    expect(xml_element.to_s).to eq("<UNDEFINED></UNDEFINED>")
  end

  it "inserts new node at index" do
    doc = Y::Doc.new
    xml_element = doc.get_xml_element("my xml")
    xml_element[0] = "root"

    expect(xml_element.to_s).to eq("<UNDEFINED><root></root></UNDEFINED>")
  end

  it "retrieve node from index" do
    doc = Y::Doc.new
    xml_element = doc.get_xml_element("my xml")
    xml_element[0] = "root"

    expect(xml_element[0].to_s).to eq("<root></root>")
  end

  it "retrieve attrs" do
    doc = Y::Doc.new
    xml_element = doc.get_xml_element("my xml")
    xml_element.attr_name = "Hello"

    expect(xml_element.attrs).to eq({ "name" => "Hello" })
  end

  it "attributes is an alias to attrs" do
    doc = Y::Doc.new
    xml_element = doc.get_xml_element("my xml")
    xml_element.attr_name = "Hello"

    expect(xml_element.attributes).to eq({ "name" => "Hello" })
  end

  it "retrieves first child from element" do
    doc = Y::Doc.new
    xml_element = doc.get_xml_element("my xml")
    xml_element[0] = "root"

    expect(xml_element.first_child.to_s).to eq("<root></root>")
  end

  it "inserts text into element at position" do
    doc = Y::Doc.new
    xml_element = doc.get_xml_element("my xml")
    xml_element.insert_text(0)

    expect(xml_element.to_s).to eq("<UNDEFINED></UNDEFINED>")
  end

  it "retrieves adjacent element or text (next)" do
    doc = Y::Doc.new
    xml_element = doc.get_xml_element("my xml")
    a = xml_element << "A"
    b = xml_element << "B"

    expect(a.next_sibling.tag).to eq(b.tag)
  end

  it "retrieves parent element" do
    doc = Y::Doc.new
    xml_element = doc.get_xml_element("my xml")
    a = xml_element << "A"

    expect(a.parent.tag).to eq(xml_element.tag)
  end

  it "retrieves adjacent element or text (previous)" do
    doc = Y::Doc.new
    xml_element = doc.get_xml_element("my xml")
    a = xml_element << "A"
    b = xml_element << "B"

    expect(b.prev_sibling.tag).to eq(a.tag)
  end

  it "pushes child to the end of the child list" do
    doc = Y::Doc.new
    xml_element = doc.get_xml_element("my xml")
    xml_element << "A"
    b = xml_element << "B"

    expect(xml_element[1].tag).to eq(b.tag)
  end

  it "push_child functions as an alias to <<" do
    doc = Y::Doc.new
    xml_element = doc.get_xml_element("my xml")
    xml_element.push_child "A"
    b = xml_element.push_child "B"

    expect(xml_element[1].tag).to eq(b.tag)
  end

  it "creates and inserts new text as last child of this element" do
    doc = Y::Doc.new
    xml_element = doc.get_xml_element("my xml")
    xml_element.push_text

    expect(xml_element.to_s).to eq("<UNDEFINED></UNDEFINED>")
  end

  it "returns size of child list" do
    doc = Y::Doc.new
    xml_element = doc.get_xml_element("my xml")
    xml_element << "A"
    xml_element << "B"

    expect(xml_element.size).to eq(2)
  end

  it "returns string representation of element" do
    doc = Y::Doc.new
    xml_element = doc.get_xml_element("my xml")
    a = xml_element << "A"
    a << "B"

    expect(xml_element.to_s).to eq("<UNDEFINED><A><B></B></A></UNDEFINED>")
  end

  it "adds child to the front of the child list" do
    doc = Y::Doc.new
    xml_element = doc.get_xml_element("my xml")
    xml_element.unshift_child "A"
    b = xml_element.unshift_child "B"

    expect(xml_element[0].tag).to eq(b.tag)
  end

  it "creates and inserts new text as first child of this element" do
    doc = Y::Doc.new
    xml_element = doc.get_xml_element("my xml")
    xml_element.unshift_text

    expect(xml_element.to_s).to eq("<UNDEFINED></UNDEFINED>")
  end

  context "when syncing documents" do
    it "updates remote xml from local xml" do
      local = Y::Doc.new
      local_xml = local.get_xml_element("my xml")
      a = local_xml << "A"
      a << "B"

      remote = Y::Doc.new
      remote_xml = remote.get_xml_element("my xml")

      update = local.diff(remote.state)
      remote.sync(update)

      expect(remote_xml.to_s).to eq("<UNDEFINED><A><B></B></A></UNDEFINED>")
    end
  end

  context "when changing" do
    it "invokes callback" do
      local = Y::Doc.new
      xml_element = local.get_xml_element("my xml element")

      called = nil
      listener = proc { |changes| called = changes }

      subscription_id = xml_element.attach(listener)

      xml_element << "A"
      xml_element << "B"

      local.commit
      xml_element.detach(subscription_id)

      expect(called.first[:added].size).to eq(2)
      expect(called.first[:added].first.tag).to eq("A")
      expect(called.first[:added].last.tag).to eq("B")
    end

    it "supports block as callback argument" do
      local = Y::Doc.new
      xml_element = local.get_xml_element("my xml element")

      called = nil

      subscription_id = xml_element.attach do |changes|
        called = changes
      end

      xml_element << "A"
      xml_element << "B"

      local.commit
      xml_element.detach(subscription_id)

      expect(called.first[:added].size).to eq(2)
      expect(called.first[:added].first.tag).to eq("A")
      expect(called.first[:added].last.tag).to eq("B")
    end

    it "commits automatically" do
      local = Y::Doc.new

      changes = []

      xml_element = local.get_xml_element("my xml element")
      xml_element.attach(proc { |delta| changes << delta })

      local.transact do
        xml_element << "A"
        xml_element << "B"
        xml_element << "C"
      end

      local.transact do
        xml_element.slice!(1)
      end

      local.transact do
        xml_element[1] = "B"
      end

      expect(xml_element.to_s).to eq(
        "<UNDEFINED><A></A><B></B><C></C></UNDEFINED>"
      )
      expect(changes.size).to eq(3)

      expect(changes[0].size).to eq(1)
      expect(changes[0].first).to have_key(:added)
      expect(changes[0].first[:added].map(&:tag)).to match_array(%w[A B C])

      expect(changes[1].size).to eq(2)
      expect(changes[1].first).to eq({ retain: 1 })
      expect(changes[1].last).to eq({ removed: 1 })

      expect(changes[2].size).to eq(2)
      expect(changes[2].first).to eq({ retain: 1 })
      expect(changes[2].last).to have_key(:added)
      expect(changes[2].last[:added].first.tag).to eq("B")
    end
  end
end
