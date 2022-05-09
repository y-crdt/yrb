# frozen_string_literal: true

require "json"

module Y
  # A map can be used to store and retrieve key-value pairs.
  #
  # The map is the replicated counterpart to a Hash. It supports a subset
  # of the Hash operations, like adding, getting and deleting values by key.
  #
  # Someone should not instantiate a map directly, but use {Y::Doc#get_map}
  # instead.
  #
  # @example
  #   doc = Y::Doc.new
  #   map = doc.get_map("my map")
  #
  #   map[:hello] = "world"
  #   puts map[:hello]
  class Map
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
      value = ymap_remove(transaction, key)
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
      ymap_contains(key)
    end

    alias has_key? key?

    def [](key)
      ymap_get(key)
    end

    def []=(key, val)
      ymap_insert(transaction, key, val)
    end

    # Returns JSON representation of map
    # @return [String] JSON string
    def to_json(*_args)
      to_h.to_json
    end

    # @!method size()
    #   Returns number of key-value pairs stored in map
    #
    # @return [Integer] Number of key-value pairs

    # @!method to_h()
    #   Returns a Hash representation of the Map
    #
    # @return [Hash] Hash representation of Map

    private

    # @!attribute [r] document
    #
    # @return [Y::Doc] The document this map belongs to
    attr_accessor :document

    # @!method ymap_clear()
    #   Removes all key-value pairs from Map

    # @!method ymap_contains(key)
    #   Check if a certain key is in the Map
    #
    # @param [String|Symbol] key
    # @return [Boolean] True, if and only if the key exists

    # @!method ymap_each(proc)
    #   Iterates over all key-value pairs in Map by calling the provided proc
    #   with the key and the value as arguments.
    #
    # @param [Proc<String, Any>] proc A proc that is called for every element

    # @!method ymap_get(key)
    #   Returns stored value for key or nil if none is present
    #
    # @param [String|Symbol] key
    # @return [Any|Nil] Value or nil

    # @!method ymap_insert(transaction, key, value)
    #   Insert value for key. In case the key already exists, the previous value
    #   will be overwritten.
    #
    # @param [Y::Transaction] transaction
    # @param [String|Symbol] key
    # @param [Any] value

    # @!method ymap_remove(transaction, key)
    #   Removes key-value pair from Map if key exists.
    #
    # @param [Y::Transaction] transaction
    # @param [String|Symbol] key

    # A reference to the current active transaction of the document this map
    # belongs to.
    #
    # @return [Y::Transaction] A transaction object
    def transaction
      document.current_transaction
    end
  end
end
