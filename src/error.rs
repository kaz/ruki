pub type Error = Box<dyn std::error::Error>;
pub type Result<R> = std::result::Result<R, Error>;

pub struct InternalError {
    message: String,
    source: Option<Error>,
}

impl std::error::Error for InternalError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.source {
            Some(e) => Some(e.as_ref()),
            _ => None,
        }
    }
}

impl InternalError {
    pub fn new<S>(msg: S) -> Error
    where
        S: Into<String>,
    {
        Box::new(InternalError {
            message: msg.into(),
            source: None,
        })
    }
    pub fn wrap<E, S>(err: E, msg: S) -> Error
    where
        E: std::error::Error + 'static,
        S: Into<String>,
    {
        Box::new(InternalError {
            message: msg.into(),
            source: Some(Box::new(err)),
        })
    }
}

macro_rules! implement {
    ($trait:ident) => {
        impl std::fmt::$trait for InternalError {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.message)
            }
        }
    };
}

implement!(Debug);
implement!(Display);
