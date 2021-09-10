use std::fmt::{Display, Formatter};
//命令参数获取工具
use structopt::StructOpt;
use crate::util::CliError;

///存储参数
// #[derive(Debug, StructOpt, Clone)]
// #[structopt()]
#[derive(Debug, Clone)]
pub struct CliParam {
    pub pattern: String,
    params: Option<Vec<String>>,
}

impl std::fmt::Display for CliParam {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "CliParam [pattern: {}, param: {}]", self.pattern, {
            match &self.params {
                None => { "".to_string() }
                Some(vec_param) => {
                    let mut p = String::new();
                    for param in vec_param {
                        p = format!("{} {}", p, param)
                    }
                    p
                }
            }
        })
    }
}

impl CliParam {
    pub fn new() -> Result<Self, CliError> {
        // CliParam::from_args()
        let mut args = std::env::args();
        let pattern = args.nth(1).ok_or_else(|| {
            CliError::custom("参数异常：无指令")
        })?;
        Ok(CliParam {
            pattern,
            params: {
                let mut vec = Vec::new();
                while let Ok(p) = args.next().ok_or(|| CliError::custom("")) {
                    vec.push(p);
                }
                if vec.is_empty() {
                    None
                } else {
                    Some(vec)
                }
            },
        })
    }

    pub fn get_params(&self) -> Option<Vec<String>> {
        self.params.clone()
            // .map_or(None, |vec| {
            //     if vec.is_empty() {
            //         None
            //     } else {
            //         Some(vec)
            //     }
            // })
    }
}


//fn read_param() {
//     let mut args = std::env::args();
//     let mut i = 1;
//     loop {
//         println!("查询参数-位置：{}", i);
//         match args.nth(i) {
//             None => {
//                 break;
//             }
//             Some(s) => {
//                 println!("输入参数：{}", s);
//             }
//         };
//         i += 1;
//     }
//     println!("打印参数结束");
// }