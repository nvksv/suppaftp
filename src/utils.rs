use super::types::{FtpError, FtpResult};
use super::Status;
use regex::Regex;
use std::net::{SocketAddr, ToSocketAddrs};


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
    
    let (code_len, delim) = line_iter.nth(CODE_LENGTH).ok_or_else(|| FtpError::BadResponse)?;
    let (tail_off, _) = line_iter.nth(0).ok_or_else(|| FtpError::BadResponse)?;

    let code: u32 = line[..code_len].parse().map_err(|_| FtpError::BadResponse)?;
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


pub trait ToSocketAddrsWithDefaultPort {
    type Iter: Iterator<Item = SocketAddr>;
    fn to_socket_addrs(&self) -> Result<Self::Iter>;
    fn default_port() -> u16;
}

macro_rules! std_impl {
    ($ty:ty) => {
        impl ToSocketAddrsWithDefaultPort for $ty {
            type Iter = <Self as ToSocketAddrs>::Iter;
            fn to_socket_addrs(&self) -> Result<Self::Iter> {
                <Self as ToSocketAddrs>::to_socket_addrs(self)
            } 
        }
    }
}

macro_rules! tuple_impl {
    ($ty:ty) => {
    impl ToSocketAddrsWithDefaultPort for $ty {
        type Iter = <(Self, u16) as ToSocketAddrs>::Iter;
        fn to_socket_addrs(&self) -> Result<Self::Iter> {
            <(Self, u16) as ToSocketAddrs>::to_socket_addrs((self, self.default_port()))
        } 
    }
}

std_impl!(SocketAddr);
std_impl!((&str, u16));
std_impl!((IpAddr, u16));
std_impl!((String, u16));
std_impl!((Ipv4Addr, u16));
std_impl!((Ipv6Addr, u16));
std_impl!(SocketAddrV4);
std_impl!(SocketAddrV6);

tuple_impl!(IpAddr);
tuple_impl!(Ipv4Addr);
tuple_impl!(Ipv6Addr);


impl ToSocketAddrs for str {

}
impl ToSocketAddrs for String


impl<'a> ToSocketAddrsWithDefaultPort for &'a [SocketAddr] {
    type Iter = <Self as ToSocketAddrs>::Iter;
    fn to_socket_addrs(&self) -> Result<Self::Iter> {
        <Self as ToSocketAddrs>::to_socket_addrs(self)
    } 
}
impl<T: ToSocketAddrs + ?Sized> ToSocketAddrsWithDefaultPort for &T {
    type Iter = <Self as ToSocketAddrs>::Iter;
    fn to_socket_addrs(&self) -> Result<Self::Iter> {
        <Self as ToSocketAddrs>::to_socket_addrs(self)
    } 
}