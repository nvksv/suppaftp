mod ftp {
    //! # Async
    //!
    //! This module contains the definition for all async implementation of suppaftp
    mod data_stream {
        //! # Data Stream
        //!
        //! This module exposes the async data stream implementation where bytes must be written to/read from
        #[cfg(feature = "async-secure")]
        use async_native_tls::TlsStream as TlsStreamAsync;
        #[cfg(feature = "async")]
        use async_std::{
            io::{Read as ReadAsync, Result as ResultAsync, Write as WriteAsync},
            net::TcpStream as TcpStreamAsync,
        };
        #[cfg(feature = "sync-secure")]
        use native_tls::TlsStream as TlsStreamSync;
        #[cfg(feature = "sync")]
        use std::{
            io::{Read as ReadSync, Result as ResultSync, Write as WriteSync},
            net::TcpStream as TcpStreamSync,
        };
        #[cfg(feature = "async")]
        use pin_project::pin_project;
        #[cfg(feature = "async")]
        use std::pin::Pin;
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
        #[cfg(feature = "sync")]
        impl DataStreamSync {
            /// Returns a reference to the underlying TcpStream.
            pub fn get_ref(&self) -> &TcpStreamSync {
                match self {
                    DataStreamSync::Tcp(ref stream) => stream,
                    #[cfg(feature = "async-secure")]
                    DataStreamSync::Ssl(ref stream) => stream.get_ref(),
                }
            }
        }
        #[cfg(feature = "async")]
        impl DataStreamAsync {
            /// Returns a reference to the underlying TcpStream.
            pub fn get_ref(&self) -> &TcpStreamAsync {
                match self {
                    DataStreamAsync::Tcp(ref stream) => stream,
                    #[cfg(feature = "async-secure")]
                    DataStreamAsync::Ssl(ref stream) => stream.get_ref(),
                }
            }
        }
        #[cfg(feature = "sync")]
        impl ReadSync for DataStreamSync {
            fn read(&mut self, buf: &mut [u8]) -> ResultSync<usize> {
                match self {
                    DataStreamSync::Tcp(ref mut stream) => stream.read(buf),
                    #[cfg(any(feature = "secure", feature = "async-secure"))]
                    DataStreamSync::Ssl(ref mut stream) => stream.mut_ref().read(buf),
                }
            }
        }
        #[cfg(feature = "sync")]
        impl WriteSync for DataStreamSync {
            fn write(&mut self, buf: &[u8]) -> ResultSync<usize> {
                match self {
                    DataStreamSync::Tcp(ref mut stream) => stream.write(buf),
                    #[cfg(any(feature = "secure", feature = "async-secure"))]
                    DataStreamSync::Ssl(ref mut stream) => stream.mut_ref().write(buf),
                }
            }
            fn flush(&mut self) -> ResultSync<()> {
                match self {
                    DataStreamSync::Tcp(ref mut stream) => stream.flush(),
                    #[cfg(any(feature = "secure", feature = "async-secure"))]
                    DataStreamSync::Ssl(ref mut stream) => stream.mut_ref().flush(),
                }
            }
        }
        #[cfg(any(feature = "secure", feature = "async-secure"))]
        /// Tls stream wrapper. This type is a garbage data type used to impl the drop trait for the tls stream.
        /// This allows me to keep returning `Read` and `Write` traits in stream methods
        pub struct TlsStreamWrapperSync {
            stream: TlsStreamSync<TcpStreamSync>,
            ssl_shutdown: bool,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for TlsStreamWrapperSync {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    TlsStreamWrapperSync {
                        stream: ref __self_0_0,
                        ssl_shutdown: ref __self_0_1,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "TlsStreamWrapperSync");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "stream",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "ssl_shutdown",
                            &&(*__self_0_1),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        #[cfg(any(feature = "secure", feature = "async-secure"))]
        impl TlsStreamWrapperSync {
            /// Get underlying tcp stream
            pub(crate) fn tcp_stream(mut self) -> TcpStreamSync {
                let mut stream = self.stream.get_ref().try_clone().unwrap();
                self.ssl_shutdown = false;
                if let Err(err) = stream.flush() {
                    {
                        let lvl = ::log::Level::Error;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api_log(
                                ::core::fmt::Arguments::new_v1(
                                    &["Error in flushing tcp stream: "],
                                    &[::core::fmt::ArgumentV1::new_display(&err)],
                                ),
                                lvl,
                                &(
                                    "suppaftp::ftp::data_stream",
                                    "suppaftp::ftp::data_stream",
                                    "src\\ftp\\data_stream.rs",
                                    136u32,
                                ),
                                ::log::__private_api::Option::None,
                            );
                        }
                    };
                }
                {
                    let lvl = ::log::Level::Trace;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api_log(
                            ::core::fmt::Arguments::new_v1(&["TLS stream terminated"], &[]),
                            lvl,
                            &(
                                "suppaftp::ftp::data_stream",
                                "suppaftp::ftp::data_stream",
                                "src\\ftp\\data_stream.rs",
                                138u32,
                            ),
                            ::log::__private_api::Option::None,
                        );
                    }
                };
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
                        {
                            let lvl = ::log::Level::Error;
                            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                                ::log::__private_api_log(
                                    ::core::fmt::Arguments::new_v1(
                                        &["Failed to shutdown stream: "],
                                        &[::core::fmt::ArgumentV1::new_display(&err)],
                                    ),
                                    lvl,
                                    &(
                                        "suppaftp::ftp::data_stream",
                                        "suppaftp::ftp::data_stream",
                                        "src\\ftp\\data_stream.rs",
                                        168u32,
                                    ),
                                    ::log::__private_api::Option::None,
                                );
                            }
                        };
                    } else {
                        {
                            let lvl = ::log::Level::Debug;
                            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                                ::log::__private_api_log(
                                    ::core::fmt::Arguments::new_v1(&["TLS Stream shut down"], &[]),
                                    lvl,
                                    &(
                                        "suppaftp::ftp::data_stream",
                                        "suppaftp::ftp::data_stream",
                                        "src\\ftp\\data_stream.rs",
                                        170u32,
                                    ),
                                    ::log::__private_api::Option::None,
                                );
                            }
                        };
                    }
                }
            }
        }
        #[cfg(feature = "async")]
        impl ReadAsync for DataStreamAsync {
            fn poll_read(
                self: Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
                buf: &mut [u8],
            ) -> std::task::Poll<ResultAsync<usize>> {
                match self.project() {
                    DataStreamProjAsync::Tcp(stream) => stream.poll_read(cx, buf),
                    #[cfg(feature = "async-secure")]
                    DataStreamProjAsync::Ssl(stream) => stream.poll_read(cx, buf),
                }
            }
        }
        #[cfg(feature = "async")]
        impl WriteAsync for DataStreamAsync {
            fn poll_write(
                self: Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
                buf: &[u8],
            ) -> std::task::Poll<ResultAsync<usize>> {
                match self.project() {
                    DataStreamProjAsync::Tcp(stream) => stream.poll_write(cx, buf),
                    #[cfg(feature = "async-secure")]
                    DataStreamProjAsync::Ssl(stream) => stream.poll_write(cx, buf),
                }
            }
            fn poll_flush(
                self: std::pin::Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
            ) -> std::task::Poll<ResultAsync<()>> {
                match self.project() {
                    DataStreamProjAsync::Tcp(stream) => stream.poll_flush(cx),
                    #[cfg(feature = "async-secure")]
                    DataStreamProjAsync::Ssl(stream) => stream.poll_flush(cx),
                }
            }
            fn poll_close(
                self: std::pin::Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
            ) -> std::task::Poll<ResultAsync<()>> {
                match self.project() {
                    DataStreamProjAsync::Tcp(stream) => stream.poll_close(cx),
                    #[cfg(feature = "async-secure")]
                    DataStreamProjAsync::Ssl(stream) => stream.poll_close(cx),
                }
            }
        }
    }
    use super::types::{FileType, FtpError, FtpResult, Mode, Response};
    use super::Status;
    use crate::command::Command;
    #[cfg(feature = "_secure")]
    use crate::command::ProtectionLevel;
    #[cfg(feature = "sync")]
    use data_stream::DataStreamSync;
    #[cfg(feature = "async")]
    use data_stream::DataStreamAsync;
    use super::utils::*;
    #[cfg(feature = "async-secure")]
    use async_native_tls::TlsConnector as TlsConnectorAsync;
    #[cfg(feature = "async")]
    use async_std::{
        io::{
            copy as copy_async, BufReader as BufReaderAsync, Read as ReadAsync, Write as WriteAsync,
        },
        net::ToSocketAddrs as ToSocketAddrsAsync,
        net::{
            SocketAddr as SocketAddrAsync, TcpListener as TcpListenerAsync,
            TcpStream as TcpStreamAsync,
        },
        prelude::*,
    };
    #[cfg(feature = "sync-secure")]
    use native_tls::TlsConnector as TlsConnectorSync;
    #[cfg(feature = "sync")]
    use std::{
        io::{
            copy as copy_sync, BufRead, BufReader as BufReaderSync, Read as ReadSync,
            Write as WriteSync,
        },
        net::{
            SocketAddr as SocketAddrSync, TcpListener as TcpListenerSync,
            TcpStream as TcpStreamSync, ToSocketAddrs as ToSocketAddrsSync,
        },
    };
    use chrono::offset::TimeZone;
    use chrono::{DateTime, Utc};
    use std::str::FromStr;
    use std::string::String;
    #[cfg(feature = "sync-secure")]
    /// Some data for TLS mode
    pub struct TlsCtxSync {
        pub tls_connector: TlsConnectorSync,
        pub domain: String,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for TlsCtxSync {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                TlsCtxSync {
                    tls_connector: ref __self_0_0,
                    domain: ref __self_0_1,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "TlsCtxSync");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "tls_connector",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "domain",
                        &&(*__self_0_1),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[cfg(feature = "async-secure")]
    /// Some data for TLS mode
    pub struct TlsCtxAsync {
        pub tls_connector: TlsConnectorAsync,
        pub domain: String,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for TlsCtxAsync {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                TlsCtxAsync {
                    tls_connector: ref __self_0_0,
                    domain: ref __self_0_1,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "TlsCtxAsync");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "tls_connector",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "domain",
                        &&(*__self_0_1),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[cfg(feature = "sync")]
    /// Stream to interface with the FTP server. This interface is only for the command stream.
    pub struct FtpStreamSync {
        reader: BufReaderSync<DataStreamSync>,
        mode: Mode,
        skip450: bool,
        #[cfg(feature = "_with-welcome-msg")]
        welcome_msg: Option<String>,
        #[cfg(feature = "_secure")]
        tls_ctx: Option<TlsCtxSync>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for FtpStreamSync {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                FtpStreamSync {
                    reader: ref __self_0_0,
                    mode: ref __self_0_1,
                    skip450: ref __self_0_2,
                    welcome_msg: ref __self_0_3,
                    tls_ctx: ref __self_0_4,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "FtpStreamSync");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "reader",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "mode",
                        &&(*__self_0_1),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "skip450",
                        &&(*__self_0_2),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "welcome_msg",
                        &&(*__self_0_3),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "tls_ctx",
                        &&(*__self_0_4),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[cfg(feature = "async")]
    /// Stream to interface with the FTP server. This interface is only for the command stream.
    pub struct FtpStreamAsync {
        reader: BufReaderAsync<DataStreamAsync>,
        mode: Mode,
        skip450: bool,
        #[cfg(feature = "_with-welcome-msg")]
        welcome_msg: Option<String>,
        #[cfg(feature = "_secure")]
        tls_ctx: Option<TlsCtxAsync>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for FtpStreamAsync {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                FtpStreamAsync {
                    reader: ref __self_0_0,
                    mode: ref __self_0_1,
                    skip450: ref __self_0_2,
                    welcome_msg: ref __self_0_3,
                    tls_ctx: ref __self_0_4,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "FtpStreamAsync");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "reader",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "mode",
                        &&(*__self_0_1),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "skip450",
                        &&(*__self_0_2),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "welcome_msg",
                        &&(*__self_0_3),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "tls_ctx",
                        &&(*__self_0_4),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[cfg(feature = "sync")]
    impl FtpStreamSync {
        /// Creates an FTP Stream.
        pub fn connect<A: ToSocketAddrsSync>(addr: A) -> FtpResult<Self> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["Connecting to server"], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 66u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            let stream = TcpStreamSync::connect(addr)?;
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Established connection with server"],
                            &[],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 69u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            let mut ftp_stream = Self {
                reader: BufReaderSync::new(DataStreamSync::Tcp(stream)),
                mode: Mode::Passive,
                skip450: false,
                #[cfg(feature = "_with-welcome-msg")]
                welcome_msg: None,
                #[cfg(feature = "_secure")]
                tls_ctx: None,
            };
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["Reading server response..."], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 81u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            let response = ftp_stream.read_response_in(&[Status::Ready])?;
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Server READY; response: "],
                            &[::core::fmt::ArgumentV1::new_display(&response.body)],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 84u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            #[cfg(feature = "_with-welcome-msg")]
            {
                ftp_stream.welcome_msg = Some(response.body.into_string());
            }
            Ok(ftp_stream)
        }
        /// Enable active mode for data channel
        pub fn active_mode(mut self) -> Self {
            self.mode = Mode::Active;
            self
        }
        /// Set the data channel transfer mode
        pub fn set_mode(&mut self, mode: Mode) {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Changed mode to "],
                            &[::core::fmt::ArgumentV1::new_debug(&mode)],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 101u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            self.mode = mode;
        }
        /// Switch to a secure mode if possible, using a provided SSL configuration.
        /// This method does nothing if the connect is already secured.
        ///
        /// ## Panics
        ///
        /// Panics if the plain TCP connection cannot be switched to TLS mode.
        ///
        /// ## Example
        ///
        /// ```rust,no_run
        /// # tokio_test::block_on(async {
        /// use suppaftp::FtpStream;
        /// use suppaftp::async_native_tls::{TlsConnector, TlsStream};
        /// use std::path::Path;
        ///
        /// // Create a TlsConnector
        /// // NOTE: For custom options see <https://docs.rs/native-tls/0.2.6/native_tls/struct.TlsConnectorBuilder.html>
        /// let mut ctx = TlsConnector::new();
        /// let mut ftp_stream = FtpStream::connect("ftp.server.local:21").await.unwrap();
        /// let mut ftp_stream = ftp_stream.into_secure(ctx, "localhost").await.unwrap();
        /// # });
        /// ```
        #[cfg(feature = "_secure")]
        pub fn into_secure(
            mut self,
            tls_connector: TlsConnectorSync,
            domain: &str,
        ) -> FtpResult<Self> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["Initializing TLS auth"], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 134u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            self.command(Command::Auth, &[Status::AuthOk])?;
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["TLS OK; initializing TLS stream"], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 136u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            let stream =
                tls_connector.connect(domain, self.reader.into_inner().into_tcp_stream())?;
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["TLS stream OK"], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 142u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            let mut secured_ftp_stream = Self {
                reader: BufReaderSync::new(DataStreamSync::Ssl(stream.into())),
                mode: self.mode,
                skip450: false,
                tls_ctx: Some(TlsCtxSync {
                    tls_connector,
                    domain: domain.into(),
                }),
                #[cfg(feature = "_with-welcome-msg")]
                welcome_msg: self.welcome_msg,
            };
            secured_ftp_stream.command(Command::Pbsz(0), &[Status::CommandOk])?;
            secured_ftp_stream.command(
                Command::Prot(ProtectionLevel::Private),
                &[Status::CommandOk],
            )?;
            Ok(secured_ftp_stream)
        }
        /// Returns welcome message retrieved from server (if available)
        #[cfg(feature = "_with-welcome-msg")]
        pub fn get_welcome_msg(&self) -> Option<&str> {
            self.welcome_msg.as_deref()
        }
        /// Returns a reference to the underlying TcpStream.
        pub fn get_ref(&self) -> &TcpStreamSync {
            self.reader.get_ref().get_ref()
        }
        /// Log in to the FTP server.
        pub fn login<S: AsRef<str>>(&mut self, user: S, password: S) -> FtpResult<()> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Signin in with user \'", "\'"],
                            &[::core::fmt::ArgumentV1::new_display(&user.as_ref())],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 174u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            let user_response = self.command(
                Command::new_user(user),
                &[Status::LoggedIn, Status::NeedPassword],
            )?;
            if user_response.status == Status::NeedPassword {
                {
                    let lvl = ::log::Level::Debug;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api_log(
                            ::core::fmt::Arguments::new_v1(&["Password is required"], &[]),
                            lvl,
                            &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 178u32),
                            ::log::__private_api::Option::None,
                        );
                    }
                };
                self.command(Command::new_pass(password), &[Status::LoggedIn])?;
            }
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["Login OK"], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 182u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            Ok(())
        }
        /// Perform clear command channel (CCC).
        /// Once the command is performed, the command channel will be encrypted no more.
        /// The data stream will still be secure.
        #[cfg(feature = "_secure")]
        pub fn clear_command_channel(mut self) -> FtpResult<Self> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["performing clear command channel"], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 192u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            self.command(Command::ClearCommandChannel, &[Status::CommandOk])?;
            {
                let lvl = ::log::Level::Trace;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["CCC OK"], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 194u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            self.reader = BufReaderSync::new(DataStreamSync::Tcp(
                self.reader.into_inner().into_tcp_stream(),
            ));
            Ok(self)
        }
        /// Change the current directory to the path specified.
        pub fn cwd<S: AsRef<str>>(&mut self, path: S) -> FtpResult<()> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Changing working directory to "],
                            &[::core::fmt::ArgumentV1::new_display(&path.as_ref())],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 201u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            self.command(Command::new_cwd(path), &[Status::RequestedFileActionOk])?;
            Ok(())
        }
        /// Move the current directory to the parent directory.
        pub fn cdup(&mut self) -> FtpResult<()> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["Going to parent directory"], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 208u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            self.command(
                Command::Cdup,
                &[Status::CommandOk, Status::RequestedFileActionOk],
            )?;
            Ok(())
        }
        /// Gets the current directory
        pub fn pwd(&mut self) -> FtpResult<String> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["Getting working directory"], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 215u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            let response = self.command(Command::Pwd, &[Status::PathCreated])?;
            let body = response.body_as_inline_result()?;
            match (body.find('"'), body.rfind('"')) {
                (Some(begin), Some(end)) if begin < end => Ok(body[begin + 1..end].to_string()),
                _ => Err(FtpError::UnexpectedResponse(response)),
            }
        }
        /// This does nothing. This is usually just used to keep the connection open.
        pub fn noop(&mut self) -> FtpResult<()> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["Pinging server"], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 227u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            self.command(Command::Noop, &[Status::CommandOk])?;
            Ok(())
        }
        /// This creates a new directory on the server.
        pub fn mkdir<S: AsRef<str>>(&mut self, pathname: S) -> FtpResult<()> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Creating directory at "],
                            &[::core::fmt::ArgumentV1::new_display(&pathname.as_ref())],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 234u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            self.command(Command::new_mkd(pathname), &[Status::PathCreated])?;
            Ok(())
        }
        /// Sets the type of file to be transferred. That is the implementation
        /// of `TYPE` command.
        pub fn transfer_type(&mut self, file_type: FileType) -> FtpResult<()> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Setting transfer type "],
                            &[::core::fmt::ArgumentV1::new_display(&file_type.to_string())],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 242u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            self.command(Command::Type(file_type), &[Status::CommandOk])?;
            Ok(())
        }
        /// Quits the current FTP session.
        pub fn quit(mut self) -> FtpResult<()> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["Quitting stream"], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 249u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            self.command(Command::Quit, &[Status::Closing])?;
            Ok(())
        }
        /// Renames the file from_name to to_name
        pub fn rename<S: AsRef<str>>(&mut self, from_name: S, to_name: S) -> FtpResult<()> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Renaming \'", "\' to \'", "\'"],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&from_name.as_ref()),
                                ::core::fmt::ArgumentV1::new_display(&to_name.as_ref()),
                            ],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 256u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            self.command(
                Command::new_rename_from(from_name),
                &[Status::RequestFilePending],
            )?;
            self.command(
                Command::new_rename_to(to_name),
                &[Status::RequestedFileActionOk],
            )?;
            Ok(())
        }
        /// The implementation of `RETR` command where `filename` is the name of the file
        /// to download from FTP and `reader` is the function which operates with the
        /// data stream opened.
        pub fn retr<S, F, T>(&mut self, file_name: S, mut reader: F) -> FtpResult<T>
        where
            F: FnMut(&mut (dyn ReadSync + std::marker::Unpin)) -> FtpResult<T>,
            S: AsRef<str>,
        {
            let mut stream = self.retr_as_stream(file_name)?;
            let result = reader(&mut stream)?;
            self.finalize_retr_stream(stream)?;
            Ok(result)
        }
        /// Retrieves the file name specified from the server as a readable stream.
        /// This method is a more complicated way to retrieve a file.
        /// The reader returned should be dropped.
        /// Also you will have to read the response to make sure it has the correct value.
        /// Once file has been read, call `finalize_retr_stream()`
        pub fn retr_as_stream<S: AsRef<str>>(&mut self, file_name: S) -> FtpResult<DataStreamSync> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Retrieving \'", "\'"],
                            &[::core::fmt::ArgumentV1::new_display(&file_name.as_ref())],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 295u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            let data_stream = self.data_command(Command::new_retr(file_name))?;
            self.read_response_in(&[Status::AboutToSend, Status::AlreadyOpen])?;
            Ok(data_stream)
        }
        /// Finalize retr stream; must be called once the requested file, got previously with `retr_as_stream()` has been read
        pub fn finalize_retr_stream(&mut self, stream: impl ReadSync) -> FtpResult<()> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["Finalizing retr stream"], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 303u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            drop(stream);
            {
                let lvl = ::log::Level::Trace;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["dropped stream"], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 306u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            self.read_response_in(&[Status::ClosingDataConnection, Status::RequestedFileActionOk])?;
            Ok(())
        }
        /// Removes the remote pathname from the server.
        pub fn rmdir<S: AsRef<str>>(&mut self, pathname: S) -> FtpResult<()> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Removing directory "],
                            &[::core::fmt::ArgumentV1::new_display(&pathname.as_ref())],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 314u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            self.command(Command::new_rmd(pathname), &[Status::RequestedFileActionOk])?;
            Ok(())
        }
        /// Remove the remote file from the server.
        pub fn rm<S: AsRef<str>>(&mut self, filename: S) -> FtpResult<()> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Removing file "],
                            &[::core::fmt::ArgumentV1::new_display(&filename.as_ref())],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 321u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            self.command(
                Command::new_dele(filename),
                &[Status::RequestedFileActionOk],
            )?;
            Ok(())
        }
        /// This stores a file on the server.
        /// r argument must be any struct which implemenents the Read trait.
        /// Returns amount of written bytes
        pub fn put_file<S, R>(&mut self, filename: S, r: &mut R) -> FtpResult<u64>
        where
            R: ReadSync + std::marker::Unpin,
            S: AsRef<str>,
        {
            let mut data_stream = self.put_with_stream(filename)?;
            let bytes = copy_sync(r, &mut data_stream)?;
            self.finalize_put_stream(data_stream)?;
            Ok(bytes)
        }
        /// Send PUT command and returns a BufWriter, which references the file created on the server
        /// The returned stream must be then correctly manipulated to write the content of the source file to the remote destination
        /// The stream must be then correctly dropped.
        /// Once you've finished the write, YOU MUST CALL THIS METHOD: `finalize_put_stream`
        pub fn put_with_stream<S: AsRef<str>>(&mut self, filename: S) -> FtpResult<DataStreamSync> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Put file "],
                            &[::core::fmt::ArgumentV1::new_display(&filename.as_ref())],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 348u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            let data_stream = self.data_command(Command::new_store(filename))?;
            self.read_response_in(&[Status::AlreadyOpen, Status::AboutToSend])?;
            Ok(data_stream)
        }
        /// Finalize put when using stream
        /// This method must be called once the file has been written and
        /// `put_with_stream` has been used to write the file
        pub fn finalize_put_stream(&mut self, stream: impl WriteSync) -> FtpResult<()> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["Finalizing put stream"], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 358u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            drop(stream);
            {
                let lvl = ::log::Level::Trace;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["Stream dropped"], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 361u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            self.read_response_in(&[Status::ClosingDataConnection, Status::RequestedFileActionOk])?;
            Ok(())
        }
        /// Open specified file for appending data. Returns the stream to append data to specified file.
        /// Once you've finished the write, YOU MUST CALL THIS METHOD: `finalize_put_stream`
        pub fn append_with_stream<S: AsRef<str>>(
            &mut self,
            filename: S,
        ) -> FtpResult<DataStreamSync> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Appending to file "],
                            &[::core::fmt::ArgumentV1::new_display(&filename.as_ref())],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 370u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            let stream = self.data_command(Command::Appe(filename.as_ref().to_string()))?;
            self.read_response_in(&[Status::AlreadyOpen, Status::AboutToSend])?;
            Ok(stream)
        }
        /// Append data from reader to file at `filename`
        pub fn append_file<R>(&mut self, filename: &str, r: &mut R) -> FtpResult<u64>
        where
            R: ReadSync + std::marker::Unpin,
        {
            let mut data_stream = self.append_with_stream(filename)?;
            let bytes = copy_sync(r, &mut data_stream)?;
            self.finalize_put_stream(Box::new(data_stream))?;
            Ok(bytes)
        }
        /// abort the previous FTP service command
        pub fn abort<R>(&mut self, data_stream: R) -> FtpResult<()>
        where
            R: ReadSync + std::marker::Unpin,
        {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["Aborting active file transfer"], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 395u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            self.perform(Command::Abor)?;
            drop(data_stream);
            {
                let lvl = ::log::Level::Trace;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["dropped stream"], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 399u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            self.read_response_in(&[Status::ClosingDataConnection, Status::TransferAborted])?;
            self.read_response(Status::ClosingDataConnection)?;
            self.skip450 = true;
            {
                let lvl = ::log::Level::Trace;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["Transfer aborted"], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 406u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            Ok(())
        }
        /// Tell the server to resume the transfer from a certain offset. The offset indicates the amount of bytes to skip
        /// from the beginning of the file.
        /// the REST command does not actually initiate the transfer.
        /// After issuing a REST command, the client must send the appropriate FTP command to transfer the file
        ///
        /// It is possible to cancel the REST command, sending a REST command with offset 0
        pub fn resume_transfer(&mut self, offset: usize) -> FtpResult<()> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Requesting to resume transfer at offset "],
                            &[::core::fmt::ArgumentV1::new_display(&offset)],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 417u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            self.command(Command::Rest(offset), &[Status::RequestFilePending])?;
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["Resume transfer accepted"], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 419u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            Ok(())
        }
        /// Execute `LIST` command which returns the detailed file listing in human readable format.
        /// If `pathname` is omited then the list of files in the current directory will be
        /// returned otherwise it will the list of files on `pathname`.
        pub fn list(&mut self, pathname: Option<&str>) -> FtpResult<Vec<String>> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Reading ", " directory content"],
                            &[::core::fmt::ArgumentV1::new_display(
                                &pathname.unwrap_or("working"),
                            )],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 427u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            self.stream_lines(Command::new_list(pathname))
        }
        /// Execute `NLST` command which returns the list of file names only.
        /// If `pathname` is omited then the list of files in the current directory will be
        /// returned otherwise it will the list of files on `pathname`.
        pub fn nlst(&mut self, pathname: Option<&str>) -> FtpResult<Vec<String>> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Getting file names for ", " directory"],
                            &[::core::fmt::ArgumentV1::new_display(
                                &pathname.unwrap_or("working"),
                            )],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 439u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            self.stream_lines(Command::new_nlst(pathname))
        }
        /// Execute `MLST` command which returns the list of file names only.
        /// If `pathname` is omited then the list of files in the current directory will be
        /// returned otherwise it will the list of files on `pathname`.
        pub fn mlst(&mut self, pathname: Option<&str>) -> FtpResult<String> {
            let response = self.command(
                Command::new_mlst(pathname),
                &[Status::RequestedFileActionOk],
            )?;
            let mut lines = response.body_into_multiline_result()?;
            if lines.len() != 1 {
                return Err(FtpError::BadResponse);
            }
            let line = lines.pop().unwrap();
            Ok(line)
        }
        /// ### mlsd
        ///
        /// Execute `MLSD` command which returns the list of file names only.
        /// If `pathname` is omited then the list of files in the current directory will be
        /// returned otherwise it will the list of files on `pathname`.
        pub fn mlsd(&mut self, pathname: Option<&str>) -> FtpResult<Vec<String>> {
            self.stream_lines(Command::new_mlsd(pathname))
        }
        /// Retrieves the modification time of the file at `pathname` if it exists.
        pub fn mdtm<S: AsRef<str>>(&mut self, pathname: S) -> FtpResult<DateTime<Utc>> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Getting modification time for "],
                            &[::core::fmt::ArgumentV1::new_display(&pathname.as_ref())],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 472u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            let response = self.command(Command::new_mdtm(pathname), &[Status::File])?;
            let line = response.body_as_inline_result()?;
            match MDTM_RE.captures(line) {
                Some(caps) => {
                    let (year, month, day) = (
                        caps[1].parse::<i32>().unwrap(),
                        caps[2].parse::<u32>().unwrap(),
                        caps[3].parse::<u32>().unwrap(),
                    );
                    let (hour, minute, second) = (
                        caps[4].parse::<u32>().unwrap(),
                        caps[5].parse::<u32>().unwrap(),
                        caps[6].parse::<u32>().unwrap(),
                    );
                    Ok(Utc.ymd(year, month, day).and_hms(hour, minute, second))
                }
                None => Err(FtpError::BadResponse),
            }
        }
        /// Requests the server to list all extension commands, or extended mechanisms, that it supports.
        pub fn feat(&mut self) -> FtpResult<Vec<String>> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["Feat"], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 496u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            let response = self.command(Command::Feat, &[Status::System])?;
            Ok(response.body.into_vec())
        }
        /// Requests the server to list all extension commands, or extended mechanisms, that it supports.
        pub fn opts<S: AsRef<str>>(&mut self, cmd: S, cmd_options: Option<S>) -> FtpResult<String> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Opts \'", "\' \'", "\'"],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&cmd.as_ref()),
                                ::core::fmt::ArgumentV1::new_display(&optstrref(&cmd_options)),
                            ],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 503u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            let response =
                self.command(Command::new_opts(cmd, cmd_options), &[Status::CommandOk])?;
            response.body_into_inline_result()
        }
        /// Requests the server to list all extension commands, or extended mechanisms, that it supports.
        pub fn lang<S: AsRef<str>>(&mut self, lang_tag: Option<S>) -> FtpResult<String> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Lang \'", "\'"],
                            &[::core::fmt::ArgumentV1::new_display(&optstrref(&lang_tag))],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 510u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            let response = self.command(Command::new_lang(lang_tag), &[Status::CommandOk])?;
            response.body_into_inline_result()
        }
        /// Retrieves the size of the file in bytes at `pathname` if it exists.
        pub fn site<S: AsRef<str>>(&mut self, cmd: S) -> FtpResult<String> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["SITE \'", "\'"],
                            &[::core::fmt::ArgumentV1::new_display(&cmd.as_ref())],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 517u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            let response = self.command(Command::new_site(cmd), &[Status::CommandOk])?;
            response.body_into_inline_result()
        }
        /// Returns information on the server status, including the status of the current connection
        pub fn stat<S: AsRef<str>>(&mut self, path: Option<S>) -> FtpResult<Vec<String>> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Stat \'", "\'"],
                            &[::core::fmt::ArgumentV1::new_display(&optstrref(&path))],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 524u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            let response = self.command(
                Command::new_stat(path),
                &[Status::System, Status::Directory, Status::File],
            )?;
            Ok(response.body.into_vec())
        }
        /// Retrieves the size of the file in bytes at `pathname` if it exists.
        pub fn size<S: AsRef<str>>(&mut self, pathname: S) -> FtpResult<usize> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Getting file size for "],
                            &[::core::fmt::ArgumentV1::new_display(&pathname.as_ref())],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 531u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            let response = self.command(Command::new_size(pathname), &[Status::File])?;
            let line = response.body_as_inline_result()?;
            match SIZE_RE.captures(line) {
                Some(caps) => Ok(caps[1].parse().unwrap()),
                None => Err(FtpError::BadResponse),
            }
        }
        /// Retrieve stream "message"
        fn get_lines_from_stream(
            data_stream: &mut BufReaderSync<DataStreamSync>,
        ) -> FtpResult<Vec<String>> {
            let mut lines: Vec<String> = Vec::new();
            loop {
                let mut line = String::new();
                match data_stream.read_line(&mut line) {
                    Ok(0) => break,
                    Ok(_) => {
                        if line.ends_with('\n') {
                            line.pop();
                            if line.ends_with('\r') {
                                line.pop();
                            }
                        }
                        if line.is_empty() {
                            continue;
                        }
                        lines.push(line);
                    }
                    Err(_) => return Err(FtpError::BadResponse),
                }
            }
            {
                let lvl = ::log::Level::Trace;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Lines from stream "],
                            &[::core::fmt::ArgumentV1::new_debug(&lines)],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 566u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            Ok(lines)
        }
        /// Read response from stream
        fn read_response(&mut self, expected_code: Status) -> FtpResult<Response> {
            self.read_response_in(&[expected_code])
        }
        /// Retrieve single line response
        fn read_response_in(&mut self, expected_status: &[Status]) -> FtpResult<Response> {
            let mut line_buffer = String::new();
            let mut line = self.read_line(&mut line_buffer)?;
            {
                let lvl = ::log::Level::Trace;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["CC IN: "],
                            &[::core::fmt::ArgumentV1::new_display(&line.trim_end())],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 581u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            if line.len() < CODE_LENGTH + 1 {
                return Err(FtpError::BadResponse);
            }
            let (mut status, mut delim, mut head) = parse_status_delim_tail(line)?;
            if self.skip450 {
                self.skip450 = false;
                if status == Status::RequestFileActionIgnored {
                    line = self.read_line(&mut line_buffer)?;
                    {
                        let lvl = ::log::Level::Trace;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api_log(
                                ::core::fmt::Arguments::new_v1(
                                    &["CC IN: "],
                                    &[::core::fmt::ArgumentV1::new_display(&line.trim_end())],
                                ),
                                lvl,
                                &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 597u32),
                                ::log::__private_api::Option::None,
                            );
                        }
                    };
                    if line.len() < CODE_LENGTH + 1 {
                        return Err(FtpError::BadResponse);
                    }
                    (status, delim, head) = parse_status_delim_tail(line)?;
                }
            }
            let response = match delim {
                SPACE_CHAR => Response::new_inline(status, head),
                MINUS_CHAR => {
                    let mut body: Vec<String> = ::alloc::vec::Vec::new();
                    loop {
                        line = self.read_line(&mut line_buffer)?;
                        {
                            let lvl = ::log::Level::Trace;
                            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                                ::log::__private_api_log(
                                    ::core::fmt::Arguments::new_v1(
                                        &["CC IN: "],
                                        &[::core::fmt::ArgumentV1::new_display(&line)],
                                    ),
                                    lvl,
                                    &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 618u32),
                                    ::log::__private_api::Option::None,
                                );
                            }
                        };
                        let first_char =
                            line.chars().nth(0).ok_or_else(|| FtpError::BadResponse)?;
                        match first_char {
                            SPACE_CHAR => body.push(line[1..].to_string()),
                            ch if ch.is_ascii_digit() => {
                                let (status2, delim, tail) = parse_status_delim_tail(line)?;
                                if status2 != status || delim != SPACE_CHAR {
                                    return Err(FtpError::BadResponse);
                                };
                                break Response::new_multiline(status, head, body, tail);
                            }
                            _ => {
                                return Err(FtpError::BadResponse);
                            }
                        }
                    }
                }
                _ => {
                    return Err(FtpError::BadResponse);
                }
            };
            if expected_status.contains(&status) {
                Ok(response)
            } else {
                let err = match status {
                    Status::BadCommand | Status::NotImplemented | Status::BadSequence => {
                        FtpError::BadCommand {
                            status,
                            message: response.body_into_inline_result()?,
                        }
                    }
                    Status::BadArguments | Status::NotImplementedParameter => {
                        FtpError::BadParameter {
                            status,
                            message: response.body_into_inline_result()?,
                        }
                    }
                    _ => FtpError::UnexpectedResponse(response),
                };
                Err(err)
            }
        }
        /// Write data to stream with command to perform
        fn perform(&mut self, command: Command) -> FtpResult<()> {
            let command = command.to_string();
            {
                let lvl = ::log::Level::Trace;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["CC OUT: "],
                            &[::core::fmt::ArgumentV1::new_display(
                                &command.trim_end_matches("\r\n"),
                            )],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 666u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            let stream = self.reader.get_mut();
            stream.write_all(command.as_bytes())?;
            Ok(())
        }
        /// Execute command which send data back in a command stream
        pub fn command(
            &mut self,
            command: Command,
            expected_code: &[Status],
        ) -> FtpResult<Response> {
            self.perform(command)?;
            self.read_response_in(expected_code)
        }
        /// Execute command which send data back in a separate stream
        fn data_command(&mut self, cmd: Command) -> FtpResult<DataStreamSync> {
            let stream = match self.mode {
                Mode::Passive => {
                    let addr = self.pasv()?;
                    self.perform(cmd)?;
                    TcpStreamSync::connect(addr)?
                }
                Mode::Active => {
                    let listener = self.active()?;
                    self.perform(cmd)?;
                    let (stream, _) = listener.accept()?;
                    stream
                }
            };
            #[cfg(feature = "_secure")]
            {
                match self.tls_ctx {
                    Some(ref tls_ctx) => {
                        let tls_stream = tls_ctx
                            .tls_connector
                            .connect(tls_ctx.domain.as_str(), stream)?;
                        Ok(DataStreamSync::Ssl(tls_stream.into()))
                    }
                    None => Ok(DataStreamSync::Tcp(stream)),
                }
            }
        }
        /// Create a new tcp listener and send a PORT command for it
        fn active(&mut self) -> FtpResult<TcpListenerSync> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["Starting local tcp listener..."], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 717u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            let listener = TcpListenerSync::bind("0.0.0.0:0")?;
            let addr = listener.local_addr()?;
            {
                let lvl = ::log::Level::Trace;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Local address is "],
                            &[::core::fmt::ArgumentV1::new_display(&addr)],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 721u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            let tcp_stream = match self.reader.get_ref() {
                DataStreamSync::Tcp(stream) => stream,
                #[cfg(feature = "_secure")]
                DataStreamSync::Ssl(stream) => stream.get_ref(),
            };
            let ip = tcp_stream.local_addr().unwrap().ip();
            let msb = addr.port() / 256;
            let lsb = addr.port() % 256;
            let ip_port = {
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["", ",", ","],
                    &[
                        ::core::fmt::ArgumentV1::new_display(&ip.to_string().replace(".", ",")),
                        ::core::fmt::ArgumentV1::new_display(&msb),
                        ::core::fmt::ArgumentV1::new_display(&lsb),
                    ],
                ));
                res
            };
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Active mode, listening on ", ":"],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&ip),
                                ::core::fmt::ArgumentV1::new_display(&addr.port()),
                            ],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 734u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["Running PORT command"], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 736u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            self.command(Command::Port(ip_port), &[Status::CommandOk])?;
            Ok(listener)
        }
        /// Runs the PASV command.
        fn pasv(&mut self) -> FtpResult<SocketAddrSync> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["PASV command"], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 744u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            let response = self.command(Command::Pasv, &[Status::PassiveMode])?;
            let body = response.body_as_inline_result()?;
            let caps = PORT_RE
                .captures(body)
                .ok_or_else(|| FtpError::UnexpectedResponse(response.clone()))?;
            let (oct1, oct2, oct3, oct4) = (
                caps[1].parse::<u8>().unwrap(),
                caps[2].parse::<u8>().unwrap(),
                caps[3].parse::<u8>().unwrap(),
                caps[4].parse::<u8>().unwrap(),
            );
            let (msb, lsb) = (
                caps[5].parse::<u8>().unwrap(),
                caps[6].parse::<u8>().unwrap(),
            );
            let port = ((msb as u16) << 8) + lsb as u16;
            let addr = {
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["", ".", ".", ".", ":"],
                    &[
                        ::core::fmt::ArgumentV1::new_display(&oct1),
                        ::core::fmt::ArgumentV1::new_display(&oct2),
                        ::core::fmt::ArgumentV1::new_display(&oct3),
                        ::core::fmt::ArgumentV1::new_display(&oct4),
                        ::core::fmt::ArgumentV1::new_display(&port),
                    ],
                ));
                res
            };
            {
                let lvl = ::log::Level::Trace;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Passive address: "],
                            &[::core::fmt::ArgumentV1::new_display(&addr)],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 765u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            let addr = SocketAddrSync::from_str(&addr)?;
            Ok(addr)
        }
        fn read_line<'s>(&mut self, line_buffer: &'s mut String) -> FtpResult<&'s str> {
            line_buffer.clear();
            match self.reader.read_line(line_buffer) {
                Ok(size) => {
                    if size == 0 {
                        {
                            let lvl = ::log::Level::Debug;
                            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                                ::log::__private_api_log(
                                    ::core::fmt::Arguments::new_v1(&["ERR read_line: EOF"], &[]),
                                    lvl,
                                    &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 778u32),
                                    ::log::__private_api::Option::None,
                                );
                            }
                        };
                        return Err(FtpError::BadResponse);
                    }
                }
                Err(e) => {
                    {
                        let lvl = ::log::Level::Debug;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api_log(
                                ::core::fmt::Arguments::new_v1(
                                    &["ERR read_line: "],
                                    &[::core::fmt::ArgumentV1::new_debug(&e)],
                                ),
                                lvl,
                                &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 783u32),
                                ::log::__private_api::Option::None,
                            );
                        }
                    };
                    return Err(e.into());
                }
            };
            let line = line_buffer.trim_end_matches(|ch| ch == '\r' || ch == '\n');
            Ok(line)
        }
        /// Execute a command which returns list of strings in a separate stream
        fn stream_lines(&mut self, cmd: Command) -> FtpResult<Vec<String>> {
            let mut data_stream = BufReaderSync::new(self.data_command(cmd)?);
            self.read_response_in(&[Status::AboutToSend, Status::AlreadyOpen])?;
            let lines = Self::get_lines_from_stream(&mut data_stream);
            self.finalize_retr_stream(data_stream)?;
            lines
        }
    }
    #[cfg(feature = "async")]
    impl FtpStreamAsync {
        /// Creates an FTP Stream.
        pub async fn connect<A: ToSocketAddrsAsync>(addr: A) -> FtpResult<Self> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["Connecting to server"], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 66u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            let stream = TcpStreamAsync::connect(addr).await?;
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Established connection with server"],
                            &[],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 69u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            let mut ftp_stream = Self {
                reader: BufReaderAsync::new(DataStreamAsync::Tcp(stream)),
                mode: Mode::Passive,
                skip450: false,
                #[cfg(feature = "_with-welcome-msg")]
                welcome_msg: None,
                #[cfg(feature = "_secure")]
                tls_ctx: None,
            };
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["Reading server response..."], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 81u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            let response = ftp_stream.read_response_in(&[Status::Ready]).await?;
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Server READY; response: "],
                            &[::core::fmt::ArgumentV1::new_display(&response.body)],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 84u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            #[cfg(feature = "_with-welcome-msg")]
            {
                ftp_stream.welcome_msg = Some(response.body.into_string());
            }
            Ok(ftp_stream)
        }
        /// Enable active mode for data channel
        pub fn active_mode(mut self) -> Self {
            self.mode = Mode::Active;
            self
        }
        /// Set the data channel transfer mode
        pub fn set_mode(&mut self, mode: Mode) {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Changed mode to "],
                            &[::core::fmt::ArgumentV1::new_debug(&mode)],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 101u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            self.mode = mode;
        }
        /// Switch to a secure mode if possible, using a provided SSL configuration.
        /// This method does nothing if the connect is already secured.
        ///
        /// ## Panics
        ///
        /// Panics if the plain TCP connection cannot be switched to TLS mode.
        ///
        /// ## Example
        ///
        /// ```rust,no_run
        /// # tokio_test::block_on(async {
        /// use suppaftp::FtpStream;
        /// use suppaftp::async_native_tls::{TlsConnector, TlsStream};
        /// use std::path::Path;
        ///
        /// // Create a TlsConnector
        /// // NOTE: For custom options see <https://docs.rs/native-tls/0.2.6/native_tls/struct.TlsConnectorBuilder.html>
        /// let mut ctx = TlsConnector::new();
        /// let mut ftp_stream = FtpStream::connect("ftp.server.local:21").await.unwrap();
        /// let mut ftp_stream = ftp_stream.into_secure(ctx, "localhost").await.unwrap();
        /// # });
        /// ```
        #[cfg(feature = "_secure")]
        pub async fn into_secure(
            mut self,
            tls_connector: TlsConnectorAsync,
            domain: &str,
        ) -> FtpResult<Self> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["Initializing TLS auth"], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 134u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            self.command(Command::Auth, &[Status::AuthOk]).await?;
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["TLS OK; initializing TLS stream"], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 136u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            let stream = tls_connector
                .connect(domain, self.reader.into_inner().into_tcp_stream())
                .await?;
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["TLS stream OK"], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 142u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            let mut secured_ftp_stream = Self {
                reader: BufReaderAsync::new(DataStreamAsync::Ssl(stream.into())),
                mode: self.mode,
                skip450: false,
                tls_ctx: Some(TlsCtxAsync {
                    tls_connector,
                    domain: domain.into(),
                }),
                #[cfg(feature = "_with-welcome-msg")]
                welcome_msg: self.welcome_msg,
            };
            secured_ftp_stream
                .command(Command::Pbsz(0), &[Status::CommandOk])
                .await?;
            secured_ftp_stream
                .command(
                    Command::Prot(ProtectionLevel::Private),
                    &[Status::CommandOk],
                )
                .await?;
            Ok(secured_ftp_stream)
        }
        /// Returns welcome message retrieved from server (if available)
        #[cfg(feature = "_with-welcome-msg")]
        pub fn get_welcome_msg(&self) -> Option<&str> {
            self.welcome_msg.as_deref()
        }
        /// Returns a reference to the underlying TcpStream.
        pub async fn get_ref(&self) -> &TcpStreamAsync {
            self.reader.get_ref().get_ref()
        }
        /// Log in to the FTP server.
        pub async fn login<S: AsRef<str>>(&mut self, user: S, password: S) -> FtpResult<()> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Signin in with user \'", "\'"],
                            &[::core::fmt::ArgumentV1::new_display(&user.as_ref())],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 174u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            let user_response = self
                .command(
                    Command::new_user(user),
                    &[Status::LoggedIn, Status::NeedPassword],
                )
                .await?;
            if user_response.status == Status::NeedPassword {
                {
                    let lvl = ::log::Level::Debug;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api_log(
                            ::core::fmt::Arguments::new_v1(&["Password is required"], &[]),
                            lvl,
                            &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 178u32),
                            ::log::__private_api::Option::None,
                        );
                    }
                };
                self.command(Command::new_pass(password), &[Status::LoggedIn])
                    .await?;
            }
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["Login OK"], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 182u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            Ok(())
        }
        /// Perform clear command channel (CCC).
        /// Once the command is performed, the command channel will be encrypted no more.
        /// The data stream will still be secure.
        #[cfg(feature = "_secure")]
        pub async fn clear_command_channel(mut self) -> FtpResult<Self> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["performing clear command channel"], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 192u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            self.command(Command::ClearCommandChannel, &[Status::CommandOk])
                .await?;
            {
                let lvl = ::log::Level::Trace;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["CCC OK"], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 194u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            self.reader = BufReaderAsync::new(DataStreamAsync::Tcp(
                self.reader.into_inner().into_tcp_stream(),
            ));
            Ok(self)
        }
        /// Change the current directory to the path specified.
        pub async fn cwd<S: AsRef<str>>(&mut self, path: S) -> FtpResult<()> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Changing working directory to "],
                            &[::core::fmt::ArgumentV1::new_display(&path.as_ref())],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 201u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            self.command(Command::new_cwd(path), &[Status::RequestedFileActionOk])
                .await?;
            Ok(())
        }
        /// Move the current directory to the parent directory.
        pub async fn cdup(&mut self) -> FtpResult<()> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["Going to parent directory"], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 208u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            self.command(
                Command::Cdup,
                &[Status::CommandOk, Status::RequestedFileActionOk],
            )
            .await?;
            Ok(())
        }
        /// Gets the current directory
        pub async fn pwd(&mut self) -> FtpResult<String> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["Getting working directory"], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 215u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            let response = self.command(Command::Pwd, &[Status::PathCreated]).await?;
            let body = response.body_as_inline_result()?;
            match (body.find('"'), body.rfind('"')) {
                (Some(begin), Some(end)) if begin < end => Ok(body[begin + 1..end].to_string()),
                _ => Err(FtpError::UnexpectedResponse(response)),
            }
        }
        /// This does nothing. This is usually just used to keep the connection open.
        pub async fn noop(&mut self) -> FtpResult<()> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["Pinging server"], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 227u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            self.command(Command::Noop, &[Status::CommandOk]).await?;
            Ok(())
        }
        /// This creates a new directory on the server.
        pub async fn mkdir<S: AsRef<str>>(&mut self, pathname: S) -> FtpResult<()> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Creating directory at "],
                            &[::core::fmt::ArgumentV1::new_display(&pathname.as_ref())],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 234u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            self.command(Command::new_mkd(pathname), &[Status::PathCreated])
                .await?;
            Ok(())
        }
        /// Sets the type of file to be transferred. That is the implementation
        /// of `TYPE` command.
        pub async fn transfer_type(&mut self, file_type: FileType) -> FtpResult<()> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Setting transfer type "],
                            &[::core::fmt::ArgumentV1::new_display(&file_type.to_string())],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 242u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            self.command(Command::Type(file_type), &[Status::CommandOk])
                .await?;
            Ok(())
        }
        /// Quits the current FTP session.
        pub async fn quit(mut self) -> FtpResult<()> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["Quitting stream"], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 249u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            self.command(Command::Quit, &[Status::Closing]).await?;
            Ok(())
        }
        /// Renames the file from_name to to_name
        pub async fn rename<S: AsRef<str>>(&mut self, from_name: S, to_name: S) -> FtpResult<()> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Renaming \'", "\' to \'", "\'"],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&from_name.as_ref()),
                                ::core::fmt::ArgumentV1::new_display(&to_name.as_ref()),
                            ],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 256u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            self.command(
                Command::new_rename_from(from_name),
                &[Status::RequestFilePending],
            )
            .await?;
            self.command(
                Command::new_rename_to(to_name),
                &[Status::RequestedFileActionOk],
            )
            .await?;
            Ok(())
        }
        /// The implementation of `RETR` command where `filename` is the name of the file
        /// to download from FTP and `reader` is the function which operates with the
        /// data stream opened.
        pub async fn retr<S, F, T>(&mut self, file_name: S, mut reader: F) -> FtpResult<T>
        where
            F: FnMut(&mut (dyn ReadAsync + std::marker::Unpin)) -> FtpResult<T>,
            S: AsRef<str>,
        {
            let mut stream = self.retr_as_stream(file_name).await?;
            let result = reader(&mut stream)?;
            self.finalize_retr_stream(stream).await?;
            Ok(result)
        }
        /// Retrieves the file name specified from the server as a readable stream.
        /// This method is a more complicated way to retrieve a file.
        /// The reader returned should be dropped.
        /// Also you will have to read the response to make sure it has the correct value.
        /// Once file has been read, call `finalize_retr_stream()`
        pub async fn retr_as_stream<S: AsRef<str>>(
            &mut self,
            file_name: S,
        ) -> FtpResult<DataStreamAsync> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Retrieving \'", "\'"],
                            &[::core::fmt::ArgumentV1::new_display(&file_name.as_ref())],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 295u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            let data_stream = self.data_command(Command::new_retr(file_name)).await?;
            self.read_response_in(&[Status::AboutToSend, Status::AlreadyOpen])
                .await?;
            Ok(data_stream)
        }
        /// Finalize retr stream; must be called once the requested file, got previously with `retr_as_stream()` has been read
        pub async fn finalize_retr_stream(&mut self, stream: impl ReadAsync) -> FtpResult<()> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["Finalizing retr stream"], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 303u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            drop(stream);
            {
                let lvl = ::log::Level::Trace;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["dropped stream"], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 306u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            self.read_response_in(&[Status::ClosingDataConnection, Status::RequestedFileActionOk])
                .await?;
            Ok(())
        }
        /// Removes the remote pathname from the server.
        pub async fn rmdir<S: AsRef<str>>(&mut self, pathname: S) -> FtpResult<()> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Removing directory "],
                            &[::core::fmt::ArgumentV1::new_display(&pathname.as_ref())],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 314u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            self.command(Command::new_rmd(pathname), &[Status::RequestedFileActionOk])
                .await?;
            Ok(())
        }
        /// Remove the remote file from the server.
        pub async fn rm<S: AsRef<str>>(&mut self, filename: S) -> FtpResult<()> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Removing file "],
                            &[::core::fmt::ArgumentV1::new_display(&filename.as_ref())],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 321u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            self.command(
                Command::new_dele(filename),
                &[Status::RequestedFileActionOk],
            )
            .await?;
            Ok(())
        }
        /// This stores a file on the server.
        /// r argument must be any struct which implemenents the Read trait.
        /// Returns amount of written bytes
        pub async fn put_file<S, R>(&mut self, filename: S, r: &mut R) -> FtpResult<u64>
        where
            R: ReadAsync + std::marker::Unpin,
            S: AsRef<str>,
        {
            let mut data_stream = self.put_with_stream(filename).await?;
            let bytes = copy_async(r, &mut data_stream).await?;
            self.finalize_put_stream(data_stream).await?;
            Ok(bytes)
        }
        /// Send PUT command and returns a BufWriter, which references the file created on the server
        /// The returned stream must be then correctly manipulated to write the content of the source file to the remote destination
        /// The stream must be then correctly dropped.
        /// Once you've finished the write, YOU MUST CALL THIS METHOD: `finalize_put_stream`
        pub async fn put_with_stream<S: AsRef<str>>(
            &mut self,
            filename: S,
        ) -> FtpResult<DataStreamAsync> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Put file "],
                            &[::core::fmt::ArgumentV1::new_display(&filename.as_ref())],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 348u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            let data_stream = self.data_command(Command::new_store(filename)).await?;
            self.read_response_in(&[Status::AlreadyOpen, Status::AboutToSend])
                .await?;
            Ok(data_stream)
        }
        /// Finalize put when using stream
        /// This method must be called once the file has been written and
        /// `put_with_stream` has been used to write the file
        pub async fn finalize_put_stream(&mut self, stream: impl WriteAsync) -> FtpResult<()> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["Finalizing put stream"], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 358u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            drop(stream);
            {
                let lvl = ::log::Level::Trace;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["Stream dropped"], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 361u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            self.read_response_in(&[Status::ClosingDataConnection, Status::RequestedFileActionOk])
                .await?;
            Ok(())
        }
        /// Open specified file for appending data. Returns the stream to append data to specified file.
        /// Once you've finished the write, YOU MUST CALL THIS METHOD: `finalize_put_stream`
        pub async fn append_with_stream<S: AsRef<str>>(
            &mut self,
            filename: S,
        ) -> FtpResult<DataStreamAsync> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Appending to file "],
                            &[::core::fmt::ArgumentV1::new_display(&filename.as_ref())],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 370u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            let stream = self
                .data_command(Command::Appe(filename.as_ref().to_string()))
                .await?;
            self.read_response_in(&[Status::AlreadyOpen, Status::AboutToSend])
                .await?;
            Ok(stream)
        }
        /// Append data from reader to file at `filename`
        pub async fn append_file<R>(&mut self, filename: &str, r: &mut R) -> FtpResult<u64>
        where
            R: ReadAsync + std::marker::Unpin,
        {
            let mut data_stream = self.append_with_stream(filename).await?;
            let bytes = copy_async(r, &mut data_stream).await?;
            self.finalize_put_stream(Box::new(data_stream)).await?;
            Ok(bytes)
        }
        /// abort the previous FTP service command
        pub async fn abort<R>(&mut self, data_stream: R) -> FtpResult<()>
        where
            R: ReadAsync + std::marker::Unpin,
        {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["Aborting active file transfer"], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 395u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            self.perform(Command::Abor).await?;
            drop(data_stream);
            {
                let lvl = ::log::Level::Trace;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["dropped stream"], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 399u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            self.read_response_in(&[Status::ClosingDataConnection, Status::TransferAborted])
                .await?;
            self.read_response(Status::ClosingDataConnection).await?;
            self.skip450 = true;
            {
                let lvl = ::log::Level::Trace;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["Transfer aborted"], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 406u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            Ok(())
        }
        /// Tell the server to resume the transfer from a certain offset. The offset indicates the amount of bytes to skip
        /// from the beginning of the file.
        /// the REST command does not actually initiate the transfer.
        /// After issuing a REST command, the client must send the appropriate FTP command to transfer the file
        ///
        /// It is possible to cancel the REST command, sending a REST command with offset 0
        pub async fn resume_transfer(&mut self, offset: usize) -> FtpResult<()> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Requesting to resume transfer at offset "],
                            &[::core::fmt::ArgumentV1::new_display(&offset)],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 417u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            self.command(Command::Rest(offset), &[Status::RequestFilePending])
                .await?;
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["Resume transfer accepted"], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 419u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            Ok(())
        }
        /// Execute `LIST` command which returns the detailed file listing in human readable format.
        /// If `pathname` is omited then the list of files in the current directory will be
        /// returned otherwise it will the list of files on `pathname`.
        pub async fn list(&mut self, pathname: Option<&str>) -> FtpResult<Vec<String>> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Reading ", " directory content"],
                            &[::core::fmt::ArgumentV1::new_display(
                                &pathname.unwrap_or("working"),
                            )],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 427u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            self.stream_lines(Command::new_list(pathname)).await
        }
        /// Execute `NLST` command which returns the list of file names only.
        /// If `pathname` is omited then the list of files in the current directory will be
        /// returned otherwise it will the list of files on `pathname`.
        pub async fn nlst(&mut self, pathname: Option<&str>) -> FtpResult<Vec<String>> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Getting file names for ", " directory"],
                            &[::core::fmt::ArgumentV1::new_display(
                                &pathname.unwrap_or("working"),
                            )],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 439u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            self.stream_lines(Command::new_nlst(pathname)).await
        }
        /// Execute `MLST` command which returns the list of file names only.
        /// If `pathname` is omited then the list of files in the current directory will be
        /// returned otherwise it will the list of files on `pathname`.
        pub async fn mlst(&mut self, pathname: Option<&str>) -> FtpResult<String> {
            let response = self
                .command(
                    Command::new_mlst(pathname),
                    &[Status::RequestedFileActionOk],
                )
                .await?;
            let mut lines = response.body_into_multiline_result()?;
            if lines.len() != 1 {
                return Err(FtpError::BadResponse);
            }
            let line = lines.pop().unwrap();
            Ok(line)
        }
        /// ### mlsd
        ///
        /// Execute `MLSD` command which returns the list of file names only.
        /// If `pathname` is omited then the list of files in the current directory will be
        /// returned otherwise it will the list of files on `pathname`.
        pub async fn mlsd(&mut self, pathname: Option<&str>) -> FtpResult<Vec<String>> {
            self.stream_lines(Command::new_mlsd(pathname)).await
        }
        /// Retrieves the modification time of the file at `pathname` if it exists.
        pub async fn mdtm<S: AsRef<str>>(&mut self, pathname: S) -> FtpResult<DateTime<Utc>> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Getting modification time for "],
                            &[::core::fmt::ArgumentV1::new_display(&pathname.as_ref())],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 472u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            let response = self
                .command(Command::new_mdtm(pathname), &[Status::File])
                .await?;
            let line = response.body_as_inline_result()?;
            match MDTM_RE.captures(line) {
                Some(caps) => {
                    let (year, month, day) = (
                        caps[1].parse::<i32>().unwrap(),
                        caps[2].parse::<u32>().unwrap(),
                        caps[3].parse::<u32>().unwrap(),
                    );
                    let (hour, minute, second) = (
                        caps[4].parse::<u32>().unwrap(),
                        caps[5].parse::<u32>().unwrap(),
                        caps[6].parse::<u32>().unwrap(),
                    );
                    Ok(Utc.ymd(year, month, day).and_hms(hour, minute, second))
                }
                None => Err(FtpError::BadResponse),
            }
        }
        /// Requests the server to list all extension commands, or extended mechanisms, that it supports.
        pub async fn feat(&mut self) -> FtpResult<Vec<String>> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["Feat"], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 496u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            let response = self.command(Command::Feat, &[Status::System]).await?;
            Ok(response.body.into_vec())
        }
        /// Requests the server to list all extension commands, or extended mechanisms, that it supports.
        pub async fn opts<S: AsRef<str>>(
            &mut self,
            cmd: S,
            cmd_options: Option<S>,
        ) -> FtpResult<String> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Opts \'", "\' \'", "\'"],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&cmd.as_ref()),
                                ::core::fmt::ArgumentV1::new_display(&optstrref(&cmd_options)),
                            ],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 503u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            let response = self
                .command(Command::new_opts(cmd, cmd_options), &[Status::CommandOk])
                .await?;
            response.body_into_inline_result()
        }
        /// Requests the server to list all extension commands, or extended mechanisms, that it supports.
        pub async fn lang<S: AsRef<str>>(&mut self, lang_tag: Option<S>) -> FtpResult<String> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Lang \'", "\'"],
                            &[::core::fmt::ArgumentV1::new_display(&optstrref(&lang_tag))],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 510u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            let response = self
                .command(Command::new_lang(lang_tag), &[Status::CommandOk])
                .await?;
            response.body_into_inline_result()
        }
        /// Retrieves the size of the file in bytes at `pathname` if it exists.
        pub async fn site<S: AsRef<str>>(&mut self, cmd: S) -> FtpResult<String> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["SITE \'", "\'"],
                            &[::core::fmt::ArgumentV1::new_display(&cmd.as_ref())],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 517u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            let response = self
                .command(Command::new_site(cmd), &[Status::CommandOk])
                .await?;
            response.body_into_inline_result()
        }
        /// Returns information on the server status, including the status of the current connection
        pub async fn stat<S: AsRef<str>>(&mut self, path: Option<S>) -> FtpResult<Vec<String>> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Stat \'", "\'"],
                            &[::core::fmt::ArgumentV1::new_display(&optstrref(&path))],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 524u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            let response = self
                .command(
                    Command::new_stat(path),
                    &[Status::System, Status::Directory, Status::File],
                )
                .await?;
            Ok(response.body.into_vec())
        }
        /// Retrieves the size of the file in bytes at `pathname` if it exists.
        pub async fn size<S: AsRef<str>>(&mut self, pathname: S) -> FtpResult<usize> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Getting file size for "],
                            &[::core::fmt::ArgumentV1::new_display(&pathname.as_ref())],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 531u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            let response = self
                .command(Command::new_size(pathname), &[Status::File])
                .await?;
            let line = response.body_as_inline_result()?;
            match SIZE_RE.captures(line) {
                Some(caps) => Ok(caps[1].parse().unwrap()),
                None => Err(FtpError::BadResponse),
            }
        }
        /// Retrieve stream "message"
        async fn get_lines_from_stream(
            data_stream: &mut BufReaderAsync<DataStreamAsync>,
        ) -> FtpResult<Vec<String>> {
            let mut lines: Vec<String> = Vec::new();
            loop {
                let mut line = String::new();
                match data_stream.read_line(&mut line).await {
                    Ok(0) => break,
                    Ok(_) => {
                        if line.ends_with('\n') {
                            line.pop();
                            if line.ends_with('\r') {
                                line.pop();
                            }
                        }
                        if line.is_empty() {
                            continue;
                        }
                        lines.push(line);
                    }
                    Err(_) => return Err(FtpError::BadResponse),
                }
            }
            {
                let lvl = ::log::Level::Trace;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Lines from stream "],
                            &[::core::fmt::ArgumentV1::new_debug(&lines)],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 566u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            Ok(lines)
        }
        /// Read response from stream
        async fn read_response(&mut self, expected_code: Status) -> FtpResult<Response> {
            self.read_response_in(&[expected_code]).await
        }
        /// Retrieve single line response
        async fn read_response_in(&mut self, expected_status: &[Status]) -> FtpResult<Response> {
            let mut line_buffer = String::new();
            let mut line = self.read_line(&mut line_buffer).await?;
            {
                let lvl = ::log::Level::Trace;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["CC IN: "],
                            &[::core::fmt::ArgumentV1::new_display(&line.trim_end())],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 581u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            if line.len() < CODE_LENGTH + 1 {
                return Err(FtpError::BadResponse);
            }
            let (mut status, mut delim, mut head) = parse_status_delim_tail(line)?;
            if self.skip450 {
                self.skip450 = false;
                if status == Status::RequestFileActionIgnored {
                    line = self.read_line(&mut line_buffer).await?;
                    {
                        let lvl = ::log::Level::Trace;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api_log(
                                ::core::fmt::Arguments::new_v1(
                                    &["CC IN: "],
                                    &[::core::fmt::ArgumentV1::new_display(&line.trim_end())],
                                ),
                                lvl,
                                &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 597u32),
                                ::log::__private_api::Option::None,
                            );
                        }
                    };
                    if line.len() < CODE_LENGTH + 1 {
                        return Err(FtpError::BadResponse);
                    }
                    (status, delim, head) = parse_status_delim_tail(line)?;
                }
            }
            let response = match delim {
                SPACE_CHAR => Response::new_inline(status, head),
                MINUS_CHAR => {
                    let mut body: Vec<String> = ::alloc::vec::Vec::new();
                    loop {
                        line = self.read_line(&mut line_buffer).await?;
                        {
                            let lvl = ::log::Level::Trace;
                            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                                ::log::__private_api_log(
                                    ::core::fmt::Arguments::new_v1(
                                        &["CC IN: "],
                                        &[::core::fmt::ArgumentV1::new_display(&line)],
                                    ),
                                    lvl,
                                    &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 618u32),
                                    ::log::__private_api::Option::None,
                                );
                            }
                        };
                        let first_char =
                            line.chars().nth(0).ok_or_else(|| FtpError::BadResponse)?;
                        match first_char {
                            SPACE_CHAR => body.push(line[1..].to_string()),
                            ch if ch.is_ascii_digit() => {
                                let (status2, delim, tail) = parse_status_delim_tail(line)?;
                                if status2 != status || delim != SPACE_CHAR {
                                    return Err(FtpError::BadResponse);
                                };
                                break Response::new_multiline(status, head, body, tail);
                            }
                            _ => {
                                return Err(FtpError::BadResponse);
                            }
                        }
                    }
                }
                _ => {
                    return Err(FtpError::BadResponse);
                }
            };
            if expected_status.contains(&status) {
                Ok(response)
            } else {
                let err = match status {
                    Status::BadCommand | Status::NotImplemented | Status::BadSequence => {
                        FtpError::BadCommand {
                            status,
                            message: response.body_into_inline_result()?,
                        }
                    }
                    Status::BadArguments | Status::NotImplementedParameter => {
                        FtpError::BadParameter {
                            status,
                            message: response.body_into_inline_result()?,
                        }
                    }
                    _ => FtpError::UnexpectedResponse(response),
                };
                Err(err)
            }
        }
        /// Write data to stream with command to perform
        async fn perform(&mut self, command: Command) -> FtpResult<()> {
            let command = command.to_string();
            {
                let lvl = ::log::Level::Trace;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["CC OUT: "],
                            &[::core::fmt::ArgumentV1::new_display(
                                &command.trim_end_matches("\r\n"),
                            )],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 666u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            let stream = self.reader.get_mut();
            stream.write_all(command.as_bytes()).await?;
            Ok(())
        }
        /// Execute command which send data back in a command stream
        pub async fn command(
            &mut self,
            command: Command,
            expected_code: &[Status],
        ) -> FtpResult<Response> {
            self.perform(command).await?;
            self.read_response_in(expected_code).await
        }
        /// Execute command which send data back in a separate stream
        async fn data_command(&mut self, cmd: Command) -> FtpResult<DataStreamAsync> {
            let stream = match self.mode {
                Mode::Passive => {
                    let addr = self.pasv().await?;
                    self.perform(cmd).await?;
                    TcpStreamAsync::connect(addr).await?
                }
                Mode::Active => {
                    let listener = self.active().await?;
                    self.perform(cmd).await?;
                    let (stream, _) = listener.accept().await?;
                    stream
                }
            };
            #[cfg(feature = "_secure")]
            {
                match self.tls_ctx {
                    Some(ref tls_ctx) => {
                        let tls_stream = tls_ctx
                            .tls_connector
                            .connect(tls_ctx.domain.as_str(), stream)
                            .await?;
                        Ok(DataStreamAsync::Ssl(tls_stream.into()))
                    }
                    None => Ok(DataStreamAsync::Tcp(stream)),
                }
            }
        }
        /// Create a new tcp listener and send a PORT command for it
        async fn active(&mut self) -> FtpResult<TcpListenerAsync> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["Starting local tcp listener..."], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 717u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            let listener = TcpListenerAsync::bind("0.0.0.0:0").await?;
            let addr = listener.local_addr()?;
            {
                let lvl = ::log::Level::Trace;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Local address is "],
                            &[::core::fmt::ArgumentV1::new_display(&addr)],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 721u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            let tcp_stream = match self.reader.get_ref() {
                DataStreamAsync::Tcp(stream) => stream,
                #[cfg(feature = "_secure")]
                DataStreamAsync::Ssl(stream) => stream.get_ref(),
            };
            let ip = tcp_stream.local_addr().unwrap().ip();
            let msb = addr.port() / 256;
            let lsb = addr.port() % 256;
            let ip_port = {
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["", ",", ","],
                    &[
                        ::core::fmt::ArgumentV1::new_display(&ip.to_string().replace(".", ",")),
                        ::core::fmt::ArgumentV1::new_display(&msb),
                        ::core::fmt::ArgumentV1::new_display(&lsb),
                    ],
                ));
                res
            };
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Active mode, listening on ", ":"],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&ip),
                                ::core::fmt::ArgumentV1::new_display(&addr.port()),
                            ],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 734u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["Running PORT command"], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 736u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            self.command(Command::Port(ip_port), &[Status::CommandOk])
                .await?;
            Ok(listener)
        }
        /// Runs the PASV command.
        async fn pasv(&mut self) -> FtpResult<SocketAddrAsync> {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(&["PASV command"], &[]),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 744u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            let response = self.command(Command::Pasv, &[Status::PassiveMode]).await?;
            let body = response.body_as_inline_result()?;
            let caps = PORT_RE
                .captures(body)
                .ok_or_else(|| FtpError::UnexpectedResponse(response.clone()))?;
            let (oct1, oct2, oct3, oct4) = (
                caps[1].parse::<u8>().unwrap(),
                caps[2].parse::<u8>().unwrap(),
                caps[3].parse::<u8>().unwrap(),
                caps[4].parse::<u8>().unwrap(),
            );
            let (msb, lsb) = (
                caps[5].parse::<u8>().unwrap(),
                caps[6].parse::<u8>().unwrap(),
            );
            let port = ((msb as u16) << 8) + lsb as u16;
            let addr = {
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["", ".", ".", ".", ":"],
                    &[
                        ::core::fmt::ArgumentV1::new_display(&oct1),
                        ::core::fmt::ArgumentV1::new_display(&oct2),
                        ::core::fmt::ArgumentV1::new_display(&oct3),
                        ::core::fmt::ArgumentV1::new_display(&oct4),
                        ::core::fmt::ArgumentV1::new_display(&port),
                    ],
                ));
                res
            };
            {
                let lvl = ::log::Level::Trace;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["Passive address: "],
                            &[::core::fmt::ArgumentV1::new_display(&addr)],
                        ),
                        lvl,
                        &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 765u32),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            let addr = SocketAddrAsync::from_str(&addr)?;
            Ok(addr)
        }
        async fn read_line<'s>(&mut self, line_buffer: &'s mut String) -> FtpResult<&'s str> {
            line_buffer.clear();
            match self.reader.read_line(line_buffer).await {
                Ok(size) => {
                    if size == 0 {
                        {
                            let lvl = ::log::Level::Debug;
                            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                                ::log::__private_api_log(
                                    ::core::fmt::Arguments::new_v1(&["ERR read_line: EOF"], &[]),
                                    lvl,
                                    &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 778u32),
                                    ::log::__private_api::Option::None,
                                );
                            }
                        };
                        return Err(FtpError::BadResponse);
                    }
                }
                Err(e) => {
                    {
                        let lvl = ::log::Level::Debug;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api_log(
                                ::core::fmt::Arguments::new_v1(
                                    &["ERR read_line: "],
                                    &[::core::fmt::ArgumentV1::new_debug(&e)],
                                ),
                                lvl,
                                &("suppaftp::ftp", "suppaftp::ftp", "src\\ftp\\mod.rs", 783u32),
                                ::log::__private_api::Option::None,
                            );
                        }
                    };
                    return Err(e.into());
                }
            };
            let line = line_buffer.trim_end_matches(|ch| ch == '\r' || ch == '\n');
            Ok(line)
        }
        /// Execute a command which returns list of strings in a separate stream
        async fn stream_lines(&mut self, cmd: Command) -> FtpResult<Vec<String>> {
            let mut data_stream = BufReaderAsync::new(self.data_command(cmd).await?);
            self.read_response_in(&[Status::AboutToSend, Status::AlreadyOpen])
                .await?;
            let lines = Self::get_lines_from_stream(&mut data_stream).await;
            self.finalize_retr_stream(data_stream).await?;
            lines
        }
    }
}
