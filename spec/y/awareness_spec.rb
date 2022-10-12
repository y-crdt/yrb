# frozen_string_literal: true

RSpec.describe Y::Awareness do
  let(:state) { { "cursor" => { "pos" => 0 } } }
  let(:local_awareness) { described_class.new }
  let(:local_id) { local_awareness.client_id }

  it "applies an update" do
    local_awareness.local_state = state.to_json

    remote_awareness = described_class.new
    remote_awareness.apply_update(local_awareness.update)

    remote_state = JSON.parse!(remote_awareness.clients[local_id])

    expect(remote_state).to eq(state)
  end

  it "cleans local state" do
    local_awareness.local_state = state.to_json
    local_awareness.clean_local_state

    expect(local_awareness.local_state).to be_nil
  end

  it "returns client_id" do
    local_awareness.local_state = state.to_json

    expect(local_id).to be_a(Numeric)
  end

  it "lists all clients" do
    local_awareness.local_state = state.to_json

    remote_awareness = described_class.new
    remote_awareness.local_state = state.to_json

    remote_awareness.apply_update(local_awareness.update)
    local_awareness.apply_update(remote_awareness.update)

    expect(local_awareness.clients.size).to eq(2)
  end

  it "receive local_state" do
    expect(local_awareness.local_state).to be_nil
  end

  it "removes state" do
    local_awareness.local_state = state.to_json
    local_awareness.remove_state(local_id)

    expect(local_awareness.local_state).to be_nil
  end

  it "sets local_state" do
    local_awareness.local_state = state.to_json

    expect(local_awareness.local_state).to eq(state.to_json)
  end

  it "receive update" do
    expect(local_awareness.update.encode).to eq([0])
  end

  it "state changes trigger an event" do
    event = nil

    local_awareness.local_state = state.to_json

    remote_awareness = described_class.new
    remote_awareness.attach(->(awareness_event) { event = awareness_event })
    remote_awareness.apply_update(local_awareness.update)

    expect(event).to be_instance_of(Y::AwarenessEvent)
  end
end
