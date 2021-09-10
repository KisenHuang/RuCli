use crate::util::{CliError, file::read_file};
use crate::ability::Ability;
use log::info;

pub struct CAT {}

impl Ability for CAT {
    fn run(&self, params: Option<Vec<String>>) -> Result<(), CliError> {
        let vec = params.ok_or_else(|| {
            CliError::custom("参数为空")
        })?;
        if vec.len() < 1 {
            return Err(CliError::custom("参数为空"));
        }
        let param = &vec[0];
        info!("File:{}", param);
        info!("{}", read_file(param)?);
        Ok(())
    }
}

pub fn create_ability() -> CAT {
    CAT {}
}
