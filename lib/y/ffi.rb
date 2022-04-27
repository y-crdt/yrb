# frozen_string_literal: true

require "ffi"

module Y
  module FFI
    extend ::FFI::Library

    LIB_NAME = "libyrs"
    private_constant :LIB_NAME

    # load pre-built library
    ffi_lib "lib/target/release/#{LIB_NAME}.#{::FFI::Platform::LIBSUFFIX}"

    # Creates a new doc. A doc can contain many shared types, like text, array,
    # map, etc.
    #
    # @example Create a new document
    #   doc = Y::CRDT.ydoc_new
    #
    # @return [::FFI::Pointer] A pointer to a YDoc instance
    attach_function :ydoc_new, [], :pointer

    # Destroy a doc and free the space that is allocated by this doc.
    #
    # @example Destroy a doc and free memory
    #   doc = Y::CRDT.ydoc_new
    #   Y::CRDT.ydoc_destroy(doc)
    #
    # @param pointer [::FFI::Pointer] A pointer to a Y::Doc instance
    # @return void
    attach_function :ydoc_destroy, %i[pointer], :void

    # Creates or retrieved a shared type: text pointer
    #
    # @param pointer [::FFI::Pointer] A pointer to a Y::Doc
    # @param name [String] Name of the shared type: text to retrieve, update,
    #   insert and delete later
    # @return [::FFI::Pointer] A pointer to a shared type: text
    attach_function :ytext, %i[pointer string], :pointer

    # Adds text at position to the shared type: text, requires a transaction to
    # do so. You can think of a transaction containing all the text manipulation
    # done in one go.
    #
    # @param pointer [::FFI::Pointer] A pointer to a shared type: text
    # @param pointer [::FFI::Pointer] A pointer to a transaction
    # @return [void]
    attach_function :ytext_insert, %i[pointer pointer int string], :void

    # Removes a range from a shared type: text. This can be used to delete
    # something from a text.
    #
    # @param pointer [::FFI::Pointer] A pointer to a shared type: text
    # @param pointer [::FFI::Pointer] A pointer to a transaction
    # @param index [Integer] Where we want to start removing things
    # @param length [Integer] How many characters we want to remove
    # @return [void]
    attach_function :ytext_remove_range, %i[pointer pointer int int], :void

    # Reads the content from the provided address and unwraps it into a regular
    # string.
    #
    # @param pointer [::FFI::Pointer] A pointer to a shared type: text
    # @return [String] The string representation of the shared type: text
    attach_function :ytext_string, %i[pointer], :string

    # TODO: this function is not implemented but shows up in the GitHub README
    # attach_function :ytext_destroy, %i[pointer], :void

    # Create a new transaction for the provided doc. Every change to any of the
    # shared types that belong to this doc are covered by this transaction.
    #
    # @example Create a new transaction for a document
    #   doc = Y::CRDT.ydoc_new
    #   tx = Y::CRDT.ytransaction_new(doc)
    #
    # @param pointer [::FFI::Pointer] A pointer to a Y::Doc
    # @return [::FFI::Pointer] A pointer to a transaction
    attach_function :ytransaction_new, [:pointer], :pointer

    # Create a new transaction for the provided doc. Every change to any of the
    # shared types that belong to this doc are covered by this transaction. The
    # value at the location of the address that is provided by the `length`
    # pointer is updated with the length of the generated binary.
    #
    # @example Create a state vector
    #   doc = Y::CRDT.ydoc_new
    #   tx = Y::CRDT.ytransaction_new(doc)
    #   length = FFI::MemoryPointer.new(:int, 0)
    #   serialized_state_vector = Y::CRDT.ytransaction_state_vector_v1(tx, length)
    #
    # @param pointer [::FFI::Pointer] Pointer to transaction
    # @param pointer [::FFI::Pointer] A pointer to an [Integer]
    # @return [::FFI::Pointer] A serialized state vector
    attach_function :ytransaction_state_vector_v1, %i[pointer pointer], :pointer

    # Returns the diff of a given document and the provided state vector. In
    # other words, if there are `n` updates to a document and the state vector
    # was created for `n - m`, there are only going to be `m` changes included
    # in the diff.
    #
    # @param pointer [::FFI:Pointer] A pointer to the transaction we want to
    #   create a diff for
    # @param pointer [::FFI::Pointer, nil] A pointer to pre-calculated state
    #   vector
    # @param pointer [Integer] The length of the provided state vector
    # @param pointer [::FFI::Pointer] An out pointer which receives the size of
    #   state diff
    # @return [::FFI::Pointer] The serialized state diff
    attach_function :ytransaction_state_diff_v1, %i[pointer pointer int pointer], :pointer

    # Commits all operations on the doc that the provided transaction was
    # created for.
    #
    # @param [::FFI::Pointer] A pointer to the transaction that should be
    #   committed
    # @return [void]
    attach_function :ytransaction_commit, %i[pointer], :void
    attach_function :ytransaction_apply, %i[pointer pointer pointer], :void

    attach_function :ybinary_destroy, %i[pointer pointer], :void
    attach_function :ystring_destroy, %i[pointer], :void
  end
end
#
# doc = Y::CRDT.ydoc_new
# tx = Y::CRDT.ytransaction_new(doc)
# txt = Y::CRDT.ytext(tx, "name")
# Y::CRDT.ytext_insert(txt, tx, 0, "hello world")
#
# remote_doc = Y::CRDT.ydoc_new
# remote_txn = Y::CRDT.ytransaction_new(remote_doc)
# remote_txt = Y::CRDT.ytext(remote_txn, "name")
#
# sv_length = ::FFI::MemoryPointer.new(:uint, 0)
# remote_sv = Y::CRDT.ytransaction_state_vector_v1(remote_txn, sv_length)
#
# update_length = ::FFI::MemoryPointer.new(:int, 0)
# update = Y::CRDT.ytransaction_state_diff_v1(tx, remote_sv, sv_length, update_length)
#
# Y::CRDT.ybinary_destroy(remote_sv, sv_length)
# # Y::CRDT.ytext_destroy(txt)
# Y::CRDT.ytransaction_commit(tx)
# Y::CRDT.ydoc_destroy(doc)
#
# Y::CRDT.ytransaction_apply(remote_txn, update, update_length)
# Y::CRDT.ybinary_destroy(update, update_length)
#
# str = Y::CRDT.ytext_string(remote_txt, remote_txn)
# puts str
#
# # Y::CRDT.ystring_destroy(str)
# # Y::CRDT.ytext_destroy(remote_txt)
# Y::CRDT.ytransaction_commit(remote_txn)
# Y::CRDT.ydoc_destroy(remote_doc)
