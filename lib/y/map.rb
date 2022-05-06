# frozen_string_literal: true

require "json"

module Y
  class Map
    # @!attribute [r] document
    # @return [Y::Doc] The document this map belongs to
    attr_accessor :document

    # Create a new map instance
    #
    # @param [Y::Doc] doc
    def initialize(doc = nil)
      @document = doc || Y::Doc.new

      super()
    end

    # Removes all map entries
    #
    # @return [Self]
    def clear
      ymap_clear(transaction)
      self
    end

    # @!method contains(key)
    #   Check if a certain key is in the Map
    # @return [Boolean] True, if and only if the key exists

    # Deletes the entry for the given key and returns its associated value.
    #
    # @example If a block is given and key is found, ignores the block,
    #   deletes the entry, and returns the associated value:
    #
    #   m = doc.get_map("my map")
    #   m[:bar] = 1
    #   m.delete(:bar) # => 1
    #   m # => {}
    #
    # @example If a block is given and key is not found, calls the block and
    #   returns the block's return value:
    #
    #   m = doc.get_map("my map")
    #   m.delete(:nosuch) { |key| "Key #{key} not found" } # => "Key nosuch not found"
    #   m # => {}
    #
    # @param [String, Symbol] key
    # @return
    def delete(key)
      value = remove(transaction, key)
      if block_given? && key?(key)
        yield key
      else
        value
      end
    end

    def each(&block)
      ymap_each(block)
    end

    def key?(key)
      contains(key)
    end

    alias has_key? key?

    def [](key)
      get(key)
    end

    def []=(key, val)
      insert(transaction, key, val)
    end

    # Returns JSON representation of map
    # @return [String] JSON string
    def to_json(*_args)
      to_h.to_json
    end

    private

    # @!method ymap_clear()
    #   Removes all map entries

    # A reference to the current active transaction of the document this map
    # belongs to.
    #
    # @return [Y::Transaction] A transaction object
    def transaction
      document.current_transaction
    end
  end
end
