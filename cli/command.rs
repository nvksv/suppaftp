use std::path::PathBuf;
use std::str::FromStr;
use suppaftp::Mode;

pub enum Command {
    Appe(PathBuf, String),
    Cdup,
    Connect(String, bool),
    Cwd(String),
    Dele(String),
    Feat,
    Help,
    Lang(Option<String>),
    List(Option<String>),
    Login,
    Mdtm(String),
    Mkdir(String),
    Mode(Mode),
    Nlst(Option<String>),
    Noop,
    Mlsd(Option<String>),
    Mlst(Option<String>),
    Opts(String, Option<String>),
    Put(PathBuf, String),
    Pwd,
    Quit,
    Rename(String, String),
    Retr(String, PathBuf),
    Rm(String),
    Rmdir(String),
    Site(String),
    Size(String),
    Stat(Option<String>),
}

impl FromStr for Command {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split string by space
        let mut args = s.split_ascii_whitespace();
        // Match args
        let cmd: String = args.next().ok_or("Unknown command")?.to_ascii_uppercase();
        let command = match cmd.as_str() {
            "APPE" => {
                let local: PathBuf = args.next().ok_or("Missing `source` field")?.into();
                let dest: String = args.next().ok_or("Missing `dest` field")?.into();
                Self::Appe(local, dest)
            },
            "CDUP" => {
                Self::Cdup
            },
            "CONNECT" => {
                let addr: String = args.next().ok_or("Missing `addr` field")?.into();
                Self::Connect(addr, false)
            },
            "CONNECT+S" => {
                let addr: String = args.next().ok_or("Missing `addr` field")?.into();
                Self::Connect(addr, true)
            },
            "CWD" => {
                let dir: String = args.next().ok_or("Missing `dir` field")?.into();
                Self::Cwd(dir)
            },
            "DELE" => {
                let file: String = args.next().ok_or("Missing `file` field")?.into();
                Self::Dele(file)
            },
            "FEAT" => {
                Self::Feat
            },
            "HELP" => {
                Self::Help
            },
            "LANG" => {
                let lang: Option<String> = args.next().map(String::from);
                Self::Lang(lang)
            },
            "LIST" => {
                let dir: Option<String> = args.next().map(String::from);
                Self::List(dir)
            },
            "LOGIN" => {
                Self::Login
            },
            "MDTM" => {
                let file: String = args.next().ok_or("Missing `file` field")?.into();
                Self::Mdtm(file)
            },
            "MKDIR" => {
                let dir: String = args.next().ok_or("Missing `dir` field")?.into();
                Self::Mkdir(dir)
            },
            "MODE" => {
                match args.next() {
                    Some("ACTIVE") => Self::Mode(Mode::Active),
                    Some("PASSIVE") => Self::Mode(Mode::Passive),
                    Some(_) => return Err("Invalid mode"),
                    None => return Err("Missing `mode` field"),
                }
            },
            "NLST" => {
                let dir: Option<String> = args.next().map(String::from);
                Self::Nlst(dir)
            },
            "NOOP" => {
                Self::Noop
            },
            "MLSD" => {
                let dir: Option<String> = args.next().map(String::from);
                Self::Mlsd(dir)
            },
            "MLST" => {
                let dir: Option<String> = args.next().map(String::from);
                Self::Mlst(dir)
            },
            "OPTS" => {
                let name: String = args.next().ok_or("Missing `name` field")?.into();
                let value: Option<String> = args.next().map(String::from);
                Self::Opts(name, value)
            },
            "PUT" => {
                let source: PathBuf = args.next().ok_or("Missing `source` field")?.into();
                let dest: String = args.next().ok_or("Missing `dest` field")?.into();
                Self::Put(source, dest)
            },
            "PWD" => {
                Self::Pwd
            },
            "QUIT" => {
                Self::Quit
            },
            "RENAME" => {
                let source: String = args.next().ok_or("Missing `source` field")?.into();
                let dest: String = args.next().ok_or("Missing `dest` field")?.into();
                Self::Rename(source, dest)
            },
            "RETR" => {
                let source: String = args.next().ok_or("Missing `file` field")?.into();
                let dest: PathBuf = args.next().ok_or("Missing `dest` field")?.into();
                Self::Retr(source, dest)
            },
            "RM" => {
                let file: String = args.next().ok_or("Missing `file` field")?.into();
                Self::Rm(file)
            },
            "RMDIR" => {
                let dir: String = args.next().ok_or("Missing `dir` field")?.into();
                Self::Rmdir(dir)
            },
            "SITE" => {
                let cmd: String = args.next().ok_or("Missing `cmd` field")?.into();
                Self::Site(cmd)
            },
            "SIZE" => {
                let file: String = args.next().ok_or("Missing `file` field")?.into();
                Self::Size(file)
            },
            "STAT" => {
                let path: Option<String> = args.next().map(String::from);
                Self::Stat(path)
            },
            _ => {
                return Err("Unknown command")
            }
        };
        Ok(command)
    }
}
