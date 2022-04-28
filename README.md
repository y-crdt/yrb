# Yrb

Yrb is a Ruby binding for Y-CRDT. It provides distributed data types that enable
real-time collaboration between devices. Ypy can sync data with any other
platform that has a Y-CRDT binding, allowing for seamless cross-domain
communication.

The library is a thin wrapper around Yrs, taking advantage of the safety and
performance of Rust.

## Installation

Add this line to your application's Gemfile:

```ruby
gem 'y-rb'
```

And then execute:

    $ bundle install

Or install it yourself as:

    $ gem install y-rb

## Usage

```ruby
local_doc = Y::Doc.new
local_transaction = local_doc.transact
local_text = local_transaction.get_text("name")
local_text.push(local_transaction, "hello ")

remote_doc = Y::Doc.new
remote_transaction = remote_doc.transact
remote_text = remote_transaction.get_text("name")

state_vector_remote = remote_doc.state_vector
update_remote = local_doc.encode_diff_v1(state_vector_remote)

remote_transaction.apply_update(update_remote)

puts remote_text.to_s == local_text.to_s # true
```

## Development

After checking out the repo, run `bin/setup` to install dependencies. Then,
run `rake spec` to run the tests. You can also run `bin/console` for an
interactive prompt that will allow you to experiment.

To install this gem onto your local machine, run `bundle exec rake install`.
To release a new version, update the version number in `version.rb`, and then
run `bundle exec rake release`, which will create a git tag for the version,
push git commits and the created tag, and push the `.gem` file to
[rubygems.org](https://rubygems.org).

## License

The gem is available as open source under the terms of the
[MIT License](https://opensource.org/licenses/MIT).
