use crate::data::SpannedValue;
use crate::error::ContextualError;

pub trait Validator {
    fn validate(&self, value: &SpannedValue) -> Result<(), ContextualError>;
}

pub struct NoSchema;

impl NoSchema {
    pub fn new() -> Self {
        Self
    }
}

impl Validator for NoSchema {
    fn validate(&self, _value: &SpannedValue) -> Result<(), ContextualError> {
        Ok(())
    }
}
