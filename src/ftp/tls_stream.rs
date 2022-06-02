//! # TLS Stream
//!
//! This module exposes the async data stream implementation where bytes must be written to/read from

maybe_async::content! {

#![maybe_async::default(
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

#[maybe_async::maybe(sync(feature = "sync-secure"), async(feature = "async-secure"))]
use async_native_tls::TlsStream;

#[maybe_async::maybe(sync(feature = "sync"), async(feature = "async"))]
use async_std::{
    io::{Read, Result, Write},
    net::TcpStream
};

#[cfg(feature = "async")]
use pin_project::pin_project;
#[cfg(feature = "async")]
use std::pin::Pin;


// -- tls stream wrapper to implement drop...

#[maybe_async::maybe(
    sync(feature = "sync", replace_feature("_secure", "sync-secure"), drop_attrs(pin)), 
    async(feature = "async", replace_feature("_secure", "async-secure"), inner("pin_project(project = TlsStreamWrapperProjAsync)")),
)]
#[derive(Debug)]
/// Tls stream wrapper. This type is a garbage data type used to impl the drop trait for the tls stream.
/// This allows me to keep returning `Read` and `Write` traits in stream methods
pub struct TlsStreamWrapper {
    #[pin]
    stream: TlsStream<TcpStream>,
    #[cfg(feature = "sync-secure")]
    tls_shutdown: bool,
}

#[maybe_async::maybe(
    sync(feature = "sync", replace_feature("_secure", "sync-secure")), 
    async(feature = "async", replace_feature("_secure", "async-secure")),
)]
impl TlsStreamWrapper {
    /// Get underlying tcp stream
    #[maybe_async::only_if(sync)]
    pub(crate) fn tcp_stream(mut self) -> TcpStream {
        let mut stream = self.stream.get_ref().try_clone().unwrap();

        // Don't perform shutdown later
        self.tls_shutdown = false;
        // flush stream (otherwise can cause bad chars on channel)
        if let Err(err) = stream.flush() {
            error!("Error in flushing tcp stream: {}", err);
        }
        trace!("TLS stream terminated");
        stream
    }

    /// Get underlying tcp stream
    #[maybe_async::only_if(async)]
    pub(crate) fn tcp_stream(self) -> TcpStream {
        self.stream.get_ref().clone()
    }

    /// Get ref to underlying tcp stream
    pub(crate) fn get_ref(&self) -> &TcpStream {
        self.stream.get_ref()
    }
}

#[maybe_async::maybe(
    sync(feature = "sync", replace_feature("_secure", "sync-secure")), 
    async(feature = "async", replace_feature("_secure", "async-secure")),
)]
impl From<TlsStream<TcpStream>> for TlsStreamWrapper {
    fn from(stream: TlsStream<TcpStream>) -> Self {
        Self {
            stream,
            #[cfg(feature = "sync-secure")]
            tls_shutdown: true,
        }
    }
}

// -- sync

#[maybe_async::maybe(sync(feature = "sync-secure"), async(feature = "async-secure"))]
#[maybe_async::only_if(sync)]
impl Drop for TlsStreamWrapper {
    fn drop(&mut self) {
        if self.tls_shutdown {
            if let Err(err) = self.stream.shutdown() {
                error!("Failed to shutdown stream: {}", err);
            } else {
                debug!("TLS Stream shut down");
            }
        }
    }
}

#[maybe_async::maybe(sync(feature="sync", replace_feature("_secure", "sync-secure")))]
impl Read for TlsStreamWrapper {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.stream.read(buf)
    }
}

#[maybe_async::maybe(sync(feature="sync", replace_feature("_secure", "sync-secure")))]
impl Write for TlsStreamWrapper {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.stream.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.stream.flush()
    }
}

// -- async

#[maybe_async::maybe(async(feature="async", replace_feature("_secure", "async-secure")))]
impl Read for TlsStreamWrapper {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut [u8],
    ) -> std::task::Poll<Result<usize>> {
        self.project().stream.poll_read(cx, buf)
    }
}

#[maybe_async::maybe(async(feature="async", replace_feature("_secure", "async-secure")))]
impl Write for TlsStreamWrapper {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<Result<usize>> {
        self.project().stream.poll_write(cx, buf)
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<()>> {
        self.project().stream.poll_flush(cx)
    }

    fn poll_close(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<()>> {
        self.project().stream.poll_close(cx)
    }
}

}