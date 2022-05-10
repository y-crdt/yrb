# frozen_string_literal: true

module Y
  class Transaction
    # Applies the binary encoded update for this document. This will bring the
    # the document to the same state as the one the update is from.
    #
    # @param [::Array<Integer>] update
    # @return [void]
    def apply(update)
      ytransaction_apply_update(update)
    end

    # Commits transaction
    #
    # @return [void]
    def commit
      ytransaction_commit
    end

    # Create or get array type
    #
    # @param [String] name
    # @return [Y::Array]
    def get_array(name)
      ytransaction_get_array(name)
    end

    # Create or get map type
    #
    # @param [String] name
    # @return [Y::Map]
    def get_map(name)
      ytransaction_get_map(name)
    end

    # Create or get text type
    #
    # @param [String] name
    # @return [Y::Text]
    def get_text(name)
      ytransaction_get_text(name)
    end

    # Create or get XMLElement type
    #
    # @param [String] name
    # @return [Y::XMLElement]
    def get_xml_element(name)
      ytransaction_get_xml_element(name)
    end

    # Create or get XMLText type
    #
    # @param [String] name
    # @return [Y::XMLRTet]
    def get_xml_text(name)
      ytransaction_get_xml_text(name)
    end

    # Return state vector for transaction
    #
    # @return [::Array<Integer>]
    def state
      ytransaction_state_vector
    end

    # @!method ytransaction_apply_update(update)
    #   Returns or creates an array by name
    #
    # @param [::Array<Integer>] update
    # @return [void]
    # @!visibility private

    # @!method ytransaction_commit()
    #
    # @return [void]
    # @!visibility private

    # @!method ytransaction_get_array(name)
    #   Returns or creates an array by name
    #
    # @param [String] name Name of the array structure to retrieve or create
    # @return [Y::Array] Array structure
    # @!visibility private

    # @!method ytransaction_get_map(name)
    #   Returns or creates a map structure by name
    #
    # @param [String] name Name of the map structure to retrieve or create
    # @return [Y::Map] Map structure
    # @!visibility private

    # @!method ytransaction_get_text(name)
    #   Returns or creates a text structure by name
    #
    # @param [String] name Name of the text structure to retrieve or create
    # @return [Y::Text] Text structure
    # @!visibility private

    # @!method ytransaction_get_xml_element(name)
    #   Returns or creates a XML structure by name
    #
    # @param [String] name Name of the XML element structure to retrieve or
    #     create
    # @return [Y::XMLElement] XMLElement structure
    # @!visibility private

    # @!method ytransaction_get_xml_text(name)
    #   Returns or creates a XML structure by name
    #
    # @param [String] name Name of the XML element structure to retrieve or
    #     create
    # @return [Y::XMLElement] XMLElement structure
    # @!visibility private

    # @!method ytransaction_state_vector
    #
    # @return [Array<Integer>]
    # @!visibility private
  end
end
