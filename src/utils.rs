use super::types::{FtpError, FtpResult};
use super::Status;
use regex::Regex;
//use std::net::{SocketAddr, ToSocketAddrs};


lazy_static! {
    // This regex extracts IP and Port details from PASV command response.
    // The regex looks for the pattern (h1,h2,h3,h4,p1,p2).
    pub static ref PORT_RE: Regex = Regex::new(r"\((\d+),(\d+),(\d+),(\d+),(\d+),(\d+)\)").unwrap();

    // This regex extracts modification time from MDTM command response.
    pub static ref MDTM_RE: Regex = Regex::new(r"\b(\d{4})(\d{2})(\d{2})(\d{2})(\d{2})(\d{2})\b").unwrap();

    // This regex extracts file size from SIZE command response.
    pub static ref SIZE_RE: Regex = Regex::new(r"(\d+)\s*$").unwrap();
}

pub fn parse_status_delim_tail( line: &str ) -> FtpResult<(Status, char, String)> {
    let mut line_iter = line.char_indices();
    
    let (code_len, delim) = line_iter.nth(CODE_LENGTH).ok_or_else(|| {debug!("parse_status_delim_tail failed 1"); FtpError::BadResponse})?;
    let (tail_off, _) = line_iter.nth(0).ok_or_else(|| {debug!("parse_status_delim_tail failed 2"); FtpError::BadResponse})?;

    let code: u32 = line[..code_len].parse().map_err(|err| {debug!("parse_status_delim_tail failed 3: {}", err); FtpError::BadResponse})?;
    let tail: String = line[tail_off..].trim_end().to_string(); 

    Ok((code.into(), delim, tail))
}

pub const CODE_LENGTH: usize = 3;
pub const SPACE_CHAR: char = ' ';
pub const MINUS_CHAR: char = '-';
    
pub fn optstrref<S: AsRef<str>>(s: &Option<S>) -> &str {
    match s { 
        Some(ref s) => s.as_ref(), 
        None => "" 
    }
}


