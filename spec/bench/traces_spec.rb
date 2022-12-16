# frozen_string_literal: true

RSpec.describe "Measure real-world traces", :bench do
  let!(:diff) do
    file = File.join(__dir__, "./input/b4-update.bin")
    File.read(file.to_s).unpack("C*")
  end

  let(:doc) { Y::Doc.new }

  it "sync b4-update.bin (~400k) must be faster than 5ms" do
    expect { doc.sync(diff) }.to perform_under(5).ms.sample(100).times
  end
end
