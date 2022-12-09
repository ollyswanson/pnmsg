# Pnmsg

This is a really basic command line tool for appending messages to PNG files.
It's a suggested project for those learning Rust (which I started with just over
a week ago - Sep 2020).

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

I'll probably come back to fix this once I have learnt some more Rust. I'm
getting to grips with the syntax and the concepts, so that's the focus here
rather than a clean API etc.

## Update (Dec 2022)
Spent an afternoon refactoring this for fun, removed many of the tests with the
intention of adding good integration tests at some point... 2024 maybe?
