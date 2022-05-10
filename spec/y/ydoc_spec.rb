# frozen_string_literal: true

RSpec.describe Y::Doc do
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
