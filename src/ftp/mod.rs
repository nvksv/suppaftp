//! # Async
//!
//! This module contains the definition for all async implementation of suppaftp

maybe_async::content! {

#![maybe_async::default(
    idents(
        async_native_tls(sync="native_tls", async), 
        async_std(sync="std", async), 
        copy(fn, use), 
        BufReader(use), 
        Read(use), 
        Write(use), 
        ToSocketAddrs(use), 
        SocketAddr(use), 
        TcpListener(use), 
        TcpStream(use),
        TlsConnector(use),
        Cursor(use),
        DataStream,
        TlsCtx,
        FtpStream,
        FtpStreamInternals,
        test_setup_stream(fn),
        test_finalize_stream(fn),
        test_tls_connector(fn),
    )
)]

mod tls_stream;
mod data_stream;
use super::utils::*;

use super::types::{FileType, FtpError, FtpResult, Mode, Response};
use super::Status;
//use crate::callbacks;
use crate::command::Command;
#[cfg(feature = "_secure")]
use crate::command::ProtectionLevel;

//#[cfg(feature = "support-ftpclient")]
//use crate::callbacks::{FtpClient};

#[maybe_async::maybe(sync(feature = "sync"), async(feature = "async"))]
use data_stream::DataStream;

#[maybe_async::maybe(sync(feature = "sync-secure"), async(feature = "async-secure"))]
use async_native_tls::TlsConnector;

#[maybe_async::maybe(sync(feature = "sync"), async(feature = "async"))]
use async_std::{
    io::{copy, BufReader, Read, Write},
    net::{ToSocketAddrs, SocketAddr, TcpListener, TcpStream},
};

#[cfg(feature = "sync")]
use std::io::BufRead;

#[cfg(feature = "async")]
use async_std::{prelude::*};

use chrono::offset::TimeZone;
use chrono::{DateTime, Utc};
use std::str::FromStr;
use std::string::String;

/// Some data for TLS mode
#[maybe_async::maybe(sync(feature="sync-secure"), async(feature="async-secure"))]
#[derive(Debug)]
pub struct TlsCtx {
    pub tls_connector: TlsConnector,
    pub domain: String,
}

#[maybe_async::maybe(
    sync(feature="sync", replace_feature("_secure", "sync-secure")), 
    async(feature="async", replace_feature("_secure", "async-secure")), 
)]
#[derive(Debug)]
pub struct FtpStreamInternals {
    skip450: bool,
}

#[maybe_async::maybe(
    sync(feature="sync", replace_feature("_secure", "sync-secure")), 
    async(feature="async", replace_feature("_secure", "async-secure")), 
)]
impl FtpStreamInternals {
    fn new() -> Self {
        Self {
            skip450: false,
        }
    }

    fn take_skip_450(&mut self) -> bool {
        std::mem::replace(&mut self.skip450, false)
    }

    fn set_skip_450(&mut self) {
        self.skip450 = true;
    }
}

/// Stream to interface with the FTP server. This interface is only for the command stream.
#[maybe_async::maybe(
    sync(feature="sync", replace_feature("_secure", "sync-secure")), 
    async(feature="async", replace_feature("_secure", "async-secure")), 
)]
#[derive(Debug)]
pub struct FtpStream {
    internals: FtpStreamInternals,
    reader: BufReader<DataStream>,
    mode: Mode,
    #[cfg(feature = "_secure")]
    tls_ctx: Option<TlsCtx>,
    #[cfg(feature = "_with-welcome-msg")]
    welcome_msg: Option<String>,
    // #[cfg(feature = "support-ftpclient")]
    // callbacks: FtpStreamCallbacksRef,
}

#[maybe_async::maybe(
    sync(feature="sync", replace_feature("_secure", "sync-secure")), 
    async(feature="async", replace_feature("_secure", "async-secure")), 
)]
impl FtpStream {
    /// Creates an FTP Stream.
    pub async fn connect<A: ToSocketAddrs, #[cfg(feature = "support-ftpclient")] Client: FtpClient>(addr: A) -> FtpResult<Self> {
        debug!("Connecting to server");

        let stream = TcpStream::connect(addr).await?;
        debug!("Established connection with server");

        let mut ftp_stream = Self {
            internals: FtpStreamInternals::new(),
            reader: BufReader::new(DataStream::Tcp(stream)),
            mode: Mode::Passive,
            #[cfg(feature = "_secure")]
            tls_ctx: None,
            #[cfg(feature = "_with-welcome-msg")]
            welcome_msg: None,
            // #[cfg(feature = "support-ftpclient")]
            // callbacks,
        };

        debug!("Reading server response...");
        #[allow(unused_variables)]
        let response = ftp_stream.read_response_in(&[Status::Ready]).await?;
        debug!("Server READY; response: {}", response.body);

        #[cfg(feature = "_with-welcome-msg")]
        {
            ftp_stream.welcome_msg = Some(response.body.into_string());
        }

        #[cfg(feature = "support-ftpclient")]
        {
//            ftp_stream.callbacks.welcome_response(response);
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
        debug!("Changed mode to {:?}", mode);
        self.mode = mode;
    }

    /// Switch to a secure mode if possible, using a provided TLS configuration.
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
        tls_connector: TlsConnector,
        domain: &str,
    ) -> FtpResult<Self> {
        // Ask the server to start securing data.
        debug!("Initializing TLS auth");
        self.command(Command::Auth, &[Status::AuthOk]).await?;
        debug!("TLS OK; initializing TLS stream");

        let stream = tls_connector.connect(
            domain, 
            self.reader.into_inner().into_tcp_stream()
        ).await?;
        debug!("TLS stream OK");

        let mut secured_ftp_stream = Self {
            internals: FtpStreamInternals::new(),
            reader: BufReader::new(DataStream::Tls(stream.into())),
            mode: self.mode,
            tls_ctx: Some(TlsCtx{ tls_connector, domain: domain.into() }),
            #[cfg(feature = "_with-welcome-msg")]
            welcome_msg: self.welcome_msg,
        };

        // Set protection buffer size
        secured_ftp_stream.command(Command::Pbsz(0), &[Status::CommandOk]).await?;
        // Change the level of data protectio to Private
        secured_ftp_stream.command(Command::Prot(ProtectionLevel::Private), &[Status::CommandOk]).await?;

        Ok(secured_ftp_stream)
    }

    /// Returns welcome message retrieved from server (if available)
    #[cfg(feature = "_with-welcome-msg")]
    pub fn get_welcome_msg(&self) -> Option<&str> {
        self.welcome_msg.as_deref()
    }

    /// Returns a reference to the underlying TcpStream.
    pub async fn get_ref(&self) -> &TcpStream {
        self.reader.get_ref().get_ref()
    }    

    /// Log in to the FTP server.
    pub async fn login<S: AsRef<str>>(&mut self, user: S, password: S) -> FtpResult<()> {
        debug!("Signin in with user '{}'", user.as_ref());
        let user_response = self.command(Command::new_user(user), &[Status::LoggedIn, Status::NeedPassword]).await?;
        
        if user_response.status == Status::NeedPassword {
            debug!("Password is required");
            self.command(Command::new_pass(password), &[Status::LoggedIn]).await?;
        }

        debug!("Login OK");
        Ok(())
    }

    /// Perform clear command channel (CCC).
    /// Once the command is performed, the command channel will be encrypted no more.
    /// The data stream will still be secure.
    #[cfg(feature = "_secure")]
    pub async fn clear_command_channel(mut self) -> FtpResult<Self> {
        // Ask the server to stop securing data
        debug!("performing clear command channel");
        self.command(Command::ClearCommandChannel, &[Status::CommandOk]).await?;
        trace!("CCC OK");
        self.reader = BufReader::new(DataStream::Tcp(self.reader.into_inner().into_tcp_stream()));
        Ok(self)
    }

    /// Change the current directory to the path specified.
    pub async fn cwd<S: AsRef<str>>(&mut self, path: S) -> FtpResult<()> {
        debug!("Changing working directory to {}", path.as_ref());
        self.command(Command::new_cwd(path), &[Status::RequestedFileActionOk]).await?;
        Ok(())
    }

    /// Move the current directory to the parent directory.
    pub async fn cdup(&mut self) -> FtpResult<()> {
        debug!("Going to parent directory");
        self.command(Command::Cdup, &[Status::CommandOk, Status::RequestedFileActionOk]).await?;
        Ok(())
    }

    /// Gets the current directory
    pub async fn pwd(&mut self) -> FtpResult<String> {
        debug!("Getting working directory");
        let response = self.command(Command::Pwd, &[Status::PathCreated]).await?;
        let body = response.body_as_inline_result()?;

        match (body.find('"'), body.rfind('"')) {
            (Some(begin), Some(end)) if begin < end => Ok(body[begin + 1..end].to_string()),
            _ => Err(FtpError::UnexpectedResponse(response)),
        }
    }

    /// This does nothing. This is usually just used to keep the connection open.
    pub async fn noop(&mut self) -> FtpResult<()> {
        debug!("Pinging server");
        self.command(Command::Noop, &[Status::CommandOk]).await?;
        Ok(())
    }

    /// This creates a new directory on the server.
    pub async fn mkdir<S: AsRef<str>>(&mut self, pathname: S) -> FtpResult<()> {
        debug!("Creating directory at {}", pathname.as_ref());
        self.command(Command::new_mkd(pathname), &[Status::PathCreated]).await?;
        Ok(())
    }

    /// Sets the type of file to be transferred. That is the implementation
    /// of `TYPE` command.
    pub async fn transfer_type(&mut self, file_type: FileType) -> FtpResult<()> {
        debug!("Setting transfer type {}", file_type.to_string());
        self.command(Command::Type(file_type), &[Status::CommandOk]).await?;
        Ok(())
    }

    /// Quits the current FTP session.
    pub async fn quit(mut self) -> FtpResult<()> {
        debug!("Quitting stream");
        self.command(Command::Quit, &[Status::Closing]).await?;
        Ok(())
    }

    /// Renames the file from_name to to_name
    pub async fn rename<S: AsRef<str>>(&mut self, from_name: S, to_name: S) -> FtpResult<()> {
        debug!(
            "Renaming '{}' to '{}'",
            from_name.as_ref(),
            to_name.as_ref()
        );
        self.command(Command::new_rename_from(from_name), &[Status::RequestFilePending]).await?;
        self.command(Command::new_rename_to(to_name),     &[Status::RequestedFileActionOk]).await?;
        Ok(())
    }

    /// The implementation of `RETR` command where `filename` is the name of the file
    /// to download from FTP and `reader` is the function which operates with the
    /// data stream opened.
    ///
    /// ```
    /// # use suppaftp::{FtpStream, FtpError};
    /// # use std::io::Cursor;
    /// # let mut conn = FtpStream::connect("ftp.server.local:21").unwrap();
    /// # conn.login("test", "test").and_then(|_| {
    /// #     let mut reader = Cursor::new("hello, world!".as_bytes());
    /// #     conn.put_file("retr.txt", &mut reader)
    /// # }).unwrap();
    /// assert!(conn.retr("retr.txt", |stream| {
    ///     let mut buf = Vec::new();
    ///     stream.read_to_end(&mut buf).map(|_|
    ///         assert_eq!(buf, "hello, world!".as_bytes())
    ///     ).map_err(|e| FtpError::ConnectionError(e))
    /// }).is_ok());
    /// # assert!(conn.rm("retr.txt").is_ok());
    /// ```
    pub async fn retr<S, F, T>(&mut self, file_name: S, mut reader: F) -> FtpResult<T>
    where
        F: FnMut(&mut (dyn Read + std::marker::Unpin)) -> FtpResult<T>,
    // pub async fn retr<S, F, T>(&mut self, file_name: S, mut reader: F) -> FtpResult<T>
    // where
    //     F: Future<Output = ()>
    //     FnMut(&mut (dyn Read + std::marker::Unpin)) -> FtpResult<T>,
    
    //     <F: >(_:  impl Fn() -> F)        
        S: AsRef<str>,
    {
        let mut stream = self.retr_as_stream(file_name).await?;

        let result = reader(&mut stream)?;
        self.finalize_retr_stream(stream).await?;
        Ok(result)
    }

    // fn retr_as_buffer(...) requires async closures which are still unstable
    //
    /// Simple way to retr a file from the server. This stores the file in a buffer in memory.
    ///
    /// ```
    /// # use suppaftp::{FtpStream, FtpError};
    /// # use std::io::Cursor;
    /// # let mut conn = FtpStream::connect("ftp.server.local:21").unwrap();
    /// # conn.login("test", "test").and_then(|_| {
    /// #     let mut reader = Cursor::new("hello, world!".as_bytes());
    /// #     conn.put_file("simple_retr.txt", &mut reader)
    /// # }).unwrap();
    /// let cursor = conn.retr_as_buffer("simple_retr.txt").unwrap();
    /// // do something with bytes
    /// assert_eq!(cursor.into_inner(), "hello, world!".as_bytes());
    /// # assert!(conn.rm("simple_retr.txt").is_ok());
    /// ```
    /// Retrieves the file name specified from the server as a readable stream.
    /// This method is a more complicated way to retrieve a file.
    /// The reader returned should be dropped.
    /// Also you will have to read the response to make sure it has the correct value.
    /// Once file has been read, call `finalize_retr_stream()`
    pub async fn retr_as_stream<S: AsRef<str>>(&mut self, file_name: S) -> FtpResult<DataStream> {
        debug!("Retrieving '{}'", file_name.as_ref());
        let data_stream = self.data_command(Command::new_retr(file_name)).await?;
        self.read_response_in(&[Status::AboutToSend, Status::AlreadyOpen]).await?;
        Ok(data_stream)
    }

    /// Finalize retr stream; must be called once the requested file, got previously with `retr_as_stream()` has been read
    pub async fn finalize_retr_stream(&mut self, stream: impl Read) -> FtpResult<()> {
        debug!("Finalizing retr stream");
        // Drop stream NOTE: must be done first, otherwise server won't return any response
        drop(stream);
        trace!("dropped stream");
        // Then read response
        self.read_response_in(&[Status::ClosingDataConnection, Status::RequestedFileActionOk]).await?;
        Ok(())
    }

    /// Removes the remote pathname from the server.
    pub async fn rmdir<S: AsRef<str>>(&mut self, pathname: S) -> FtpResult<()> {
        debug!("Removing directory {}", pathname.as_ref());
        self.command(Command::new_rmd(pathname), &[Status::RequestedFileActionOk]).await?;
        Ok(())
    }

    /// Remove the remote file from the server.
    pub async fn rm<S: AsRef<str>>(&mut self, filename: S) -> FtpResult<()> {
        debug!("Removing file {}", filename.as_ref());
        self.command(Command::new_dele(filename), &[Status::RequestedFileActionOk]).await?;
        Ok(())
    }

    /// This stores a file on the server.
    /// r argument must be any struct which implemenents the Read trait.
    /// Returns amount of written bytes
    pub async fn put_file<S, R>(&mut self, filename: S, r: &mut R) -> FtpResult<u64>
    where
        R: Read + std::marker::Unpin,
        S: AsRef<str>,
    {
        // Get stream
        let mut data_stream = self.put_with_stream(filename).await?;

        let bytes = copy(r, &mut data_stream).await?;

        self.finalize_put_stream(data_stream).await?;
        Ok(bytes)
    }

    /// Send PUT command and returns a BufWriter, which references the file created on the server
    /// The returned stream must be then correctly manipulated to write the content of the source file to the remote destination
    /// The stream must be then correctly dropped.
    /// Once you've finished the write, YOU MUST CALL THIS METHOD: `finalize_put_stream`
    pub async fn put_with_stream<S: AsRef<str>>(&mut self, filename: S) -> FtpResult<DataStream> {
        debug!("Put file {}", filename.as_ref());
        let data_stream = self.data_command(Command::new_store(filename)).await?;
        self.read_response_in(&[Status::AlreadyOpen, Status::AboutToSend]).await?;
        Ok(data_stream)
    }

    /// Finalize put when using stream
    /// This method must be called once the file has been written and
    /// `put_with_stream` has been used to write the file
    pub async fn finalize_put_stream(&mut self, stream: impl Write) -> FtpResult<()> {
        debug!("Finalizing put stream");
        // Drop stream NOTE: must be done first, otherwise server won't return any response
        drop(stream);
        trace!("Stream dropped");
        // Read response
        self.read_response_in(&[Status::ClosingDataConnection, Status::RequestedFileActionOk]).await?;
        Ok(())
    }

    /// Open specified file for appending data. Returns the stream to append data to specified file.
    /// Once you've finished the write, YOU MUST CALL THIS METHOD: `finalize_put_stream`
    pub async fn append_with_stream<S: AsRef<str>>(&mut self, filename: S) -> FtpResult<DataStream> {
        debug!("Appending to file {}", filename.as_ref());
        let stream = self.data_command(Command::Appe(filename.as_ref().to_string())).await?;
        self.read_response_in(&[Status::AlreadyOpen, Status::AboutToSend]).await?;
        Ok(stream)
    }

    /// Append data from reader to file at `filename`
    pub async fn append_file<R>(&mut self, filename: &str, r: &mut R) -> FtpResult<u64>
    where
        R: Read + std::marker::Unpin,
    {
        // Get stream
        let mut data_stream = self.append_with_stream(filename).await?;

        let bytes = copy(r, &mut data_stream).await?;

        self.finalize_put_stream(Box::new(data_stream)).await?;
        Ok(bytes)
    }

    /// abort the previous FTP service command
    pub async fn abort<R>(&mut self, data_stream: R) -> FtpResult<()>
    where
        R: Read + std::marker::Unpin,
    {
        debug!("Aborting active file transfer");
        self.perform(Command::Abor).await?;
        // Drop stream NOTE: must be done first, otherwise server won't return any response
        drop(data_stream);
        trace!("dropped stream");
        self.read_response_in(&[Status::ClosingDataConnection, Status::TransferAborted]).await?;
        self.read_response_in(&[Status::ClosingDataConnection, Status::TransferAborted]).await?;

        // Sometimes ProFTPd sends three lines, so we must to skip the last one "450 Transfer aborted. Link to file server lost"
        self.internals.set_skip_450();

        trace!("Transfer aborted");
        Ok(())
    }

    /// Tell the server to resume the transfer from a certain offset. The offset indicates the amount of bytes to skip
    /// from the beginning of the file.
    /// the REST command does not actually initiate the transfer.
    /// After issuing a REST command, the client must send the appropriate FTP command to transfer the file
    ///
    /// It is possible to cancel the REST command, sending a REST command with offset 0
    pub async fn resume_transfer(&mut self, offset: usize) -> FtpResult<()> {
        debug!("Requesting to resume transfer at offset {}", offset);
        self.command(Command::Rest(offset), &[Status::RequestFilePending]).await?;
        debug!("Resume transfer accepted");
        Ok(())
    }

    /// Execute `LIST` command which returns the detailed file listing in human readable format.
    /// If `pathname` is omited then the list of files in the current directory will be
    /// returned otherwise it will the list of files on `pathname`.
    ///
    /// ### Parse result
    ///
    /// You can parse the output of this command with
    ///
    /// ```rust
    ///
    /// use std::str::FromStr;
    /// use suppaftp::list::File;
    ///
    /// let file: File = File::from_str("-rw-rw-r-- 1 0  1  8192 Nov 5 2018 omar.txt")
    ///     .ok()
    ///     .unwrap();
    /// ```
    pub async fn list(&mut self, pathname: Option<&str>) -> FtpResult<Vec<String>> {
        debug!(
            "Reading {} directory content",
            pathname.unwrap_or("working")
        );

        self.stream_lines(Command::new_list(pathname)).await
    }

    /// Execute `NLST` command which returns the list of file names only.
    /// If `pathname` is omited then the list of files in the current directory will be
    /// returned otherwise it will the list of files on `pathname`.
    pub async fn nlst(&mut self, pathname: Option<&str>) -> FtpResult<Vec<String>> {
        debug!(
            "Getting file names for {} directory",
            pathname.unwrap_or("working")
        );
        self.stream_lines(Command::new_nlst(pathname)).await
    }

    /// Execute `MLST` command which returns the list of file names only.
    /// If `pathname` is omited then the list of files in the current directory will be
    /// returned otherwise it will the list of files on `pathname`.
    pub async fn mlst(&mut self, pathname: Option<&str>) -> FtpResult<String> {
        let response = self.command(Command::new_mlst(pathname), &[Status::RequestedFileActionOk]).await?;
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
        debug!("Getting modification time for {}", pathname.as_ref());
        let response = self.command(Command::new_mdtm(pathname), &[Status::File]).await?;
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
        debug!("Feat");
        let response = self.command(Command::Feat, &[Status::System]).await?;
        Ok(response.body.into_vec())
    }

    /// Requests the server to list all extension commands, or extended mechanisms, that it supports.
    pub async fn opts<S: AsRef<str>>(&mut self, cmd: S, cmd_options: Option<S>) -> FtpResult<String> {
        debug!("Opts '{}' '{}'", cmd.as_ref(), optstrref(&cmd_options));
        let response = self.command(Command::new_opts(cmd, cmd_options), &[Status::CommandOk]).await?;
        response.body_into_inline_result()
    }

    /// Requests the server to list all extension commands, or extended mechanisms, that it supports.
    pub async fn lang<S: AsRef<str>>(&mut self, lang_tag: Option<S>) -> FtpResult<String> {
        debug!("Lang '{}'", optstrref(&lang_tag));
        let response = self.command(Command::new_lang(lang_tag), &[Status::CommandOk]).await?;
        response.body_into_inline_result()
    }

    /// Retrieves the size of the file in bytes at `pathname` if it exists.
    pub async fn site<S: AsRef<str>>(&mut self, cmd: S) -> FtpResult<String> {
        debug!("SITE '{}'", cmd.as_ref());
        let response = self.command(Command::new_site(cmd), &[Status::CommandOk]).await?;
        response.body_into_inline_result()
    }

    /// Returns information on the server status, including the status of the current connection
    pub async fn stat<S: AsRef<str>>(&mut self, path: Option<S>) -> FtpResult<Vec<String>> {
        debug!("Stat '{}'", optstrref(&path));
        let response = self.command(Command::new_stat(path), &[Status::System, Status::Directory, Status::File]).await?;
        Ok(response.body.into_vec())
    }

    /// Retrieves the size of the file in bytes at `pathname` if it exists.
    pub async fn size<S: AsRef<str>>(&mut self, pathname: S) -> FtpResult<usize> {
        debug!("Getting file size for {}", pathname.as_ref());
        let response = self.command(Command::new_size(pathname), &[Status::File]).await?;
        let line = response.body_as_inline_result()?;

        match SIZE_RE.captures(line) {
            Some(caps) => Ok(caps[1].parse().unwrap()),
            None => Err(FtpError::BadResponse),
        }
    }

    // -- private

    /// Retrieve stream "message"
    async fn get_lines_from_stream(data_stream: &mut BufReader<DataStream>) -> FtpResult<Vec<String>> {
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
        trace!("Lines from stream {:?}", lines);

        Ok(lines)
    }

    // /// Read response from stream
    // async fn read_response(&mut self, expected_code: Status) -> FtpResult<Response> {
    //     self.read_response_in(&[expected_code]).await
    // }

    /// Retrieve single line response
    async fn read_response_in(&mut self, expected_status: &[Status]) -> FtpResult<Response> {
        let mut line_buffer = String::new();
        let mut line = self.read_line(&mut line_buffer).await?;

        trace!("CC IN: {}", line.trim_end());

        if line.len() < CODE_LENGTH+1 {
            debug!("Response line too short: \"{}\"", line);
            return Err(FtpError::BadResponse);
        }

        let (mut status, mut delim, mut head) = parse_status_delim_tail(line)?;

        if self.internals.take_skip_450() {
            if status == Status::RequestFileActionIgnored {
                // Skip this status and retrieve next line

                line = self.read_line(&mut line_buffer).await?;

                trace!("CC IN: {}", line.trim_end());
        
                if line.len() < CODE_LENGTH+1 {
                    debug!("Response line too short: \"{}\"", line);
                    return Err(FtpError::BadResponse);
                }

                (status, delim, head) = parse_status_delim_tail(line)?;
            }
        }

        let response = match delim {
            SPACE_CHAR => {
                Response::new_inline( status, head )
            },
            MINUS_CHAR => {
                let mut body: Vec<String> = vec![];

                // multiple line reply
                // loop while the line does not begin with the code and a space
                loop {
                    line = self.read_line(&mut line_buffer).await?;
                    trace!("CC IN: {}", line);
                    
                    let first_char = line.chars().nth(0).ok_or_else(|| FtpError::BadResponse)?;
                    match first_char {
                        SPACE_CHAR => {
                            body.push(line[1..].to_string())
                        },
                        ch if ch.is_ascii_digit() => {
                            let (status2, delim, tail) = parse_status_delim_tail(line)?;
                            if status2 != status || delim != SPACE_CHAR {
                                return Err(FtpError::BadResponse);
                            };
                            break Response::new_multiline( status, head, body, tail );
                        },
                        _ => {
                            debug!("Bad first char of response line: \"{}\"", line);
                            return Err(FtpError::BadResponse);
                        }
                    }

                }
            },
            _ => {
                debug!("Bad delimitier of response line: \"{}\"", line);
                return Err(FtpError::BadResponse);
            }
        };

        // Return Ok or error with response
        if expected_status.contains(&status) {
            Ok(response)
        } else {
            let err = match status {
                Status::BadCommand | Status::NotImplemented | Status::BadSequence => {
                    FtpError::BadCommand{ status, message: response.body_into_inline_result()? }
                },
                Status::BadArguments | Status::NotImplementedParameter => {
                    FtpError::BadParameter{ status, message: response.body_into_inline_result()? }
                },
                _ => {
                    debug!("Bad status: {:?}", status);
                    FtpError::UnexpectedResponse(response)
                }
            };
            Err(err)
        }
    }

    /// Write data to stream with command to perform
    async fn perform(&mut self, command: Command) -> FtpResult<()> {
        let command = command.to_string();
        trace!("CC OUT: {}", command.trim_end_matches("\r\n"));

        let stream = self.reader.get_mut();
        stream.write_all(command.as_bytes()).await?;
        Ok(())
    }

    /// Execute command which send data back in a command stream
    pub async fn command(&mut self, command: Command, expected_code: &[Status]) -> FtpResult<Response> {
        self.perform(command).await?;
        self.read_response_in(expected_code).await
    }

    /// Execute command which send data back in a separate stream
    async fn data_command(&mut self, cmd: Command) -> FtpResult<DataStream> {
        let stream = match self.mode {
            Mode::Passive => {
                let addr = self.pasv().await?;
                self.perform(cmd).await?;
                TcpStream::connect(addr).await?
            },
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
                    let tls_stream = tls_ctx.tls_connector.connect(tls_ctx.domain.as_str(), stream).await?;
                    Ok(DataStream::Tls(tls_stream.into()))
                },
                None => {
                    Ok(DataStream::Tcp(stream))
                },
            }
        }

        #[cfg(not(feature = "_secure"))]
        {
            Ok(DataStream::Tcp(stream))
        }

    }

    /// Create a new tcp listener and send a PORT command for it
    async fn active(&mut self) -> FtpResult<TcpListener> {
        debug!("Starting local tcp listener...");
        let listener = TcpListener::bind("0.0.0.0:0").await?;

        let addr = listener.local_addr()?;
        trace!("Local address is {}", addr);

        let tcp_stream = match self.reader.get_ref() {
            DataStream::Tcp(stream) => stream,

            #[cfg(feature = "_secure")]
            DataStream::Tls(stream) => stream.get_ref(),
        };
        let ip = tcp_stream.local_addr().unwrap().ip();

        let msb = addr.port() / 256;
        let lsb = addr.port() % 256;
        let ip_port = format!("{},{},{}", ip.to_string().replace(".", ","), msb, lsb);
        debug!("Active mode, listening on {}:{}", ip, addr.port());

        debug!("Running PORT command");
        self.command(Command::Port(ip_port), &[Status::CommandOk]).await?;

        Ok(listener)
    }
    
    /// Runs the PASV command.
    async fn pasv(&mut self) -> FtpResult<SocketAddr> {
        debug!("PASV command");
        let response = self.command(Command::Pasv, &[Status::PassiveMode]).await?;
        let body = response.body_as_inline_result()?;

        // PASV response format: 227 Entering Passive Mode (h1,h2,h3,h4,p1,p2).
        let caps = PORT_RE.captures(body).ok_or_else(|| FtpError::UnexpectedResponse(response.clone()))?;

        // If the regex matches we can be sure groups contains numbers
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
        let addr = format!("{}.{}.{}.{}:{}", oct1, oct2, oct3, oct4, port);
        
        trace!("Passive address: {}", addr);
        let addr = SocketAddr::from_str(&addr)?;

        Ok(addr)
    }

    async fn read_line<'s>(&mut self, line_buffer: &'s mut String) -> FtpResult<&'s str> {

        line_buffer.clear();
        
        match self.reader.read_line(line_buffer).await {
            Ok(size) => {
                if size == 0 {
                    debug!("ERR read_line: EOF");
                    return Err(FtpError::BadResponse);
                }
            },
            Err(e) => {
                debug!("ERR read_line: {:?}", e);
                return Err(e.into())
            },
        };
    
        let line = line_buffer.trim_end_matches(|ch| ch == '\r' || ch == '\n');
    
        Ok(line)
    }     

    /// Execute a command which returns list of strings in a separate stream
    async fn stream_lines(&mut self, cmd: Command) -> FtpResult<Vec<String>> {
        let mut data_stream = BufReader::new(self.data_command(cmd).await?);
        self.read_response_in(&[Status::AboutToSend, Status::AlreadyOpen]).await?;
        let lines = Self::get_lines_from_stream(&mut data_stream).await;
        self.finalize_retr_stream(data_stream).await?;
        lines
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::test::*;
    use crate::types::FormatControl;

    use pretty_assertions::assert_eq;
    use rand::{distributions::Alphanumeric, thread_rng, Rng};

    use serial_test::serial;

    #[maybe_async::maybe(sync(feature="sync"), async(feature="async"))]
    use async_std::io::Cursor;

    #[maybe_async::maybe(sync(feature="sync"), async(feature="async"))]
    #[maybe_async::only_if(async)]
    fn test_tls_connector() -> TlsConnector {
        TlsConnector::new()
    }

    #[maybe_async::maybe(sync(feature="sync"), async(feature="async"))]
    #[maybe_async::only_if(sync)]
    fn test_tls_connector() -> TlsConnector {
        TlsConnector::new().unwrap()
    }

    #[maybe_async::maybe(sync(feature="sync", test), async(feature="async", async_attributes::test))]
    #[serial]
    async fn connect() {
        crate::log_init();
        let stream: FtpStream = test_setup_stream().await;
        test_finalize_stream(stream).await;
    }

    #[maybe_async::maybe(sync(feature="sync-secure", test), async(feature="async-secure", async_attributes::test))]
    #[serial]
    async fn connect_tls() {
        crate::log_init();
        let ftp_stream = FtpStream::connect(TEST_TLS_SERVER_ADDR).await.unwrap();
        let mut ftp_stream = ftp_stream
            .into_secure(test_tls_connector(), TEST_TLS_SERVER_NAME)
            .await
            .ok()
            .unwrap();
        // Set timeout (to test ref to tls)
        assert!(ftp_stream.get_ref().await.set_ttl(255).is_ok());
        // Login
        assert!(ftp_stream.login(TEST_TLS_SERVER_LOGIN, TEST_TLS_SERVER_PASSWORD).await.is_ok());
        // PWD
        assert_eq!(ftp_stream.pwd().await.ok().unwrap().as_str(), "/");
        // Quit
        assert!(ftp_stream.quit().await.is_ok());
    }

    #[maybe_async::maybe(sync(feature="sync-secure", test), async(feature="async-secure", async_attributes::test))]
    #[serial]
    async fn should_work_after_clear_command_channel() {
        crate::log_init();
        let mut ftp_stream = FtpStream::connect(TEST_TLS_SERVER_ADDR)
            .await
            .unwrap()
            .into_secure(test_tls_connector(), TEST_TLS_SERVER_NAME)
            .await
            .ok()
            .unwrap()
            .clear_command_channel()
            .await
            .ok()
            .unwrap();
        // Login
        assert!(ftp_stream.login(TEST_TLS_SERVER_LOGIN, TEST_TLS_SERVER_PASSWORD).await.is_ok());
        // CCC
        assert!(ftp_stream.pwd().await.is_ok());
        assert!(ftp_stream.list(None).await.is_ok());
        assert!(ftp_stream.quit().await.is_ok());
    }

    #[maybe_async::maybe(sync(feature="sync", test), async(feature="async", async_attributes::test))]
    #[serial]
    async fn should_change_mode() {
        crate::log_init();
        let mut ftp_stream = FtpStream::connect(TEST_TLS_SERVER_ADDR)
            .await
            .map(|x| x.active_mode())
            .unwrap();
        assert_eq!(ftp_stream.mode, Mode::Active);
        ftp_stream.set_mode(Mode::Passive);
        assert_eq!(ftp_stream.mode, Mode::Passive);
    }

    #[maybe_async::maybe(sync(feature="sync", test), async(feature="async", async_attributes::test))]
    #[cfg(feature = "_with-welcome-msg")]
    #[serial]
    async fn welcome_message() {
        crate::log_init();
        let stream: FtpStream = test_setup_stream().await;
        assert_eq!(
            stream.get_welcome_msg().unwrap(),
            TEST_SERVER_WELCOME
        );
        test_finalize_stream(stream).await;
    }

    #[maybe_async::maybe(sync(feature="sync", test), async(feature="async", async_attributes::test))]
    #[serial]
    async fn get_ref() {
        crate::log_init();
        let stream: FtpStream = test_setup_stream().await;
        assert!(stream.get_ref().await.set_ttl(255).is_ok());
        test_finalize_stream(stream).await;
    }

    #[maybe_async::maybe(sync(feature="sync", test), async(feature="async", async_attributes::test))]
    #[serial]
    async fn change_wrkdir() {
        crate::log_init();
        let mut stream: FtpStream = test_setup_stream().await;
        let wrkdir: String = stream.pwd().await.ok().unwrap();
        assert!(stream.cwd("/").await.is_ok());
        assert_eq!(stream.pwd().await.ok().unwrap().as_str(), "/");
        assert!(stream.cwd(wrkdir.as_str()).await.is_ok());
        test_finalize_stream(stream).await;
    }

    #[maybe_async::maybe(sync(feature="sync", test), async(feature="async", async_attributes::test))]
    #[serial]
    async fn cd_up() {
        crate::log_init();
        let mut stream: FtpStream = test_setup_stream().await;
        let wrkdir: String = stream.pwd().await.ok().unwrap();
        assert!(stream.cdup().await.is_ok());
        assert_eq!(stream.pwd().await.ok().unwrap().as_str(), "/");
        assert!(stream.cwd(wrkdir.as_str()).await.is_ok());
        test_finalize_stream(stream).await;
    }

    #[maybe_async::maybe(sync(feature="sync", test), async(feature="async", async_attributes::test))]
    #[serial]
    async fn noop() {
        crate::log_init();
        let mut stream: FtpStream = test_setup_stream().await;
        assert!(stream.noop().await.is_ok());
        test_finalize_stream(stream).await;
    }

    #[maybe_async::maybe(sync(feature="sync", test), async(feature="async", async_attributes::test))]
    #[serial]
    async fn make_and_remove_dir() {
        crate::log_init();
        let mut stream: FtpStream = test_setup_stream().await;
        // Make directory
        assert!(stream.mkdir("omar").await.is_ok());
        // It shouldn't allow me to re-create the directory; should return error code 550
        match stream.mkdir("omar").await.err().unwrap() {
            FtpError::UnexpectedResponse(Response { status, body: _ }) => {
                assert_eq!(status, Status::FileUnavailable)
            }
            err => panic!("Expected UnexpectedResponse, got {}", err),
        }
        // Remove directory
        assert!(stream.rmdir("omar").await.is_ok());
        test_finalize_stream(stream).await;
    }

    #[maybe_async::maybe(sync(feature="sync", test), async(feature="async", async_attributes::test))]
    #[serial]
    async fn set_transfer_type() {
        crate::log_init();
        let mut stream: FtpStream = test_setup_stream().await;
        assert!(stream.transfer_type(FileType::Binary).await.is_ok());
        assert!(stream
            .transfer_type(FileType::Ascii(FormatControl::Default))
            .await
            .is_ok());
        test_finalize_stream(stream).await;
    }

    #[maybe_async::maybe(sync(feature="sync", test), async(feature="async", async_attributes::test))]
    #[serial]
    async fn transfer_file() {
        crate::log_init();

        let mut stream: FtpStream = test_setup_stream().await;
        // Set transfer type to Binary
        assert!(stream.transfer_type(FileType::Binary).await.is_ok());
        // Write file
        let file_data = "test data\n";
        let mut reader = Cursor::new(file_data.as_bytes());
        assert!(stream.put_file("test.txt", &mut reader).await.is_ok());
        // Append file
        let mut reader = Cursor::new(file_data.as_bytes());
        assert!(stream.append_file("test.txt", &mut reader).await.is_ok());
        // Read file
        let mut reader = stream.retr_as_stream("test.txt").await.ok().unwrap();
        let mut buffer = Vec::new();
        assert!(reader.read_to_end(&mut buffer).await.is_ok());
        // Verify file matches
        assert_eq!(buffer.as_slice(), "test data\ntest data\n".as_bytes());
        // Finalize
        assert!(stream.finalize_retr_stream(reader).await.is_ok());
        // assert_eq!(
        //     stream
        //         .retr_as_buffer("test.txt")
        //         .map(|bytes| bytes.into_inner())
        //         .ok()
        //         .unwrap(),
        //     file_data.as_bytes()
        // );        
        // Get size
        assert_eq!(stream.size("test.txt").await.ok().unwrap(), 20);
        // Size of non-existing file
        assert!(stream.size("omarone.txt").await.is_err());
        // List directory
        assert_eq!(stream.list(None).await.ok().unwrap().len(), 1);
        // list names
        assert_eq!(
            stream.nlst(None).await.ok().unwrap().as_slice(),
            &["test.txt"]
        );
        // modification time
        assert!(stream.mdtm("test.txt").await.is_ok());
        // Remove file
        assert!(stream.rm("test.txt").await.is_ok());
        assert!(stream.mdtm("test.txt").await.is_err());
        // Write file, rename and get
        let file_data = "test data\n";
        let mut reader = Cursor::new(file_data.as_bytes());
        assert!(stream.put_file("test.txt", &mut reader).await.is_ok());
        assert!(stream.rename("test.txt", "toast.txt").await.is_ok());
        assert!(stream.rm("toast.txt").await.is_ok());

        // assert!(stream.put_file("test.txt", &mut reader).is_ok());
        // // Append file
        // let mut reader = Cursor::new(file_data.as_bytes());
        // assert!(stream.append_file("test.txt", &mut reader).is_ok());
        // // Read file
        // let mut reader = stream.retr_as_stream("test.txt").ok().unwrap();
        // let mut buffer = Vec::new();
        // assert!(reader.read_to_end(&mut buffer).is_ok());
        // // Finalize
        // assert!(stream.finalize_retr_stream(Box::new(reader)).is_ok());
        // // Verify file matches
        // assert_eq!(buffer.as_slice(), "test data\ntest data\n".as_bytes());
        // // Rename
        // assert!(stream.rename("test.txt", "toast.txt").is_ok());
        // assert!(stream.rm("toast.txt").is_ok());

        // List directory again
        assert_eq!(stream.list(None).await.ok().unwrap().len(), 0);
        test_finalize_stream(stream).await;
    }

    #[maybe_async::maybe(sync(feature="sync", test), async(feature="async", async_attributes::test))]
    #[serial]
    async fn should_abort_transfer() {
        crate::log_init();
        let mut stream: FtpStream = test_setup_stream().await;
        // Set transfer type to Binary
        assert!(stream.transfer_type(FileType::Binary).await.is_ok());
        // cleanup
        let _ = stream.rm("test.bin").await;
        // put as stream
        let mut transfer_stream = stream.put_with_stream("test.bin").await.ok().unwrap();
        assert_eq!(
            transfer_stream
                .write(&[0x00, 0x01, 0x02, 0x03, 0x04])
                .await
                .ok()
                .unwrap(),
            5
        );
        // Abort
        assert!(stream.abort(transfer_stream).await.is_ok());
        // Check whether other commands still work after transfer
        assert!(stream.pwd().await.is_ok());
        // Check whether data channel still works
        assert!(stream.list(None).await.is_ok());
        // cleanup
        let _ = stream.rm("test.bin").await;
        test_finalize_stream(stream).await;
    }

    #[maybe_async::maybe(sync(feature="sync", test), async(feature="async", async_attributes::test))]
    #[serial]
    async fn should_resume_transfer() {
        crate::log_init();
        let mut stream: FtpStream = test_setup_stream().await;
        // Set transfer type to Binary
        assert!(stream.transfer_type(FileType::Binary).await.is_ok());
        // get dir
        let wrkdir = stream.pwd().await.ok().unwrap();
        // put as stream
        let mut transfer_stream = stream.put_with_stream("test.bin").await.ok().unwrap();
        assert_eq!(
            transfer_stream
                .write(&[0x00, 0x01, 0x02, 0x03, 0x04])
                .await
                .ok()
                .unwrap(),
            5
        );
        // Drop stream on purpose to simulate a failed connection
        drop(stream);
        drop(transfer_stream);
        // Re-connect to server
        let mut stream = test_setup_stream().await;
        assert!(stream.login(TEST_SERVER_LOGIN, TEST_SERVER_PASSWORD).await.is_ok());
        // Go back to previous dir
        assert!(stream.cwd(wrkdir).await.is_ok());
        // Set transfer type to Binary
        assert!(stream.transfer_type(FileType::Binary).await.is_ok());
        // Resume transfer
        assert!(stream.resume_transfer(5).await.is_ok());
        // Reopen stream
        let mut transfer_stream = stream.put_with_stream("test.bin").await.ok().unwrap();
        assert_eq!(
            transfer_stream
                .write(&[0x05, 0x06, 0x07, 0x08, 0x09, 0x0a])
                .await
                .ok()
                .unwrap(),
            6
        );
        // Finalize
        assert!(stream.finalize_put_stream(transfer_stream).await.is_ok());
        // Get size
        assert_eq!(stream.size("test.bin").await.ok().unwrap(), 11);
        // Remove file
        assert!(stream.rm("test.bin").await.is_ok());
        // Drop stream
        test_finalize_stream(stream).await;
    }

    // -- test utils

    #[maybe_async::maybe(sync(feature="sync"), async(feature="async"))]
    async fn test_setup_stream() -> FtpStream {
        let mut ftp_stream = FtpStream::connect(TEST_SERVER_ADDR).await.unwrap();
        assert!(ftp_stream.login(TEST_SERVER_LOGIN, TEST_SERVER_PASSWORD).await.is_ok());
        // Create wrkdir
        let tempdir: String = generate_tempdir();
        assert!(ftp_stream.mkdir(tempdir.as_str()).await.is_ok());
        // Change directory
        assert!(ftp_stream.cwd(tempdir.as_str()).await.is_ok());
        ftp_stream
    }

    #[maybe_async::maybe(sync(feature="sync"), async(feature="async"))]
    async fn test_finalize_stream(mut stream: FtpStream) {
        crate::log_init();
        // Get working directory
        let wrkdir: String = stream.pwd().await.ok().unwrap();
        // Remove directory
        assert!(stream.rmdir(wrkdir.as_str()).await.is_ok());
        assert!(stream.quit().await.is_ok());
    }

    fn generate_tempdir() -> String {
        let mut rng = thread_rng();
        let name: String = std::iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .map(char::from)
            .take(5)
            .collect();
        format!("temp_{}", name)
    }
}

}

