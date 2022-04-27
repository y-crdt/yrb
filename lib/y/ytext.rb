# frozen_string_literal: true

module Y
  class YText
    def initialize(transaction, str)
      @pointer = Y::FFI.ytext(transaction, str)
      @offset = 0
    end

    def push(transaction, str)
      Y::FFI.ytext_insert(@pointer, transaction, @offset, str)
      @offset += str.length
    end

    def to_s
      Y::FFI.ytext_string(@pointer)
    end
  end
end
