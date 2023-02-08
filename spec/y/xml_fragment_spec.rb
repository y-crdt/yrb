# frozen_string_literal: true

RSpec.describe Y::XMLFragment do
  it "creates new XMLFragment" do
    doc = Y::Doc.new
    xml_fragment = doc.get_xml_fragment("default")

    expect(xml_fragment.to_s).to eq("")
  end

  it "inserts new node at index" do
    doc = Y::Doc.new
    xml_fragment = doc.get_xml_fragment("default")
    xml_fragment[0] = "firstNode"

    expect(xml_fragment.to_s).to eq("<firstNode></firstNode>")
  end

  it "retrieve node from index" do
    doc = Y::Doc.new
    xml_fragment = doc.get_xml_fragment("default")
    xml_fragment[0] = "firstNode"

    expect(xml_fragment[0].to_s).to eq("<firstNode></firstNode>")
  end

  it "retrieves first child from element" do
    doc = Y::Doc.new
    xml_fragment = doc.get_xml_fragment("default")
    xml_fragment[0] = "root"

    expect(xml_fragment.first_child.to_s).to eq("<root></root>")
  end

  it "retrieves adjacent element or text (next)" do
    doc = Y::Doc.new
    xml_fragment = doc.get_xml_fragment("default")
    a = xml_fragment << "A"
    b = xml_fragment << "B"

    expect(a.next_sibling.tag).to eq(b.tag)
  end

  it "retrieves parent element" do
    doc = Y::Doc.new
    xml_fragment = doc.get_xml_fragment("default")
    a = xml_fragment << "A"

    expect(a.parent).to be_a(described_class)
  end

  it "retrieves adjacent element or text (previous)" do
    doc = Y::Doc.new
    xml_fragment = doc.get_xml_fragment("default")
    a = xml_fragment << "A"
    b = xml_fragment << "B"

    expect(b.prev_sibling.tag).to eq(a.tag)
  end

  it "pushes child to the end of the child list" do
    doc = Y::Doc.new
    xml_fragment = doc.get_xml_fragment("default")
    xml_fragment << "A"
    b = xml_fragment << "B"

    expect(xml_fragment[1].tag).to eq(b.tag)
  end

  it "push functions as an alias to <<" do
    doc = Y::Doc.new
    xml_fragment = doc.get_xml_fragment("default")
    xml_fragment.push "A"
    b = xml_fragment.push "B"

    expect(xml_fragment[1].tag).to eq(b.tag)
  end

  it "returns size of child list" do
    doc = Y::Doc.new
    xml_fragment = doc.get_xml_fragment("default")
    xml_fragment << "A"
    xml_fragment << "B"

    expect(xml_fragment.size).to eq(2)
  end

  it "returns string representation of element" do
    doc = Y::Doc.new
    xml_fragment = doc.get_xml_fragment("root")
    a = xml_fragment << "A"
    a << "B"

    expect(xml_fragment.to_s).to eq("<A><B></B></A>")
  end

  it "adds child to the front of the child list" do
    doc = Y::Doc.new
    xml_fragment = doc.get_xml_fragment("default")
    xml_fragment.unshift "A"
    b = xml_fragment.unshift "B"

    expect(xml_fragment[0].tag).to eq(b.tag)
  end
end
