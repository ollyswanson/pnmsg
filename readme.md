# Pnmsg

This is a really basic command line tool for appending messages to PNG files.
It's a suggested project for those learning Rust (which I started with a couple
of weeks ago).

You can encode a message with 
```
cargo run encode [file] [chunk_type] [message] [output]
```
and decode with
```
cargo run decode [file] [chunk_type]
```

`chunk_type` must be 4 characters and ASCII alphabetic, the third character
should be uppercase such as `olLy`, if you decode `message.png` you'll find
a message in there, alternatively you could just open the file in a text editor
and find the message that way, (it's really not the best way to hide secrets).
