//! # Command
//!
//! The set of FTP commands

use crate::types::FileType;

use std::string::ToString;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Ftp commands with their arguments
pub enum Command {
    /// Abort an active file transfer
    Abor,
    /// Append to file
    Appe(String),
    /// Set auth to TLS
    #[cfg(feature = "_secure")]
    Auth,
    /// Ask server not to encrypt command channel
    #[cfg(feature = "_secure")]
    ClearCommandChannel,
    /// Change directory to parent directory
    Cdup,
    /// Change working directory
    Cwd(String),
    /// Remove file at specified path
    Dele(String),
    /// Get the feature list implemented by the server
    Feat,
    /// Language Negotiation
    Lang(Option<String>),
    /// List entries at specified path. If path is not provided list entries at current working directory
    List(Option<String>),
    /// Get modification time for file at specified path
    Mdtm(String),
    /// Make directory
    Mkd(String),
    /// Get the list of file names at specified path. If path is not provided list entries at current working directory
    Nlst(Option<String>),
    /// Ping server
    Noop,
    /// Lists the contents of a directory in a standardized machine-readable format
    Mlsd(Option<String>),
    /// Provides data about exactly the object named on its command line in a standardized machine-readable format
    Mlst(Option<String>),
    /// Select options for a feature (for example OPTS UTF8 ON)
    Opts(String, Option<String>),
    /// Provide login password
    Pass(String),
    /// Passive mode
    Pasv,
    /// Protection buffer size
    #[cfg(feature = "_secure")]
    Pbsz(usize),
    /// Specifies an address and port to which the server should connect (active mode)
    Port(String),
    /// Set protection level for protocol
    #[cfg(feature = "_secure")]
    Prot(ProtectionLevel),
    /// Print working directory
    Pwd,
    /// Quit
    Quit,
    /// Select file to rename
    RenameFrom(String),
    /// Rename selected file to
    RenameTo(String),
    /// Resume transfer from offset
    Rest(usize),
    /// Retrieve file
    Retr(String),
    /// Remove directory
    Rmd(String),
    /// Sends site specific commands to remote server (like SITE IDLE 60 or SITE UMASK 002). Inspect SITE HELP output for complete list of supported commands
    Site(String),
    /// Get file size of specified path
    Size(String),
    /// Returns information on the server status, including the status of the current connection
    Stat(Option<String>),
    /// Put file at specified path
    Store(String),
    /// Set transfer type
    Type(FileType),
    /// Provide user to login as
    User(String),
}

macro_rules! impl_command_new_str {
    ($cmd_name:ident, $fn_name:ident) => {
        pub fn $fn_name<S: AsRef<str>>(s: S) -> Self {
            Self::$cmd_name(s.as_ref().to_string())
        }
    };
}
macro_rules! impl_command_new_optstr {
    ($cmd_name:ident, $fn_name:ident) => {
        pub fn $fn_name<S: AsRef<str>>(s: Option<S>) -> Self {
            Self::$cmd_name(s.map(|s| s.as_ref().to_string()))
        }
    };
}
macro_rules! impl_command_new_str_optstr {
    ($cmd_name:ident, $fn_name:ident) => {
        pub fn $fn_name<S: AsRef<str>>(s: S, s2: Option<S>) -> Self {
            Self::$cmd_name(s.as_ref().to_string(), s2.map(|s| s.as_ref().to_string()))
        }
    };
}

impl Command {
    impl_command_new_str!(Cwd, new_cwd);
    impl_command_new_str!(Dele, new_dele);
    impl_command_new_optstr!(Lang, new_lang);
    impl_command_new_optstr!(List, new_list);
    impl_command_new_optstr!(Nlst, new_nlst);
    impl_command_new_str!(Mkd, new_mkd);
    impl_command_new_optstr!(Mlst, new_mlst);
    impl_command_new_optstr!(Mlsd, new_mlsd);
    impl_command_new_str!(Mdtm, new_mdtm);
    impl_command_new_str_optstr!(Opts, new_opts);
    impl_command_new_str!(Pass, new_pass);
    impl_command_new_str!(RenameFrom, new_rename_from);
    impl_command_new_str!(RenameTo, new_rename_to);
    impl_command_new_str!(Retr, new_retr);
    impl_command_new_str!(Rmd, new_rmd);
    impl_command_new_str!(Site, new_site);
    impl_command_new_str!(Size, new_size);
    impl_command_new_optstr!(Stat, new_stat);
    impl_command_new_str!(Store, new_store);
    impl_command_new_str!(User, new_user);
}

#[cfg(feature = "_secure")]
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(unused)]
/// Protection level; argument for `Prot` command
pub enum ProtectionLevel {
    Clear,
    Private,
}

// -- stringify

impl ToString for Command {
    fn to_string(&self) -> String {
        let mut s = match self {
            Self::Abor => "ABOR".to_string(),
            Self::Appe(f) => format!("APPE {}", f),
            #[cfg(feature = "_secure")]
            Self::Auth => "AUTH TLS".to_string(),
            Self::Cdup => "CDUP".to_string(),
            #[cfg(feature = "_secure")]
            Self::ClearCommandChannel => "CCC".to_string(),
            Self::Cwd(d) => format!("CWD {}", d),
            Self::Dele(f) => format!("DELE {}", f),
            Self::Feat => "FEAT".to_string(),
            Self::Lang(l) => match l {
                Some(l) => format!("LANG {}", l),
                None => "LANG".to_string(),
            },
            Self::List(p) => p
                .as_deref()
                .map(|x| format!("LIST {}", x))
                .unwrap_or_else(|| "LIST".to_string()),
            Self::Mdtm(p) => format!("MDTM {}", p),
            Self::Mlsd(p) => match p {
                Some(p) => format!("MLSD {}", p),
                None => "MLSD".to_string(),
            },
            Self::Mlst(p) => match p {
                Some(p) => format!("MLST {}", p),
                None => "MLST".to_string(),
            },
            Self::Mkd(p) => format!("MKD {}", p),
            Self::Nlst(p) => p
                .as_deref()
                .map(|x| format!("NLST {}", x))
                .unwrap_or_else(|| "NLST".to_string()),
            Self::Noop => "NOOP".to_string(),
            Self::Opts(s, s2) => match s2 {
                Some(s2) => format!("OPTS {} {}", s, s2),
                None => format!("OPTS {}", s),
            },
            Self::Pass(p) => format!("PASS {}", p),
            Self::Pasv => "PASV".to_string(),
            #[cfg(feature = "_secure")]
            Self::Pbsz(sz) => format!("PBSZ {}", sz),
            Self::Port(p) => format!("PORT {}", p),
            #[cfg(feature = "_secure")]
            Self::Prot(l) => format!("PROT {}", l.to_string()),
            Self::Pwd => "PWD".to_string(),
            Self::Quit => "QUIT".to_string(),
            Self::RenameFrom(p) => format!("RNFR {}", p),
            Self::RenameTo(p) => format!("RNTO {}", p),
            Self::Rest(offset) => format!("REST {}", offset),
            Self::Retr(p) => format!("RETR {}", p),
            Self::Rmd(p) => format!("RMD {}", p),
            Self::Site(p) => format!("SITE {}", p),
            Self::Size(p) => format!("SIZE {}", p),
            Self::Stat(p) => match p {
                Some(p) => format!("STAT {}", p),
                None => "STAT".to_string(),
            },
            Self::Store(p) => format!("STOR {}", p),
            Self::Type(t) => format!("TYPE {}", t.to_string()),
            Self::User(u) => format!("USER {}", u),
        };
        s.push_str("\r\n");
        s
    }
}

#[cfg(feature = "_secure")]
impl ToString for ProtectionLevel {
    fn to_string(&self) -> String {
        match self {
            Self::Clear => "C",
            Self::Private => "P",
        }
        .to_string()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_stringify_command() {
        assert_eq!(Command::Abor.to_string().as_str(), "ABOR\r\n");
        assert_eq!(
            Command::Appe(String::from("foobar.txt"))
                .to_string()
                .as_str(),
            "APPE foobar.txt\r\n"
        );
        #[cfg(feature = "_secure")]
        assert_eq!(Command::Auth.to_string().as_str(), "AUTH TLS\r\n");
        #[cfg(feature = "_secure")]
        assert_eq!(Command::ClearCommandChannel.to_string().as_str(), "CCC\r\n");
        assert_eq!(Command::Cdup.to_string().as_str(), "CDUP\r\n");
        assert_eq!(
            Command::Cwd(String::from("/tmp")).to_string().as_str(),
            "CWD /tmp\r\n"
        );
        assert_eq!(
            Command::Dele(String::from("a.txt")).to_string().as_str(),
            "DELE a.txt\r\n"
        );
        assert_eq!(
            Command::List(Some(String::from("/tmp")))
                .to_string()
                .as_str(),
            "LIST /tmp\r\n"
        );
        assert_eq!(Command::List(None).to_string().as_str(), "LIST\r\n");
        assert_eq!(
            Command::Mdtm(String::from("a.txt")).to_string().as_str(),
            "MDTM a.txt\r\n"
        );
        assert_eq!(
            Command::Mkd(String::from("/tmp")).to_string().as_str(),
            "MKD /tmp\r\n"
        );
        assert_eq!(
            Command::Nlst(Some(String::from("/tmp")))
                .to_string()
                .as_str(),
            "NLST /tmp\r\n"
        );
        assert_eq!(Command::Nlst(None).to_string().as_str(), "NLST\r\n");
        assert_eq!(Command::Noop.to_string().as_str(), "NOOP\r\n");
        assert_eq!(
            Command::Pass(String::from("qwerty123"))
                .to_string()
                .as_str(),
            "PASS qwerty123\r\n"
        );
        assert_eq!(Command::Pasv.to_string().as_str(), "PASV\r\n");
        #[cfg(feature = "_secure")]
        assert_eq!(Command::Pbsz(0).to_string().as_str(), "PBSZ 0\r\n");
        assert_eq!(
            Command::Port(String::from("0.0.0.0:21"))
                .to_string()
                .as_str(),
            "PORT 0.0.0.0:21\r\n"
        );
        #[cfg(feature = "_secure")]
        assert_eq!(
            Command::Prot(ProtectionLevel::Clear).to_string().as_str(),
            "PROT C\r\n"
        );
        assert_eq!(Command::Pwd.to_string().as_str(), "PWD\r\n");
        assert_eq!(Command::Quit.to_string().as_str(), "QUIT\r\n");
        assert_eq!(
            Command::RenameFrom(String::from("a.txt"))
                .to_string()
                .as_str(),
            "RNFR a.txt\r\n"
        );
        assert_eq!(
            Command::RenameTo(String::from("b.txt"))
                .to_string()
                .as_str(),
            "RNTO b.txt\r\n"
        );
        assert_eq!(Command::Rest(123).to_string().as_str(), "REST 123\r\n");
        assert_eq!(
            Command::Retr(String::from("a.txt")).to_string().as_str(),
            "RETR a.txt\r\n"
        );
        assert_eq!(
            Command::Rmd(String::from("/tmp")).to_string().as_str(),
            "RMD /tmp\r\n"
        );
        assert_eq!(
            Command::Size(String::from("a.txt")).to_string().as_str(),
            "SIZE a.txt\r\n"
        );
        assert_eq!(
            Command::Store(String::from("a.txt")).to_string().as_str(),
            "STOR a.txt\r\n"
        );
        assert_eq!(
            Command::Type(FileType::Binary).to_string().as_str(),
            "TYPE I\r\n"
        );
        assert_eq!(
            Command::User(String::from("omar")).to_string().as_str(),
            "USER omar\r\n"
        );
    }

    #[cfg(feature = "_secure")]
    #[test]
    fn should_stringify_protection_level() {
        assert_eq!(ProtectionLevel::Clear.to_string().as_str(), "C");
        assert_eq!(ProtectionLevel::Private.to_string().as_str(), "P");
    }
}
