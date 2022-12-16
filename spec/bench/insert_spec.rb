# frozen_string_literal: true

RSpec.describe "Measure inserts", :bench do
  let!(:diff) do
    file = File.join(__dir__, "./input/b4-update.bin")
    File.read(file.to_s).unpack("C*")
  end

  let(:doc) { Y::Doc.new }
  let(:array) { doc.get_array("array") }

  it "insert element into array has a linear trend" do
    expect { |n, i| n.times { array << i } }.to perform_linear
  end
end
