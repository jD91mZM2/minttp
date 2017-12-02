use native_tls::Error as TlsError;
use native_tls::HandshakeError as TlsHandshakeError;
use std::error::Error as StdError;
use std::fmt;
use std::io::Error as IoError;
use std::net::TcpStream;
use std::num::ParseIntError;

/// minttp error type
#[derive(Debug)]
pub enum Error {
	InvalidHeader,
	InvalidStatusLine,
    IoError(IoError),
    ParseIntError(ParseIntError),
    TlsError(TlsError),
    TlsHandshakeError(TlsHandshakeError<TcpStream>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidHeader       => write!(f, "{}", self.description()),
            Error::InvalidStatusLine   => write!(f, "{}", self.description()),
            Error::IoError(ref inner)  => write!(f, "{}", inner),
            Error::ParseIntError(ref inner) => write!(f, "{}", inner),
            Error::TlsError(ref inner) => write!(f, "{}", inner),
            Error::TlsHandshakeError(ref inner) => write!(f, "{}", inner)
        }
    }
}
impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::InvalidHeader       => "Response parsing error: Invalid header",
            Error::InvalidStatusLine   => "Response parsing error: Invalid status line",
            Error::IoError(ref inner)  => inner.description(),
            Error::ParseIntError(ref inner) => inner.description(),
            Error::TlsError(ref inner) => inner.description(),
            Error::TlsHandshakeError(ref inner) => inner.description()
        }
    }
    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::IoError(ref inner)  => Some(inner),
            Error::ParseIntError(ref inner) => Some(inner),
            Error::TlsError(ref inner) => Some(inner),
            Error::TlsHandshakeError(ref inner) => Some(inner),
            _ => None
        }
    }
}
impl From<TlsError> for Error {
    fn from(error: TlsError) -> Self { Error::TlsError(error) }
}
impl From<IoError> for Error {
    fn from(error: IoError) -> Self { Error::IoError(error) }
}
impl From<TlsHandshakeError<TcpStream>> for Error {
    fn from(error: TlsHandshakeError<TcpStream>) -> Self { Error::TlsHandshakeError(error) }
}
impl From<ParseIntError> for Error {
    fn from(error: ParseIntError) -> Self { Error::ParseIntError(error) }
}
