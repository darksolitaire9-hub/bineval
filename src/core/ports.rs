use crate::core::primitive::Primitive;
use crate::core::suite::Suite;
use crate::core::BinevalError;

pub trait PrimitiveRepositoryPort {
    fn list_primitives(&self) -> Result<Vec<Primitive>, BinevalError>;
}

pub trait SuiteRepositoryPort {
    fn list_suites(&self) -> Result<Vec<Suite>, BinevalError>;
    fn get_suite(&self, id: &str) -> Result<Suite, BinevalError>;
}

pub trait LoggerPort {
    fn info(&self, msg: &str);
    fn warn(&self, msg: &str);
    fn error(&self, msg: &str);
}
