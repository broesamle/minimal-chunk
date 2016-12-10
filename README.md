Mimimal Example for http chunked encoding
=========================================

install / build
---------------

```
git clone https://github.com/broesamle/minimal-chunk.git
cd minimal-chunk
cargo build --bin writechunks
cargo build --bin server
```

testing
-------

Start the server:

`cargo run --bin server`

you should see something like:

```
    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/server`
streaming on ("127.0.0.1", 50123)
```

Then in another terminal:

`curl -v http://localhost:50123/`
