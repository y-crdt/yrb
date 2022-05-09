# Decision log

## 2022-05-09

### Supported operations on data types are either read-only or mutable

In a meeting with the wider `y-crdt` group on May 6th, 2022, it became clear
that the cost of creating a new instance of a data type is common operation in
many languages (Python, Ruby, …), but it is not something we should to support.
This pattern leads to all sort of problems, mostly because it contradicts the
common usage pattern of replicating changes between two or more clients.

Instead, the API should be explicit about this fact and should not make it too
easy to do that.

## 2022-05-06

### Transactions are implicit by default

The developer should not be exposed to transactions until they need to
change the default logic in any way. If someone creates a structure and inserts,
removes, this should be part of the same transaction until stated otherwise.

### Synchronisation happens at the document level

It might be interesting to sync at a more granular level, but for the sake of
simplicity, the first iteration of this library will only support
synchronization of the complete document.

## 2022-05-05

### No direct exposure of internal API

The internal API (`y-crdt`) is subject to constant changes and does not
necessarily offer an idiomatic Ruby interface. Therefore, the public API of
`y-rb` does not follow the `y-crdt` API, but prefers familiar Ruby idioms when
possible and might even require libraries where it makes sense (e.g. `nokogiri` for XML).

### Rutie is a temporary dependency

The Ruby<->Rust binding feels immature. But it is not the goal of this project
to fix this immediately. The long-term vision is to replace this part by
something more lightweight and opinionated that could be part of the Rust
codebase of this project.
