//! # Types
//!
//! The set of valid values for FTP commands

use super::Status;
use std::convert::From;
use std::fmt;
use thiserror::Error;

////////////////////////////////////////////////////////////////////////////////

/// A shorthand for a Result whose error type is always an FtpError.
pub type FtpResult<T> = std::result::Result<T, FtpError>;

////////////////////////////////////////////////////////////////////////////////

/// `FtpError` is a library-global error type to describe the different kinds of
/// errors that might occur while using FTP.
#[derive(Debug, Error)]
pub enum FtpError {

    /// Connection error
    #[error("Connection error: {0}")]
    ConnectionError(std::io::Error),
    
    /// There was an error with the secure stream
    #[cfg(feature = "_secure")]
    #[error("Secure error: {0}")]
    SecureError(String),
    
    /// Unexpected response from remote. The command expected a certain response, but got another one.
    /// This means the ftp server refused to perform your request or there was an error while processing it.
    /// Contains the response data.
    #[error("Invalid response: {0}")]
    UnexpectedResponse(Response),
    
    /// The response syntax is invalid
    #[error("Response contains an invalid syntax")]
    BadResponse,
    
    /// The address provided was invalid
    #[error("Invalid address: {0}")]
    InvalidAddress(std::net::AddrParseError),
    
    /// 500 Syntax error, command unrecognized (this may include errors such as command line too long).
    /// 502 Command not implemented.
    /// 503 Bad sequence of commands.
    #[error("Bad command")]
    BadCommand{ status: Status, message: String },

    /// 501 Syntax error in parameters or arguments.
    /// 504 Command not implemented for that parameter.    
    #[error("Bad parameter")]
    BadParameter{ status: Status, message: String },
}

impl FtpError {
    pub fn is_recoverable(&self) -> bool {
        match self {
            FtpError::ConnectionError(ioe) => {
                match ioe.kind() {
                    std::io::ErrorKind::ConnectionRefused |
                    std::io::ErrorKind::ConnectionReset |
                    std::io::ErrorKind::ConnectionAborted |
                    std::io::ErrorKind::NotConnected => { true },
                    _ => { false },
                }
            },

            #[cfg(feature = "_secure")]
            FtpError::SecureError(_) => { true },

            _ => { false },
        }
    }
}

impl From<std::io::Error> for FtpError {
    fn from(e: std::io::Error) -> Self {
        Self::ConnectionError(e)
    }
}

impl From<std::net::AddrParseError> for FtpError {
    fn from(e: std::net::AddrParseError) -> Self {
        Self::InvalidAddress(e)
    }
}

#[cfg(feature = "sync-secure")]
impl<S: 'static> From<native_tls::HandshakeError<S>> for FtpError
where
    S: std::io::Read + std::io::Write + std::fmt::Debug
{
    fn from(e: native_tls::HandshakeError<S>) -> Self {
        FtpError::SecureError(format!("{}", e))
    }
}

#[cfg(feature = "async-secure")]
impl From<async_native_tls::Error> for FtpError
{
    fn from(e: async_native_tls::Error) -> Self {
        FtpError::SecureError(format!("{}", e))
    }
}

////////////////////////////////////////////////////////////////////////////////

/// Defines a response body from the ftp server
#[derive(Clone, Debug, Error)]
pub enum ResponseBody {
    #[error("{body}")]
    Inline { 
        body: String 
    },
    #[error("{body:?}")]
    Multiline {
        head: String, 
        body: Vec<String>, 
        tail: String, 
    },
}

impl ResponseBody {
    fn join_vec(v: &Vec<String>) -> String {
        v.join("\r\n")
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Inline{ body } => {
                body.clone()
            },
            Self::Multiline{ body, .. } => {
                Self::join_vec( body )
            }
        }
    }

    pub fn into_string(self) -> String {
        match self {
            Self::Inline{ body } => {
                body
            },
            Self::Multiline{ body, .. } => {
                Self::join_vec( &body )
            }
        }
    }

    pub fn to_vec(&self) -> Vec<String> {
        match self {
            Self::Inline{ body } => {
                vec![body.clone()]
            },
            Self::Multiline{ body, .. } => {
                body.iter().cloned().collect()
            }
        }
    }

    pub fn into_vec(self) -> Vec<String> {
        match self {
            Self::Inline{ body } => {
                vec![body]
            },
            Self::Multiline{ body, .. } => {
                body
            }
        }
    }

    pub fn as_inline(&self) -> Option<&String> {
        match self {
            Self::Inline{ body } => {
                Some(body)
            },
            Self::Multiline{ .. } => {
                None
            }
        }        
    }

    pub fn into_inline(self) -> Option<String> {
        match self {
            Self::Inline{ body } => {
                Some(body)
            },
            Self::Multiline{ .. } => {
                None
            }
        }        
    }

    pub fn as_multiline(&self) -> Option<&Vec<String>> {
        match self {
            Self::Inline{ .. } => {
                None
            },
            Self::Multiline{ body, .. } => {
                Some(body)
            }
        }        
    }

    pub fn into_multiline(self) -> Option<Vec<String>> {
        match self {
            Self::Inline{ .. } => {
                None
            },
            Self::Multiline{ body, .. } => {
                Some(body)
            }
        }        
    }
}

////////////////////////////////////////////////////////////////////////////////

/// Defines a response from the ftp server
#[derive(Clone, Debug, Error)]
pub struct Response {
    pub status: Status,
    pub body: ResponseBody,
}

impl Response {
    /// ### new
    ///
    /// Instantiates a new `Response`
    pub fn new_inline<S: AsRef<str>>(status: Status, body: S) -> Self {
        Self {
            status,
            body: ResponseBody::Inline { body: body.as_ref().to_string() },
        }
    }

    pub fn new_multiline(status: Status, head: String, body: Vec<String>, tail: String) -> Self {
        Self {
            status,
            body: ResponseBody::Multiline { head, body, tail },
        }
    }

    pub(crate) fn body_as_inline_result(&self) -> FtpResult<&String> {
        self.body
            .as_inline()
            .ok_or_else(|| FtpError::UnexpectedResponse(self.clone()))
    }

    #[allow(dead_code)]    
    pub(crate) fn body_into_inline_result(self) -> FtpResult<String> {
        match &self.body {
            ResponseBody::Inline{..} => {
                if let ResponseBody::Inline{body} = self.body {
                    Ok(body)
                } else {
                    unreachable!()
                }
            },
            _ => {
                Err(FtpError::UnexpectedResponse(self))
            }
        }
    }

    pub(crate) fn body_into_multiline_result(self) -> FtpResult<Vec<String>> {
        match &self.body {
            ResponseBody::Multiline{..} => {
                if let ResponseBody::Multiline{body, ..} = self.body {
                    Ok(body)
                } else {
                    unreachable!()
                }
            },
            _ => {
                Err(FtpError::UnexpectedResponse(self))
            }
        }
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.status.code(), self.body)
    }
}

////////////////////////////////////////////////////////////////////////////////

/// Text Format Control used in `TYPE` command
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FormatControl {
    /// Default text format control (is NonPrint)
    Default,
    /// Non-print (not destined for printing)
    NonPrint,
    /// Telnet format control (\<CR\>, \<FF\>, etc.)
    Telnet,
    /// ASA (Fortran) Carriage Control
    Asa,
}

impl ToString for FormatControl {
    fn to_string(&self) -> String {
        match self {
            FormatControl::Default | FormatControl::NonPrint => String::from("N"),
            FormatControl::Telnet => String::from("T"),
            FormatControl::Asa => String::from("C"),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

/// File Type used in `TYPE` command
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FileType {
    /// ASCII text (the argument is the text format control)
    Ascii(FormatControl),
    /// EBCDIC text (the argument is the text format control)
    Ebcdic(FormatControl),
    /// Image,
    Image,
    /// Binary (the synonym to Image)
    Binary,
    /// Local format (the argument is the number of bits in one byte on local machine)
    Local(u8),
}

impl ToString for FileType {
    fn to_string(&self) -> String {
        match self {
            FileType::Ascii(fc) => format!("A {}", fc.to_string()),
            FileType::Ebcdic(fc) => format!("E {}", fc.to_string()),
            FileType::Image | FileType::Binary => String::from("I"),
            FileType::Local(bits) => format!("L {}", bits),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

/// Connection mode for data channel
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Passive,
    Active,
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn fmt_error() {
        assert_eq!(
            FtpError::ConnectionError(std::io::Error::new(std::io::ErrorKind::NotFound, "omar"))
                .to_string()
                .as_str(),
            "Connection error: omar"
        );
        #[cfg(feature = "_secure")]
        assert_eq!(
            FtpError::SecureError("omar".to_string())
                .to_string()
                .as_str(),
            "Secure error: omar"
        );
        assert_eq!(
            FtpError::UnexpectedResponse(Response::new_inline(Status::ExceededStorage, "error"))
                .to_string()
                .as_str(),
            "Invalid response: [552] error"
        );
        assert_eq!(
            FtpError::BadResponse.to_string().as_str(),
            "Response contains an invalid syntax"
        );
    }

    #[test]
    fn response() {
        let response: Response = Response::new_inline(Status::AboutToSend, "error");
        assert_eq!(response.status, Status::AboutToSend);
        assert_eq!(response.body.to_string().as_str(), "error");
    }

    #[test]
    fn fmt_response() {
        let response: Response = Response::new_inline(
            Status::FileUnavailable,
            "Can't create directory: File exists",
        );
        assert_eq!(
            response.to_string().as_str(),
            "[550] Can't create directory: File exists"
        );
    }

    #[test]
    fn fmt_format_control() {
        assert_eq!(FormatControl::Asa.to_string().as_str(), "C");
        assert_eq!(FormatControl::Telnet.to_string().as_str(), "T");
        assert_eq!(FormatControl::Default.to_string().as_str(), "N");
        assert_eq!(FormatControl::NonPrint.to_string().as_str(), "N");
    }

    #[test]
    fn fmt_file_type() {
        assert_eq!(
            FileType::Ascii(FormatControl::Telnet).to_string().as_str(),
            "A T"
        );
        assert_eq!(FileType::Binary.to_string().as_str(), "I");
        assert_eq!(FileType::Image.to_string().as_str(), "I");
        assert_eq!(
            FileType::Ebcdic(FormatControl::Telnet).to_string().as_str(),
            "E T"
        );
        assert_eq!(FileType::Local(2).to_string().as_str(), "L 2");
    }
}
