use crate::core::ports::{LoggerPort, SuiteRepositoryPort};
use crate::core::BinevalError;

pub struct ValidateUseCase<'a> {
    pub suites: &'a dyn SuiteRepositoryPort,
    pub logger: &'a dyn LoggerPort,
}

impl<'a> ValidateUseCase<'a> {
    pub fn run(&self) -> Result<(), BinevalError> {
        // Attempt to load all suites to ensure they parse correctly
        let suites = self.suites.list_suites()?;

        self.logger
            .info(&format!("Successfully validated {} suites.", suites.len()));
        Ok(())
    }
}
