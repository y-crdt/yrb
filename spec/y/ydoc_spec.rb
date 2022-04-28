# frozen_string_literal: true

RSpec.describe Y::Doc do
  it "sync changes of a local document to a remote doc" do
    local_doc = Y::Doc.new
    local_transaction = local_doc.transact
    local_text = local_transaction.get_text("name")
    local_text.push(local_transaction, "hello ")

    remote_doc = Y::Doc.new
    remote_transaction = remote_doc.transact
    remote_text = remote_transaction.get_text("name")

    state_vector_remote = remote_doc.state_vector
    update_remote = local_doc.encode_diff_v1(state_vector_remote)

    remote_transaction.apply_update(update_remote)

    expect(remote_text.to_s).to eq(local_text.to_s)
  end
end
