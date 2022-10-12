# frozen_string_literal: true

module Y
  class Awareness
    def apply_update(update)
      yawareness_apply_update(update)
    end

    def clean_local_state
      yawareness_clean_local_state
    end

    def client_id
      yawareness_client_id
    end

    def clients
      yawareness_clients
    end

    def local_state
      yawareness_local_state
    end

    def local_state=(json)
      yawareness_set_local_state(json)
    end

    def attach(callback, &block)
      return yawareness_on_update(callback) unless callback.nil?

      yawareness_on_update(block.to_proc) unless block.nil?
    end

    def detach(subscription_id)
      yawareness_remove_on_update(subscription_id)
    end

    def remove_state(client_id)
      yawareness_remove_state(client_id)
    end

    def update
      yawareness_update
    end

    def update_with_clients(clients)
      yawareness_update_with_clients(clients)
    end

    private

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
  end

  # rubocop:disable Lint/EmptyClass
  class AwarenessEvent
    private

    # @!method added
    # @return [Array<Integer>] Added clients

    # @!method updated
    # @return [Array<Integer>] Updated clients

    # @!method removed
    # @return [Array<Integer>] Removed clients
  end
  # rubocop:enable Lint/EmptyClass

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
end
