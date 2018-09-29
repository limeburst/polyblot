verbose-engine
==============

github gave me this name.


Contributing
------------

```bash
$ cargo install systemfd cargo-watch
$ systemfd --no-pid -s http::3000 -- cargo watch -x run 
$ open http://localhost:3000
```