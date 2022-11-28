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
    include Enumerable

    # @!attribute [r] document
    #
    # @return [Y::Doc] The document this map belongs to
    attr_accessor :document

    # Create a new map instance
    #
    # @param doc [Y::Doc]
    def initialize(doc = nil)
      @document = doc || Y::Doc.new

      super()
    end

    # Attach a listener to get notified about any changes to the map
    #
    # @param callback [Proc]
    # @param block [Block]
    # @return [Integer]
    def attach(callback, &block)
      return ymap_observe(callback) unless callback.nil?

      ymap_observe(block.to_proc) unless block.nil?
    end

    # Removes all map entries
    #
    # @return [Self]
    def clear
      document.current_transaction { |tx| ymap_clear(tx) }
      self
    end

    # rubocop:disable Layout/LineLength

    # Deletes the entry for the given key and returns its associated value.
    #
    # @example Deletes the entry and return associated value
    #
    #   m = doc.get_map("my map")
    #   m[:bar] = 1
    #   m.delete(:bar) # => 1
    #   m # => {}
    #
    # @example Unknown key is handled in block
    #
    #   m = doc.get_map("my map")
    #   m.delete(:nosuch) { |key| "Key #{key} not found" }# => "Key nosuch not found"
    #   m # => {}
    #
    # @param key [String|Symbol]
    # @return [void]
    def delete(key)
      value = document.current_transaction { |tx| ymap_remove(tx, key) }
      if block_given? && key?(key)
        yield key
      else
        value
      end
    end

    # rubocop:enable Layout/LineLength

    # Detach listener
    #
    # @param subscription_id [Integer]
    # @return [void]
    def detach(subscription_id)
      ymap_unobserve(subscription_id)
    end

    # @return [void]
    def each(&block)
      document.current_transaction { |tx| ymap_each(tx, block) }
    end

    # @return [true|false]
    def key?(key)
      document.current_transaction { |tx| ymap_contains(tx, key) }
    end

    alias has_key? key?

    # @return [Object]
    def [](key)
      document.current_transaction { |tx| ymap_get(tx, key) }
    end

    # @return [void]
    def []=(key, val)
      document.current_transaction { |tx| ymap_insert(tx, key, val) }
    end

    # Returns size of map
    #
    # @return [Integer]
    def size
      document.current_transaction { |tx| ymap_size(tx) }
    end

    # Returns a Hash representation of this map
    #
    # @return [Hash]
    def to_h
      document.current_transaction { |tx| ymap_to_h(tx) }
    end

    # Returns a JSON representation of map
    #
    # @return [String] JSON string
    def to_json(*_args)
      to_h.to_json
    end

    # @!method ymap_clear(tx)
    #   Removes all key-value pairs from Map
    #
    # @param tx [Y::Transaction]

    # @!method ymap_contains(tx, key)
    #   Check if a certain key is in the Map
    #
    # @param tx [Y::Transaction]
    # @param key [String|Symbol]
    # @return [Boolean] True, if and only if the key exists

    # @!method ymap_each(tx, proc)
    #   Iterates over all key-value pairs in Map by calling the provided proc
    #   with the key and the value as arguments.
    #
    # @param tx [Y::Transaction]
    # @param proc [Proc<String|Any>] A proc that is called for every element

    # @!method ymap_get(tx, key)
    #   Returns stored value for key or nil if none is present
    #
    # @param tx [Y::Transaction]
    # @param key [String|Symbol]
    # @return [Object|nil] Value or nil

    # @!method ymap_insert(tx, key, value)
    #   Insert value for key. In case the key already exists, the previous value
    #   will be overwritten.
    #
    # @param tx [Y::Transaction]
    # @param key [String|Symbol]
    # @param value [Object]

    # @!method ymap_observe(callback)
    #
    # @param callback [Proc]
    # @return [Integer]

    # @!method ymap_remove(tx, key)
    #   Removes key-value pair from Map if key exists.
    #
    # @param tx [Y::Transaction]
    # @param key [String|Symbol]

    # @!method ymap_size(tx)
    #   Returns number of key-value pairs stored in map
    #
    # @param tx [Y::Transaction]
    # @return [Integer] Number of key-value pairs

    # @!method ymap_to_h(tx)
    #   Returns a Hash representation of the Map
    #
    # @param tx [Y::Transaction]
    # @return [Hash] Hash representation of Map

    # @!method ymap_unobserve(subscription_id)
    #
    # @param subscription_id [Integer]
    # @return [void]
  end
end
