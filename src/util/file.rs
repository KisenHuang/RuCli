use std::fs::File;
use std::io::Read;
use crate::util::CliError;
use std::fs::{read_to_string, create_dir_all, write};

pub fn read_file(path: &str) -> Result<String, CliError> {
    // let mut result_str = String::new();
    // let mut f = File::open(path).or_else(|err|{
    //     Err(CliError::io(err,"文件不存在"))
    // })?;
    // f.read_to_string(&mut result_str).or_else(|err|{
    //     Err(CliError::io(err,"文件异常"))
    // })?;
    // Ok(result_str)
    read_to_string(path).or_else(|err| {
        Err(CliError::io(err, ""))
    })
}

pub fn write_file(file: &str, content: &str) -> Result<(), CliError> {
    write(file, content).or_else(|e| {
        Err(CliError::io(e, "写入文件失败"))
    })
}

pub fn create_dir(dir: &str) -> Result<(), CliError> {
    create_dir_all(dir).or_else(|e| {
        Err(CliError::io(e, "创建文件目录失败"))
    })
}

pub fn get_file_parent(file: &str) -> Result<String, CliError> {
    let index = file.rfind("/").ok_or_else(|| {
        CliError::custom("文件路径错误")
    })?;
    let (path, file) = file.split_at(index);

    Ok(path.to_string().clone())
}