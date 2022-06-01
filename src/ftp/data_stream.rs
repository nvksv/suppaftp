//! # Data Stream
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


/// Data Stream used for communications. It can be both of type Tcp in case of plain communication or Tls in case of FTPS
#[cfg(feature = "async")]
#[pin_project(project = DataStreamProjAsync)]
#[derive(Debug)]
pub enum DataStreamAsync {
    Tcp(#[pin] TcpStreamAsync),
    #[cfg(feature = "async-secure")]
    Tls(#[pin] TlsStreamAsync<TcpStreamAsync>),
}

/// Data Stream used for communications. It can be both of type Tcp in case of plain communication or Tls in case of FTPS
#[cfg(feature = "sync")]
#[derive(Debug)]
pub enum DataStreamSync {
    Tcp(TcpStreamSync),
    #[cfg(feature = "sync-secure")]
    Tls(TlsStreamWrapperSync),
}

#[cfg(feature = "async")]
impl DataStreamAsync {
    /// Unwrap the stream into TcpStream. This method is only used in secure connection.
    pub fn into_tcp_stream(self) -> TcpStreamAsync {
        match self {
            DataStreamAsync::Tcp(stream) => stream,
            #[cfg(feature = "async-secure")]
            DataStreamAsync::Tls(stream) => stream.get_ref().clone(),
        }
    }
}

#[cfg(feature = "sync")]
impl DataStreamSync {
    /// Unwrap the stream into TcpStream. This method is only used in secure connection.
    pub fn into_tcp_stream(self) -> TcpStreamSync {
        match self {
            DataStreamSync::Tcp(stream) => stream,
            #[cfg(feature = "sync-secure")]
            DataStreamSync::Tls(stream) => stream.tcp_stream(),
        }
    }
}

#[maybe_async::maybe(
    sync(feature="sync", replace_features(_secure = "sync-secure")), 
    async(feature="async", replace_features(_secure = "async-secure")), 
)]
impl DataStream {
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

#[maybe_async::maybe(
    sync(feature="sync", replace_features(_secure = "sync-secure")), 
    idents(DataStream, DataStreamProj, TcpStream, TlsStream, Read, Result),
)]
impl Read for DataStream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        match self {
            DataStream::Tcp(ref mut stream) => stream.read(buf),
            #[cfg(feature = "_secure")]
            DataStream::Tls(ref mut stream) => stream.mut_ref().read(buf),
        }
    }
}

#[maybe_async::maybe(sync(feature="sync", replace_features(_secure = "sync-secure")))]
impl Write for DataStream {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        match self {
            DataStream::Tcp(ref mut stream) => stream.write(buf),
            #[cfg(feature = "_secure")]
            DataStream::Tls(ref mut stream) => stream.mut_ref().write(buf),
        }
    }

    fn flush(&mut self) -> Result<()> {
        match self {
            DataStream::Tcp(ref mut stream) => stream.flush(),
            #[cfg(feature = "_secure")]
            DataStream::Tls(ref mut stream) => stream.mut_ref().flush(),
        }
    }
}

// -- tls stream wrapper to implement drop...

#[maybe_async::maybe(sync(feature = "sync-secure"), async(feature = "async-secure"))]
#[derive(Debug)]
/// Tls stream wrapper. This type is a garbage data type used to impl the drop trait for the tls stream.
/// This allows me to keep returning `Read` and `Write` traits in stream methods
pub struct TlsStreamWrapper {
    stream: TlsStream<TcpStream>,
    #[cfg(feature = "sync-secure")]
    tls_shutdown: bool,
}


#[maybe_async::maybe(sync(feature = "sync-secure"), async(feature = "async-secure"))]
impl TlsStreamWrapper {
    /// Get underlying tcp stream
    #[cfg(feature = "sync-secure")]
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
    #[cfg(feature = "async-secure")]
    pub(crate) fn tcp_stream(mut self) -> TcpStream {
        let mut stream = self.stream.get_ref().clone();
        stream
    }

    /// Get ref to underlying tcp stream
    pub(crate) fn get_ref(&self) -> &TcpStream {
        self.stream.get_ref()
    }

    /// Get mutable reference to tls stream
    pub(crate) fn mut_ref(&mut self) -> &mut TlsStream<TcpStream> {
        &mut self.stream
    }
}

#[maybe_async::maybe(sync(feature = "sync-secure"), async(feature = "async-secure"))]
impl From<TlsStream<TcpStream>> for TlsStreamWrapper {
    fn from(stream: TlsStream<TcpStream>) -> Self {
        Self {
            stream,
            #[cfg(feature = "sync-secure")]
            tls_shutdown: true,
        }
    }
}

#[maybe_async::maybe(sync(feature = "sync-secure"), async(feature = "async-secure"))]
#[cfg(feature = "sync-secure")]
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



// -- async

#[maybe_async::maybe(async(feature="async", replace_features(_secure = "async-secure")))]
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

#[maybe_async::maybe(async(feature="async", replace_features(_secure = "async-secure")))]
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