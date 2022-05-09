# frozen_string_literal: true

module Y
  # rubocop:disable Lint/EmptyClass
  class Transaction
    # @!method get_array(name)
    #   Returns or creates an array by name
    #
    # @param [String] name Name of the array structure to retrieve or create
    # @return [Y::Array] Array structure

    # @!method get_map(name)
    #   Returns or creates a map structure by name
    #
    # @param [String] name Name of the map structure to retrieve or create
    # @return [Y::Map] Map structure

    # @!method get_text(name)
    #   Returns or creates a text structure by name
    #
    # @param [String] name Name of the text structure to retrieve or create
    # @return [Y::Text] Text structure

    # @!method get_xml_element(name)
    #   Returns or creates a XML structure by name
    #
    # @param [String] name Name of the XML element structure to retrieve or
    #     create
    # @return [Y::XMLElement] XMLElement structure
  end
  # rubocop:enable Lint/EmptyClass
end
