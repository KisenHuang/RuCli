use std::thread;
use std::time::Duration;

use crate::cli_error::CliError;
use crate::ability;
use crate::cli::cli_param::CliParam;

pub struct Deal {
    param: CliParam,
}

impl Deal {
    pub fn read_args() -> Result<Self, CliError> {
        let args = CliParam::new()?;
        Ok(Deal {
            param: args
        })
    }

    pub fn deal(&self) -> Result<(), CliError> {
        ability::run(&self.param)
    }
}