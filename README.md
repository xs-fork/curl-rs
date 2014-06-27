# curl

Master
![build status](https://travis-ci.org/vhbit/curl-rs.svg?branch=master)

Dev
![build status](https://travis-ci.org/vhbit/curl-rs.svg?branch=dev)

A lightweight Curl-wrapper for using (mostly) HTTP from Rust.

While there are a couple of Rust HTTP libraries like
[rust-http](https://github.com/chris-morgan/rust-http) and its
successor [teepee](https://github.com/teepee/teepee). But the first
one is considered deprecated and the second one is not pretty usable
yet. Plus SSL support comes only with OpenSSL.

Curl, in other hand, has pretty good history + allows to use a lot of
different SSL backends, for example, it works with Apple security
frameworks on Mac/iOS. It also can be used to support other protocols
although they're not in plans in near future.

I hope situation in Rust-land will change soon and it will be possible
to drop this one in favor of Rust-only stack.

## Docs

[Available here](http://www.rust-ci.org/vhbit/curl-rs/doc/curl/)

## compile

    make lib

## test

    make tests
