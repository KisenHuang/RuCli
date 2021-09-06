use crate::cli_error::CliError;
use crate::ability::Ability;
use log::info;

pub struct Length {}

impl Ability for Length {
    fn run(&self, params: Option<Vec<String>>) -> Result<(), CliError> {
        let vec = params.ok_or_else(|| {
            CliError::custom("参数为空")
        })?;
        if vec.len() < 1 {
            return Err(CliError::custom("参数为空"));
        }
        let param = &vec[0];
        info!("length:{}", param);
        Ok(())
    }
}

pub fn create_ability() -> Length {
    Length {}
}
