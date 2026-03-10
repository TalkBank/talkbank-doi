## update-chat-types

https://github.com/TalkBank/update-chat-types

Implemented in Rust. Runs fastest on a local machine and file system
on SSD.

TODO License.

### Binaries built

On M1 Mac:

``` shellsession
$ cargo build --release
```

generates binary to `target/release/update-chat-types`.

Cross-compilation:

Just once, do

``` shellsession
$ rustup target add x86_64-apple-darwin
```

``` shellsession
$ cargo build --release --target x86_64-apple-darwin
```

generates binary to `target/x86_64-apple-darwin/release/update-chat-types`.
