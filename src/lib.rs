//! Asynchronous TLS/SSL streams for async-std and AsyncRead/AsyncWrite sockets using [Rustls](https://github.com/ctz/rustls).

#![feature(async_await)]

pub mod client;
mod common;
pub mod server;
mod connector;
mod acceptor;

pub use acceptor::TlsAcceptor as TlsAcceptor;
pub use connector::TlsConnector as TlsConnector;