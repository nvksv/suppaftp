//! # Data Stream
//!
//! This module exposes the async data stream implementation where bytes must be written to/read from

maybe_async_cfg::content! {

#![maybe_async_cfg::default(
    idents(
        async_native_tls(sync="native_tls", async), 
        async_std(sync="std", async), 
        Read(use), 
        Result(use), 
        Write(use), 
        TcpStream(use),
        TlsStream(use),
        DataStream,
        DataStreamProj,
        TlsStreamWrapper,
        TlsStreamWrapperProj,
    ),
)]

#[maybe_async_cfg::maybe(sync(feature = "sync-secure"), async(feature = "async-secure"))]
use super::tls_stream::TlsStreamWrapper;

#[maybe_async_cfg::maybe(sync(feature = "sync"), async(feature = "async"))]
use async_std::{
    io::{Read, Result, Write},
    net::TcpStream
};

#[cfg(feature = "async")]
use pin_project::pin_project;
#[cfg(feature = "async")]
use std::pin::Pin;


/// Data Stream used for communications. It can be both of type Tcp in case of plain communication or Tls in case of FTPS
#[maybe_async_cfg::maybe(
    sync(feature = "sync", replace_feature("_secure", "sync-secure"), drop_attrs(pin)), 
    async(feature = "async", replace_feature("_secure", "async-secure"), inner("pin_project(project = DataStreamProjAsync)")),
)]
#[derive(Debug)]
pub enum DataStream {
    Tcp(#[pin] TcpStream),
    #[cfg(feature = "_secure")]
    Tls(#[pin] TlsStreamWrapper),
}

#[maybe_async_cfg::maybe(
    sync(feature = "sync", replace_feature("_secure", "sync-secure")), 
    async(feature = "async", replace_feature("_secure", "async-secure")),
)]
impl DataStream {
    /// Unwrap the stream into TcpStream. This method is only used in secure connection.
    pub fn into_tcp_stream(self) -> TcpStream {
        match self {
            DataStream::Tcp(stream) => stream,
            #[cfg(feature = "_secure")]
            DataStream::Tls(stream) => stream.tcp_stream(),
        }
    }

    /// Returns a reference to the underlying TcpStream.
    pub fn get_ref(&self) -> &TcpStream {
        match self {
            DataStream::Tcp(ref stream) => stream,
            #[cfg(feature = "_secure")]
            DataStream::Tls(ref stream) => stream.get_ref(),
        }
    }
}


// -- sync

#[maybe_async_cfg::maybe(sync(feature="sync", replace_feature("_secure", "sync-secure")))]
impl Read for DataStream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        match self {
            DataStream::Tcp(ref mut stream) => stream.read(buf),
            #[cfg(feature = "_secure")]
            DataStream::Tls(ref mut stream) => stream.read(buf),
        }
    }
}

#[maybe_async_cfg::maybe(sync(feature="sync", replace_feature("_secure", "sync-secure")))]
impl Write for DataStream {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        match self {
            DataStream::Tcp(ref mut stream) => stream.write(buf),
            #[cfg(feature = "_secure")]
            DataStream::Tls(ref mut stream) => stream.write(buf),
        }
    }

    fn flush(&mut self) -> Result<()> {
        match self {
            DataStream::Tcp(ref mut stream) => stream.flush(),
            #[cfg(feature = "_secure")]
            DataStream::Tls(ref mut stream) => stream.flush(),
        }
    }
}

// -- sync

#[maybe_async_cfg::maybe(async(feature="async", replace_feature("_secure", "async-secure")))]
impl Read for DataStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut [u8],
    ) -> std::task::Poll<Result<usize>> {
        match self.project() {
            DataStreamProj::Tcp(stream) => stream.poll_read(cx, buf),
            #[cfg(feature = "_secure")]
            DataStreamProj::Tls(stream) => stream.poll_read(cx, buf),
        }
    }
}

#[maybe_async_cfg::maybe(async(feature="async", replace_feature("_secure", "async-secure")))]
impl Write for DataStream {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<Result<usize>> {
        match self.project() {
            DataStreamProj::Tcp(stream) => stream.poll_write(cx, buf),
            #[cfg(feature = "_secure")]
            DataStreamProj::Tls(stream) => stream.poll_write(cx, buf),
        }
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<()>> {
        match self.project() {
            DataStreamProj::Tcp(stream) => stream.poll_flush(cx),
            #[cfg(feature = "_secure")]
            DataStreamProj::Tls(stream) => stream.poll_flush(cx),
        }
    }

    fn poll_close(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<()>> {
        match self.project() {
            DataStreamProj::Tcp(stream) => stream.poll_close(cx),
            #[cfg(feature = "_secure")]
            DataStreamProj::Tls(stream) => stream.poll_close(cx),
        }
    }
}

}