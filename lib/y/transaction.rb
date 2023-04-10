# frozen_string_literal: true

module Y
  class Transaction
    # @!attribute [r] document
    #
    # @return [Y::Doc] The document this array belongs to
    attr_accessor :document

    def initialize(doc = nil)
      @document = doc

      super()
    end

    # Applies the encoded update on this document. This will bring the
    # the document to the same state as the one the update is from.
    #
    # @param update [::Array<Integer>]
    # @return [void]
    def apply(update)
      ytransaction_apply_update(update)
    end

    # Applies the v2 encoded update on this document. This will bring the
    # the document to the same state as the one the update is from.
    #
    # @param update [::Array<Integer>]
    # @return [void]
    def apply_v2(update)
      ytransaction_apply_update_v2(update)
    end

    # Commits transaction
    #
    # @return [void]
    def commit
      ytransaction_commit
    end

    # Create or get array type
    #
    # @param name [String]
    # @return [Y::Array]
    def get_array(name)
      array = ytransaction_get_array(name)
      array.document = document
      array
    end

    # Create or get map type
    #
    # @param name [String]
    # @return [Y::Map]
    def get_map(name)
      map = ytransaction_get_map(name)
      map.document = document
      map
    end

    # Create or get text type
    #
    # @param name [String]
    # @return [Y::Text]
    def get_text(name)
      text = ytransaction_get_text(name)
      text.document = document
      text
    end

    # Create or get XMLElement type
    #
    # @param name [String]
    # @return [Y::XMLElement]
    def get_xml_element(name)
      xml_element = ytransaction_get_xml_element(name)
      xml_element.document = document
      xml_element
    end

    # Create or get XMLFragment type
    #
    # @param name [String]
    # @return [Y::XMLFragment]
    def get_xml_fragment(name)
      xml_fragment = ytransaction_get_xml_fragment(name)
      xml_fragment.document = document
      xml_fragment
    end

    # Create or get XMLText type
    #
    # @param name [String]
    # @return [Y::XMLText]
    def get_xml_text(name)
      xml_text = ytransaction_get_xml_text(name)
      xml_text.document = document
      xml_text
    end

    # Return a state vector for this transaction
    #
    # @return [::Array<Integer>]
    def state
      ytransaction_state_vector
    end

    # Returns a v2 state vector for this transaction
    #
    # @return [::Array<Integer>]
    def state_v2
      ytransaction_state_vector_v2
    end

    # @!method ytransaction_apply_update(update)
    #   Apply the encoded update within current transaction
    #
    # @param update [::Array<Integer>]
    # @return [void]
    # @!visibility private

    # @!method ytransaction_apply_update_v2(update)
    #   Apply the v2 encoded update within current transaction
    #
    # @param update [::Array<Integer>]
    # @return [void]
    # @!visibility private

    # @!method ytransaction_commit()
    #
    # @return [void]
    # @!visibility private

    # @!method ytransaction_get_array(name)
    #   Returns or creates an array by name
    #
    # @param name [String] Name of the array structure to retrieve or create
    # @return [Y::Array] Array structure
    # @!visibility private

    # @!method ytransaction_get_map(name)
    #   Returns or creates a map structure by name
    #
    # @param name [String] Name of the map structure to retrieve or create
    # @return [Y::Map] Map structure
    # @!visibility private

    # @!method ytransaction_get_text(name)
    #   Returns or creates a text structure by name
    #
    # @param name [String] Name of the text structure to retrieve or create
    # @return [Y::Text] Text structure
    # @!visibility private

    # @!method ytransaction_get_xml_element(name)
    #   Returns or creates a XML structure by name
    #
    # @param name [String] Name of the XML element structure to retrieve or
    #     create
    # @return [Y::XMLElement] XMLElement structure
    # @!visibility private

    # @!method ytransaction_get_xml_fragment(name)
    #   Returns or creates a XML fragment
    #
    # @param name [String] Name of the XML fragment to retrieve or
    #     create by
    # @return [Y::XMLFragment] XMLFragment structure
    # @!visibility private

    # @!method ytransaction_get_xml_text(name)
    #   Returns or creates a XML structure by name
    #
    # @param name [String] Name of the XML element structure to retrieve or
    #     create
    # @return [Y::XMLElement] XMLElement structure
    # @!visibility private

    # @!method ytransaction_state_vector
    #
    # @return [Array<Integer>]
    # @!visibility private

    # @!method ytransaction_state_vector_v2
    #
    # @return [Array<Integer>]
    # @!visibility private
  end
end
