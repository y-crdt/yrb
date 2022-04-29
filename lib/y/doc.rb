# frozen_string_literal: true

require_relative "transaction"

module Y
  class Doc
    def begin_transaction(&block)
      tx = transact
      block.call tx
      nil
    end

    # @!method transact
    #   Creates a new transaction for the document
    #
    #   @example Create transaction on doc
    #     doc = Y::Doc.new
    #     tx = doc.transact
    #
    #   @return [Y::Transaction] The transaction object
  end
end
