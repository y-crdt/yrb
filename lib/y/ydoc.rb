# frozen_string_literal: true

require "y/ffi"

module Y
  class YDoc
    attr_reader :pointer

    def initialize(_str = "")
      @pointer = Y::FFI.ydoc_new
    end

    def apply(transaction, update)
      Y::FFI.ytransaction_apply(transaction, update)
    end

    def get_text(name)
      with_transaction { |t| YText.new(t, name) }
    end

    def transact
      Y::FFI.ytransaction_new(pointer)
    end

    def state_vector(transaction)
      Y::FFI.ytransaction_state_vector_v1(transaction)
    end

    def encode_delta(transaction, state_vector)
      Y::FFI.ytransaction_state_diff_v1(transaction, state_vector)
    end

    def with_transaction
      raise "must provide block to begin transaction" unless block_given?

      transaction = Y::FFI.ytransaction_new(pointer)

      result = yield transaction

      Y::FFI.ytransaction_commit(transaction)

      result
    end
  end
end
