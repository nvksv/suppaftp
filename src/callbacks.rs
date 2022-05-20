use crate::types::{FileType, FtpError, FtpResult, Mode, Response};

pub trait FtpStreamCallbacks {
    fn welcome_response(&mut self, response: Response);
}

pub type FtpStreamCallbacksRef = Box<dyn FtpStreamCallbacks>;