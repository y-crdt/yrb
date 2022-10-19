# Examples

> Usage patterns

## Send a full document update to a client

```ruby
doc = Y::Doc.new
text = doc.get_text("about")
text << "My name is, my name is, my name is … Slim Shady"

zero = Y::Doc.new
update = doc.diff(zero.state)

# transfer could be anything, ActionCable broadcast, HTTP response, …
transfer update
```
