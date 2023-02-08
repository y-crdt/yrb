# frozen_string_literal: true

module Y
  # The Awareness class implements a simple shared state protocol that can be
  # used for non-persistent data like awareness information (cursor, username,
  # status, ..). Each client can update its own local state and listen to state
  # changes of remote clients.
  #
  # Each client is identified by a unique client id (something we borrow from
  # doc.client_id). A client can override its own state by propagating a message
  # with an increasing timestamp (clock). If such a message is received, it is
  # applied if the known state of that client is older than the new state
  # (`clock < new_clock`). If a client thinks that a remote client is offline,
  # it may propagate a message with `{ clock, state: null, client }`. If such a
  # message is received, and the known clock of that client equals the received
  # clock, it will clean the state.
  #
  # Before a client disconnects, it should propagate a null state with an
  # updated clock.
  #
  # Awareness is an integral part of collaborative applications, you can read
  # more about the concept here: https://docs.yjs.dev/getting-started/adding-awareness
  #
  # @example Instantiate awareness instance and encode update for broadcast
  #   local_state = {
  #     editing: { field: "description", pos: 0 },
  #     name: "Hannes Moser"
  #   }.to_json
  #
  #   awareness = Y::Awareness.new
  #   awareness.local_state = local_state
  #   awareness.diff # [1,227,245,175,195,11,1,65,123, …]
  #
  # @example Two connected clients
  #   local_state_a = { name: "User A" }.to_json
  #
  #   client_a = Y::Awareness.new
  #   client_a.local_state = local_state
  #
  #   local_state_b = { name: "User B" }.to_json
  #
  #   client_b = Y::Awareness.new
  #   client_b.local_state = local_state_b
  #
  #   client_a.sync(client_b.diff)
  #   client_a.clients # {1242157267=>"{\"name\":\"User A\"}", 2401067547=>…
  class Awareness
    # Applies an incoming update. This gets the local awareness instance in
    # sync with changes from another client. i.e., updates the state of another
    # user in the local awareness instance.
    #
    # @example Apply an incoming update
    #   update = [1,227,245,175,195,11,1,65,123, …]
    #
    #   awareness = Y::Awareness.new
    #   awareness.sync(update)
    #
    # @param diff [Array<Integer>] A binary encoded update
    # @return [void]
    def sync(diff)
      yawareness_apply_update(diff)
    end

    # Clears out a state of a current client, effectively marking it as
    # disconnected.
    #
    # @return [void]
    def clean_local_state
      yawareness_clean_local_state
    end

    # Returns a globally unique client ID of an underlying Doc.
    #
    # @return [Integer] Returns the client_id of the local user
    def client_id
      yawareness_client_id
    end

    # Returns a state map of all of the clients tracked by current Awareness
    # instance. Those states are identified by their corresponding ClientIDs.
    # The associated state is represented and replicated to other clients as a
    # JSON string.
    #
    # @example Instantiate awareness instance and encode update for broadcast
    #   local_state = {
    #     editing: { field: "description", pos: 0 },
    #     name: "Hannes Moser"
    #   }.to_json
    #
    #   awareness = Y::Awareness.new
    #   awareness.local_state = local_state
    #   awareness.clients # {312134501=>"{\"editing\":{\"field\":\"descriptio …
    #
    # @return [Hash] All clients and their current state
    def clients
      transform = yawareness_clients.map do |client_id, state|
        [client_id, JSON.parse!(state)]
      end
      transform.to_h
    end

    # Returns the state of the local Awareness instance.
    #
    # @example Create local state and inspect it
    #   local_state = {
    #     editing: { field: "description", pos: 0 },
    #     name: "Hannes Moser"
    #   }
    #
    #   awareness = Y::Awareness.new
    #   awareness.local_state = local_state
    #   awareness.local_state # {  editing: { field: "description", ...
    #
    # @return [String] The current state of the local client
    def local_state
      json = yawareness_local_state
      JSON.parse!(json) if json
    end

    # Sets a current Awareness instance state to a corresponding JSON string.
    # This state will be replicated to other clients as part of the
    # AwarenessUpdate.
    #
    # @example Set local state
    #   local_state = {
    #     editing: { field: "description", pos: 0 },
    #     name: "Hannes Moser"
    #   }.to_json
    #
    #   awareness = Y::Awareness.new
    #   awareness.local_state = local_state
    #
    # @param [#to_json] state
    # @return [void]
    def local_state=(state)
      raise "state cannot be encoded to JSON" unless state.respond_to? :to_json

      yawareness_set_local_state(state.to_json)
    end

    # Subscribes to changes
    #
    # @return [Integer] The subscription ID
    def attach(callback = nil, &block)
      return yawareness_on_update(callback) unless callback.nil?

      yawareness_on_update(block.to_proc) unless block.nil?
    end

    # Clears out a state of a given client, effectively marking it as
    # disconnected.
    #
    # @param client_id [Integer] Clears the state for given client_id
    # @return [void]
    def remove_state(client_id)
      yawareness_remove_state(client_id)
    end

    # Returns a serializable update object which is representation of a current
    # Awareness state.
    #
    # @return [::Array<Integer>] Binary encoded update of the local instance
    def diff
      yawareness_update
    end

    # Returns a serializable update object which is representation of a current
    # Awareness state. Unlike Awareness::update, this method variant allows to
    # prepare update only for a subset of known clients. These clients must all
    # be known to a current Awareness instance, otherwise a
    # Error::ClientNotFound error will be returned.
    #
    # @param clients [::Array<Integer>] A list of client IDs
    # @return [String] Binary encoded update including all given client IDs
    def diff_with_clients(*clients)
      yawareness_update_with_clients(clients)
    end

    # rubocop:disable Lint/UselessAccessModifier
    private

    # @!method yawareness_apply_update(update)
    #   Applies an update
    #
    # @param A [Y::AwarenessUpdate] Structure that represents an encodable state
    #   of an Awareness struct.
    # @!visibility private

    # @!method yawareness_apply_update(update)
    #   Applies an update
    #
    # @param A [Y::AwarenessUpdate] Structure that represents an encodable state
    #   of an Awareness struct.
    # @!visibility private

    # @!method yawareness_clean_local_state
    #   Clears out a state of a current client , effectively marking it as
    #   disconnected.
    # @!visibility private

    # @!method yawareness_client_id
    #   Returns a globally unique client ID of an underlying Doc.
    # @return [Integer] The Client ID
    # @!visibility private

    # @!method yawareness_clients
    #   Returns a state map of all of the clients
    #   tracked by current Awareness instance. Those states are identified by
    #   their corresponding ClientIDs. The associated state is represented and
    #   replicated to other clients as a JSON string.
    #
    # @return [Hash<Integer, String>] Map of clients
    # @!visibility private

    # @!method yawareness_local_state
    #
    # @return [String|nil] Returns a JSON string state representation of a
    #   current Awareness instance.
    # @!visibility private

    # @!method yawareness_on_update(callback, &block)
    #
    # @param callback [callback]
    # @return [Integer] The subscription ID
    # @!visibility private

    # @!method yawareness_remove_on_update(subscription_id)
    #
    # @param subscription_id [Integer] The subscription id to remove
    # @!visibility private

    # @!method yawareness_remove_state(client_id)
    #   Clears out a state of a given client, effectively marking it as
    #   disconnected.
    #
    # @param client_id [Integer] A Client ID
    # @return [String|nil] Returns a JSON string state representation of a
    #   current Awareness instance.
    # @!visibility private

    # @!method yawareness_set_local_state(state)
    #   Sets a current Awareness instance state to a corresponding JSON string.
    #   This state will be replicated to other clients as part of the
    #   AwarenessUpdate and it will trigger an event to be emitted if current
    #   instance was created using [Awareness::with_observer] method.
    #
    # @param Returns [String] A state map of all of the clients tracked by
    #   current Awareness instance. Those states are identified by their
    #   corresponding ClientIDs. The associated state is represented and
    #   replicated to other clients as a JSON string.
    # @!visibility private

    # @!method yawareness_update
    #   Returns a serializable update object which is representation of a
    #   current Awareness state.
    #
    # @return [Y::AwarenessUpdate] The update object
    # @!visibility private

    # @!method yawareness_update_with_clients(clients)
    #   Returns a serializable update object which is representation of a
    #   current Awareness state. Unlike [Y::Awareness#update], this method
    #   variant allows to prepare update only for a subset of known clients.
    #   These clients must all be known to a current Awareness instance,
    #   otherwise an error will be returned.
    #
    # @param clients [::Array<Integer>]
    # @return [::Array<Integer>] A serialized (binary encoded) update object
    # @!visibility private

    # rubocop:enable Lint/UselessAccessModifier
  end

  # @!visibility private
  class AwarenessEvent
    private # rubocop:disable Lint/UselessAccessModifier

    # @!method added
    # @return [::Array<Integer>] Added clients
    # @!visibility private

    # @!method updated
    # @return [::Array<Integer>] Updated clients
    # @!visibility private

    # @!method removed
    # @return [::Array<Integer>] Removed clients
    # @!visibility private
  end
end
