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
  #   awareness.update.encode # [1,227,245,175,195,11,1,65,123, …]
  #
  #
  class Awareness
    # Applies an incoming update. This gets the local awareness instance in
    # sync with changes from another client. i.e., updates the state of another
    # user in the local awareness instance.
    #
    # @example Apply an incoming update
    #   update = [1,227,245,175,195,11,1,65,123, …]
    #
    #   awareness = Y::Awareness.new
    #   awareness.apply_update(update)
    #
    # @param [Array<Integer>] update A binary encoded update
    # @return [void]
    def apply_update(update)
      yawareness_apply_update(update)
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
    #   awareness.clients # {312134501=>"{\"editing\":{\"field\":\"description\",\"pos\":0},\"name\":\"Hannes Moser\"}"}
    #
    # @return [Hash] All clients and their current state
    def clients
      yawareness_clients
    end

    # Returns a JSON string state representation of a current Awareness
    # instance.
    #
    # @example Create local state and inspect it
    #   local_state = {
    #     editing: { field: "description", pos: 0 },
    #     name: "Hannes Moser"
    #   }.to_json
    #
    #   awareness = Y::Awareness.new
    #   awareness.local_state = local_state
    #   local_state # "{\"editing\":{\"field\":\"description\",\"pos\":0},\"name\":\"Hannes Moser\"}"
    #
    # @return [String] The current state of the local client
    def local_state
      yawareness_local_state
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
    # @return [void]
    def local_state=(json)
      yawareness_set_local_state(json)
    end

    # Subscribes to changes
    #
    # @return [Integer] The subscription ID
    def attach(callback, &block)
      return yawareness_on_update(callback) unless callback.nil?

      yawareness_on_update(block.to_proc) unless block.nil?
    end

    # Unsubscribe from changes
    #
    # @param [Integer] subscription_id
    # @return [void]
    def detach(subscription_id)
      yawareness_remove_on_update(subscription_id)
    end

    # Clears out a state of a given client, effectively marking it as
    # disconnected.
    #
    # @param [Integer] client_id Clears the state for given client_id
    # @return [void]
    def remove_state(client_id)
      yawareness_remove_state(client_id)
    end

    # Returns a serializable update object which is representation of a current
    # Awareness state.
    #
    # @return [String] Binary encoded update of this local awareness instance
    def update
      yawareness_update
    end

    # Returns a serializable update object which is representation of a current
    # Awareness state. Unlike Awareness::update, this method variant allows to
    # prepare update only for a subset of known clients. These clients must all
    # be known to a current Awareness instance, otherwise a
    # Error::ClientNotFound error will be returned.
    #
    # @return [String] Binary encoded update including all given client IDs
    def update_with_clients(clients)
      yawareness_update_with_clients(clients)
    end

    # rubocop:disable Lint/UselessAccessModifier
    private

    # @!method yawareness_apply_update(update)
    #   Applies an update
    #
    # @param [Y::AwarenessUpdate] A structure that represents an encodable state
    #   of an Awareness struct.

    # @!method yawareness_apply_update(update)
    #   Applies an update
    #
    # @param [Y::AwarenessUpdate] A structure that represents an encodable state
    #   of an Awareness struct.

    # @!method yawareness_clean_local_state
    #   Clears out a state of a current client , effectively marking it as
    #   disconnected.

    # @!method yawareness_client_id
    #   Returns a globally unique client ID of an underlying Doc.
    # @return [Integer] The Client ID

    # @!method yawareness_clients
    #   Returns a state map of all of the clients
    #   tracked by current Awareness instance. Those states are identified by
    #   their corresponding ClientIDs. The associated state is represented and
    #   replicated to other clients as a JSON string.
    #
    # @return [Hash<Integer, String>] Map of clients

    # @!method yawareness_local_state
    #
    # @return [String|nil] Returns a JSON string state representation of a
    #   current Awareness instance.

    # @!method yawareness_on_update(callback, &block)
    #
    # @param [Proc] A callback handler for updates
    # @return [Integer] The subscription ID

    # @!method yawareness_remove_on_update(subscription_id)
    #
    # @param [Integer] subscription_id The subscription id to remove

    # @!method yawareness_remove_state(client_id)
    #   Clears out a state of a given client, effectively marking it as
    #   disconnected.
    #
    # @param [Integer] A Client ID
    # @return [String|nil] Returns a JSON string state representation of a
    #   current Awareness instance.

    # @!method yawareness_set_local_state(state)
    #   Sets a current Awareness instance state to a corresponding JSON string.
    #   This state will be replicated to other clients as part of the
    #   AwarenessUpdate and it will trigger an event to be emitted if current
    #   instance was created using [Awareness::with_observer] method.
    #
    # @param [String] Returns a state map of all of the clients tracked by
    #   current Awareness instance. Those states are identified by their
    #   corresponding ClientIDs. The associated state is represented and
    #   replicated to other clients as a JSON string.

    # @!method yawareness_update
    #   Returns a serializable update object which is representation of a
    #   current Awareness state.
    #
    # @return [Y::AwarenessUpdate] The update object

    # @!method yawareness_update_with_clients(clients)
    #   Returns a serializable update object which is representation of a
    #   current Awareness state. Unlike [Y::Awareness#update], this method
    #   variant allows to prepare update only for a subset of known clients.
    #   These clients must all be known to a current Awareness instance,
    #   otherwise an error will be returned.
    #
    # @param [Array<Integer>]
    # @return [Y::AwarenessUpdate] The update object

    # rubocop:enable Lint/UselessAccessModifier
  end

  # rubocop:disable Lint/UselessAccessModifier
  class AwarenessEvent
    private

    # @!method added
    # @return [Array<Integer>] Added clients

    # @!method updated
    # @return [Array<Integer>] Updated clients

    # @!method removed
    # @return [Array<Integer>] Removed clients
  end
  # rubocop:enable Lint/UselessAccessModifier

  # rubocop:disable Lint/UselessAccessModifier
  class AwarenessUpdate
    def encode
      yawareness_update_encode
    end

    private

    # @!method yawareness_update_encode
    #   Encode the awareness state for simple transport
    #
    # @return [Array<Integer>] Encoded update
  end
  # rubocop:enable Lint/UselessAccessModifier
end
