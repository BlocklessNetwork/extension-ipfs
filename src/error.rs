use std::fmt::Display;

#[derive(Debug)]
pub enum ErrorKind {
    RequestError,
    BodyError,
    JsonError,
    HandleError,
    ParameterError,
    Unsupport,
    Others,
}

impl From<&ErrorKind> for &'static str {
    fn from(k: &ErrorKind) -> &'static str {
        match k {
            &ErrorKind::Others => "others",
            &ErrorKind::BodyError => "body error",
            &ErrorKind::JsonError => "json error",
            &ErrorKind::HandleError => "handle error",
            &ErrorKind::ParameterError => "parameter error",
            &ErrorKind::Unsupport => "unsupport error",
            &ErrorKind::RequestError => "request error",
        }
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.into())
    }
}
