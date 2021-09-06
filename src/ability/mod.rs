use crate::cli_error::CliError;
use crate::cli::cli_param::CliParam;

mod ability_length;
mod ability_cat;
mod ability_jni;

const LENGTH: &str = "length";
const CAT: &str = "cat";
const JNI: &str = "jni";

pub fn run(param: &CliParam) -> Result<(), CliError> {
    let pattern = &param.pattern;
    let ability = generate_pattern(pattern).ok_or_else(|| {
        CliError::custom("不支持指令")
    })?;
    ability.run(param.get_params())
}

fn generate_pattern(pattern: &str) -> Option<Box<dyn Ability>> {
    match pattern {
        LENGTH => {
            Some(Box::new(ability_length::create_ability()))
        }
        CAT => {
            Some(Box::new(ability_cat::create_ability()))
        }
        JNI => {
            Some(Box::new(ability_jni::create_ability()))
        }
        _ => {
            None
        }
    }
}

pub trait Ability {
    fn run(&self, params: Option<Vec<String>>) -> Result<(), CliError>;
}