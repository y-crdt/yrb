# frozen_string_literal: true

require_relative "transaction"

module Y
  # @example Create a local and remote doc and syncs the diff
  #   local = Y::Doc.new
  #   local_map = local.get_map("my map")
  #   local_map[:hello] = "world"
  #
  #   remote = Y::Doc.new
  #
  #   diff = local.diff(remote.state)
  #   remote.sync(diff)
  #
  #   remote_map = remote.get_map("my_map")
  #   pp remote_map.to_h #=> {hello: "world"}
  class Doc
    # The currently active transaction for this document
    # @return [Y::Transaction]
    def current_transaction
      @current_transaction ||= transact
    end

    # Create a diff between this document and another document. The diff is
    # created based on a state vector provided by the other document. It only
    # returns the missing blocks, as binary encoded sequence.
    #
    # @param [::Array<Int>] state The state to create the diff against
    # @return [::Array<Int>] Binary encoded diff
    def diff(state)
      encode_diff_v1(state)
    end

    def get_array(name)
      array = current_transaction.get_array(name)
      array.document = self
      array
    end

    # Gets or creates a new map by name
    #
    # If the optional input hash is present, fills the map up with key-value
    # pairs from the provided input hash. If the map already exists and isn't
    # empty, any existing keys are overridden and new keys are added.
    #
    # @param [String] name The name of the structure
    # @param [Hash] input Optional initial map key-value pairs
    # @return [Y::Map]
    def get_map(name, input = nil)
      map = current_transaction.get_map(name)
      map.document = self
      input&.each { |key, value| map[key] = value }
      map
    end

    # Gets or creates a new text by name
    #
    # If the optional input string is provided, fills a new text with the string
    # at creation time. If the text isn't new and not empty, appends the input
    # to the end of the text.
    #
    # @param [String] name The name of the structure
    # @param [String] input Optional initial text value
    # @return [Y::Text]
    def get_text(name, input = nil)
      text = current_transaction.get_text(name)
      text.document = self
      text.push(current_transaction, input) unless input.nil?
      text
    end

    # Creates a state vector of this document. This can be used to compare the
    # state of two documents with each other and to later on sync them.
    #
    # @return [::Array<Int>] Binary encoded state vector
    def state
      current_transaction.state_vector
    end

    # Synchronizes this document with the diff from another document
    #
    # @param [::Array<Int>] diff Binary encoded update
    def sync(diff)
      current_transaction.apply_update(diff)
    end

    # @!method transact
    #   Creates a new transaction for the document
    #
    #   @example Create transaction on doc
    #     doc = Y::Doc.new
    #     tx = doc.transact
    #
    # @return [Y::Transaction] The transaction object
  end
end
