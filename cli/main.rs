//! # SuppaFTP Client
//!
//! This is a client you can install via `cargo install suppaftp` on your system to connect and work with FTP servers
//!

const SUPPAFTP_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const SUPPAFTP_VERSION: &str = env!("CARGO_PKG_VERSION");

// -- mods
mod actions;
mod command;
use actions::*;
use command::Command;
// -- locals

use suppaftp::{FtpError, FtpStream};
// -- ext
use std::env;
use std::io;
use std::io::Write;
use std::process::exit;
use std::str::FromStr;

fn usage() {
    println!(
        "suppaftp {} ~ Developed by {}",
        SUPPAFTP_VERSION, SUPPAFTP_AUTHORS
    );
    println!();
    println!("Available commands:");
    println!("APPE <file> <dest>                  Append content of local file `file` to `dest`");
    println!("CDUP                                Go to parent directory");
    println!("CONNECT <addr:port>                 Connect to remote host");
    println!("CONNECT+S <addr:port>               Connect to remote host using FTPS");
    println!("CWD <dir>                           Change working directory");
    println!("DELE <file>                         Remove file at specified path");
    println!("FEAT                                Get the feature list implemented by the server");
    println!("HELP                                Print this help");
    println!("LANG [lang]                         Language negotiation");
    println!("LIST [dir]                          List files. If directory is not provided, current directory is used");
    println!("LOGIN                               Login to remote");
    println!("MDTM <file>                         Get modification time for `file`");
    println!("MKDIR <dir>                         Make directory");
    println!("MODE <PASSIVE|ACTIVE>               Set mode");
    println!("NLST [path]                         Get the list of file names at specified path. If path is not provided list entries at current working directory");
    println!("NOOP                                Ping server");
    println!("MLSD [path]                         Lists the contents of a directory in a standardized machine-readable format");
    println!("MLST [path]                         Provides data about exactly the object named on its command line in a standardized machine-readable format");
    println!("OPTS <name> [value]                 Select options for a feature (for example OPTS UTF8 ON)");
    println!("PUT <file> <dest>                   Upload local file `file` to `dest`");
    println!("PWD                                 Print working directory");
    println!("QUIT                                Quit suppaftp");
    println!("RENAME <source> <dest>              Rename file `source` to `dest`");
    println!("RETR <file> <dest>                  Download `file` to `dest`");
    println!("RM <file>                           Remove file");
    println!("RMDIR <dir>                         Remove directory");
    println!("SITE <cmd>                          Sends site specific commands to remote server (like SITE IDLE 60 or SITE UMASK 002). Inspect SITE HELP output for complete list of supported commands");
    println!("SIZE <file>                         Get `file` size");
    println!();
    println!("Please, report issues to <https://github.com/veeso/suppaftp>");
    println!("Please, consider supporting the author <https://ko-fi.com/veeso>");
    println!();
}

fn input() -> Command {
    loop {
        print!(">> ");
        let _ = io::stdout().flush();
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read stdin");
        // Try to create command
        if let Ok(cmd) = Command::from_str(input.as_str()) {
            return cmd;
        }
        println!("Unknown command");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some("-h") = args.get(1).map(|x| x.as_str()) {
        usage();
        exit(255);
    }
    // init logger
    env_logger::init();
    // Main loop
    let mut ftp: Option<FtpStream> = None;
    loop {
        match input() {
            Command::Quit => {
                // Break if quit
                quit(ftp);
                break;
            }
            Command::Help => usage(),
            cmd => perform(&mut ftp, cmd),
        }
    }
}

fn perform(ftp: &mut Option<FtpStream>, command: Command) {
    match ftp {
        Some(ftp) => perform_connected(ftp, command),
        None => {
            if let Some(stream) = perform_uninitialized(command) {
                *ftp = Some(stream);
            }
        }
    }
}

fn perform_uninitialized(command: Command) -> Option<FtpStream> {
    match command {
        Command::Connect(remote, secure) => connect(remote.as_str(), secure),
        _ => {
            eprintln!("Can't perform command: you must connect to remote first");
            None
        }
    }
}

fn perform_connected(ftp: &mut FtpStream, command: Command) {
    match command {
        Command::Appe(src, dest) => appe(ftp, src.as_path(), dest.as_str()),
        Command::Cdup => cdup(ftp),
        Command::Connect(remote, secure) => {
            if let Some(stream) = connect(remote.as_str(), secure) {
                *ftp = stream;
            }
        }
        Command::Cwd(dir) => cwd(ftp, dir.as_str()),
        Command::Dele(file) => dele(ftp, file.as_str()),
        Command::Feat => feat(ftp),
        Command::Lang(l) => lang(ftp, l.as_deref()),
        Command::List(p) => list(ftp, p.as_deref()),
        Command::Login => login(ftp),
        Command::Mdtm(p) => mdtm(ftp, p.as_str()),
        Command::Mkdir(p) => mkdir(ftp, p.as_str()),
        Command::Mode(m) => set_mode(ftp, m),
        Command::Nlst(p) => nlst(ftp, p.as_deref()),
        Command::Noop => noop(ftp),
        Command::Mlsd(p) => mlsd(ftp, p.as_deref()),
        Command::Mlst(p) => mlst(ftp, p.as_deref()),
        Command::Opts(n, v) => opts(ftp, n.as_str(), v.as_deref()),
        Command::Put(src, dest) => put(ftp, src.as_path(), dest.as_str()),
        Command::Pwd => pwd(ftp),
        Command::Rename(src, dest) => rename(ftp, src.as_str(), dest.as_str()),
        Command::Retr(file, dest) => retr(ftp, file.as_str(), dest.as_path()),
        Command::Rm(file) => rm(ftp, file.as_str()),
        Command::Rmdir(file) => rmdir(ftp, file.as_str()),
        Command::Site(cmd) => site(ftp, cmd.as_str()),
        Command::Size(file) => size(ftp, file.as_str()),
        Command::Stat(path) => stat(ftp, path.as_deref()),
        Command::Help | Command::Quit => {
            panic!("Something unexpected happened")
        }
    }
}
