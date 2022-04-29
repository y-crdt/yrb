# frozen_string_literal: true

module Y
  class Doc
    def begin_transaction(&block)
      tx = transact
      block.call tx
      nil
    end
  end
end
