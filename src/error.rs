use crate::data::Position;
use annotate_snippets::formatter::DisplayListFormatter;
use annotate_snippets::snippet::*;

#[derive(Debug)]
pub struct ContextualError {
    pub(crate) snippet: Snippet,
}

impl ContextualError {
    pub(crate) fn new(error: &str) -> ContextualError {
        ContextualError {
            snippet: Snippet {
                title: Some(Annotation {
                    id: None,
                    label: Some(error.to_owned()),
                    annotation_type: AnnotationType::Error,
                }),
                footer: Vec::new(),
                slices: Vec::new(),
            },
        }
    }
}

impl std::fmt::Display for ContextualError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "contextual error")
    }
}

impl std::error::Error for ContextualError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Contextual(ContextualError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Io(err) => write!(f, "{}", err),
            Self::Contextual(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            Self::Contextual(err) => Some(err),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<ContextualError> for Error {
    fn from(err: ContextualError) -> Self {
        Self::Contextual(err)
    }
}
