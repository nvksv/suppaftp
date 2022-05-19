//! # Data Stream
//!
//! This module exposes the async data stream implementation where bytes must be written to/read from

#[cfg(feature = "async-secure")]
use async_native_tls::TlsStream as TlsStreamAsync;
#[cfg(feature = "async")]
use async_std::{
    io::{Read as ReadAsync, Result as ResultAsync, Write as WriteAsync},
    net::TcpStream as TcpStreamAsync
};
#[cfg(feature = "sync-secure")]
use native_tls::TlsStream as TlsStreamSync;
#[cfg(feature = "sync")]
use std::{
    io::{Read as ReadSync, Result as ResultSync, Write as WriteSync},
    net::TcpStream as TcpStreamSync
};
#[cfg(feature = "async")]
use pin_project::pin_project;
#[cfg(feature = "async")]
use std::pin::Pin;

/// Data Stream used for communications. It can be both of type Tcp in case of plain communication or Ssl in case of FTPS
#[cfg(feature = "async")]
#[pin_project(project = DataStreamProjAsync)]
#[derive(Debug)]
pub enum DataStreamAsync {
    Tcp(#[pin] TcpStreamAsync),
    #[cfg(feature = "async-secure")]
    Ssl(#[pin] TlsStreamAsync<TcpStreamAsync>),
}

/// Data Stream used for communications. It can be both of type Tcp in case of plain communication or Ssl in case of FTPS
#[cfg(feature = "sync")]
//#[pin_project(project = DataStreamProjSync)]
#[derive(Debug)]
pub enum DataStreamSync {
    Tcp(TcpStreamSync),
    #[cfg(feature = "sync-secure")]
    Ssl(TlsStreamWrapperSync),
}

#[cfg(feature = "async")]
impl DataStreamAsync {
    /// Unwrap the stream into TcpStream. This method is only used in secure connection.
    pub fn into_tcp_stream(self) -> TcpStreamAsync {
        match self {
            DataStreamAsync::Tcp(stream) => stream,
            #[cfg(feature = "async-secure")]
            DataStreamAsync::Ssl(stream) => stream.get_ref().clone(),
        }
    }
}

impl DataStreamSync {
    /// Unwrap the stream into TcpStream. This method is only used in secure connection.
    pub fn into_tcp_stream(self) -> TcpStreamSync {
        match self {
            DataStreamSync::Tcp(stream) => stream,
            DataStreamSync::Ssl(stream) => stream.tcp_stream(),
        }
    }
}

#[maybe_async::maybe(sync(feature="sync"), async(feature="async"), idents = "DataStream, TcpStream, TlsStream")]
impl DataStream {
    /// Returns a reference to the underlying TcpStream.
    pub fn get_ref(&self) -> &TcpStream {
        match self {
            DataStream::Tcp(ref stream) => stream,
            #[cfg(feature = "async-secure")]
            DataStream::Ssl(ref stream) => stream.get_ref(),
        }
    }
}


// -- sync

#[maybe_async::maybe(sync(feature="sync"), idents = "DataStream, DataStreamProj, TcpStream, TlsStream, Read, Result")]
impl Read for DataStream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        match self {
            DataStream::Tcp(ref mut stream) => stream.read(buf),
            #[cfg(any(feature = "secure", feature = "async-secure"))]
            DataStream::Ssl(ref mut stream) => stream.mut_ref().read(buf),
        }
    }
}

#[maybe_async::maybe(sync(feature="sync"), idents = "DataStream, DataStreamProj, TcpStream, TlsStream, Write, Result")]
impl Write for DataStream {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        match self {
            DataStream::Tcp(ref mut stream) => stream.write(buf),
            #[cfg(any(feature = "secure", feature = "async-secure"))]
            DataStream::Ssl(ref mut stream) => stream.mut_ref().write(buf),
        }
    }

    fn flush(&mut self) -> Result<()> {
        match self {
            DataStream::Tcp(ref mut stream) => stream.flush(),
            #[cfg(any(feature = "secure", feature = "async-secure"))]
            DataStream::Ssl(ref mut stream) => stream.mut_ref().flush(),
        }
    }
}

// -- tls stream wrapper to implement drop...

#[cfg(any(feature = "secure", feature = "async-secure"))]
#[derive(Debug)]
/// Tls stream wrapper. This type is a garbage data type used to impl the drop trait for the tls stream.
/// This allows me to keep returning `Read` and `Write` traits in stream methods
pub struct TlsStreamWrapperSync {
    stream: TlsStreamSync<TcpStreamSync>,
    ssl_shutdown: bool,
}

#[cfg(any(feature = "secure", feature = "async-secure"))]
impl TlsStreamWrapperSync {
    /// Get underlying tcp stream
    pub(crate) fn tcp_stream(mut self) -> TcpStreamSync {
        let mut stream = self.stream.get_ref().try_clone().unwrap();
        // Don't perform shutdown later
        self.ssl_shutdown = false;
        // flush stream (otherwise can cause bad chars on channel)
        if let Err(err) = stream.flush() {
            error!("Error in flushing tcp stream: {}", err);
        }
        trace!("TLS stream terminated");
        stream
    }

    /// Get ref to underlying tcp stream
    pub(crate) fn get_ref(&self) -> &TcpStreamSync {
        self.stream.get_ref()
    }

    /// Get mutable reference to tls stream
    pub(crate) fn mut_ref(&mut self) -> &mut TlsStreamSync<TcpStreamSync> {
        &mut self.stream
    }
}

#[cfg(any(feature = "secure", feature = "async-secure"))]
impl From<TlsStreamSync<TcpStreamSync>> for TlsStreamWrapperSync {
    fn from(stream: TlsStreamSync<TcpStreamSync>) -> Self {
        Self {
            stream,
            ssl_shutdown: true,
        }
    }
}

#[cfg(any(feature = "secure", feature = "async-secure"))]
impl Drop for TlsStreamWrapperSync {
    fn drop(&mut self) {
        if self.ssl_shutdown {
            if let Err(err) = self.stream.shutdown() {
                error!("Failed to shutdown stream: {}", err);
            } else {
                debug!("TLS Stream shut down");
            }
        }
    }
}



// -- async

#[maybe_async::maybe(async(feature="async"), idents = "DataStream, DataStreamProj, TcpStream, TlsStream, Read, Result")]
impl Read for DataStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut [u8],
    ) -> std::task::Poll<Result<usize>> {
        match self.project() {
            DataStreamProj::Tcp(stream) => stream.poll_read(cx, buf),
            #[cfg(feature = "async-secure")]
            DataStreamProj::Ssl(stream) => stream.poll_read(cx, buf),
        }
    }
}

#[maybe_async::maybe(async(feature="async"), idents = "DataStream, DataStreamProj, TcpStream, TlsStream, Write, Result")]
impl Write for DataStream {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<Result<usize>> {
        match self.project() {
            DataStreamProj::Tcp(stream) => stream.poll_write(cx, buf),
            #[cfg(feature = "async-secure")]
            DataStreamProj::Ssl(stream) => stream.poll_write(cx, buf),
        }
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<()>> {
        match self.project() {
            DataStreamProj::Tcp(stream) => stream.poll_flush(cx),
            #[cfg(feature = "async-secure")]
            DataStreamProj::Ssl(stream) => stream.poll_flush(cx),
        }
    }

    fn poll_close(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<()>> {
        match self.project() {
            DataStreamProj::Tcp(stream) => stream.poll_close(cx),
            #[cfg(feature = "async-secure")]
            DataStreamProj::Ssl(stream) => stream.poll_close(cx),
        }
    }
}
