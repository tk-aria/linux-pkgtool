# rust-boilerplate
boilerplate rust.

- [playground](https://play.rust-lang.org/)

## documents

- http://www.tohoho-web.com/ex/rust.html
- https://doc.rust-jp.rs/
  - https://doc.rust-jp.rs/book/second-edition/ch01-01-installation.html

## test

```
$ cargo run
$ curl http://localhost:8088
```

```
$ ssh -R 80:localhost:8080 ssh.localhost.run
```

## manual install rls in docker

```
/usr/local/cargo/bin/rustup component add rust-analysis --toolchain 1.46.0-x86_64-unknown-linux-gnu
/usr/local/cargo/bin/rustup component add rust-src --toolchain 1.46.0-x86_64-unknown-linux-gnu
/usr/local/cargo/bin/rustup component add rls --toolchain 1.46.0-x86_64-unknown-linux-gnu
```
