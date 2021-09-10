use crate::ability::Ability;
use crate::util::CliError;
use crate::util::file::{read_file, create_dir, write_file, get_file_parent};
use log::info;
use regex::Regex;
use std::fmt::{Display, Formatter};
use std::io::Write;
use std::collections::HashMap;

struct JniApi {
    package: String,
    class: String,
    method: String,
    params: Vec<(String, String)>,
    return_value: String,
}

pub struct Jni {}

impl Ability for Jni {
    fn run(&self, params: Option<Vec<String>>) -> Result<(), CliError> {
        let vec = params.ok_or_else(|| {
            CliError::custom("参数为空")
        })?;
        if vec.len() < 1 {
            return Err(CliError::custom("参数为空"));
        }

        let source = vec.get(0).ok_or_else(||
            CliError::custom("缺少需要操作的文件")
        )?;

        // 读取文件内容
        let file_source = read_file(source)?;
        // 解析原文件
        let jni_api = self.analysis(file_source)?;
        // 获取目标文件存放目录
        let target = match vec.get(1) {
            None => { "./" }
            Some(s) => { s }
        };
        // 创建目标文件
        self.create_target_and_write(jni_api, target)?;

        Ok(())
    }
}

impl Jni {
    fn analysis(&self, content: String) -> Result<Vec<JniApi>, CliError> {
        let mut vec = Vec::new();
        let result = Regex::new(" fn Java_\\w+\\([^{]+\\{").unwrap();
        for matched in result.find_iter(&content) {
            let content = matched.as_str();
            vec.push(JniApi::new(content));
        }
        Ok(vec)
    }

    fn create_target_and_write(&self, apis: Vec<JniApi>, target: &str) -> Result<(), CliError> {
        info!("target:{}", target);

        let target =
            if !target.ends_with("/") {
                format!("{}/", target)
            } else {
                target.to_string().clone()
            };

        // let mut vec_api: HashMap<String, Vec<JniApi>> = HashMap::new();
        let mut vec_api: HashMap<String, (String, String, Vec<String>)> = HashMap::new();

        for api in apis {
            println!("{}", api);

            let java_path = format!("{}{}/{}.java", target, api.package, api.class);
            if vec_api.contains_key(&java_path) {
                let mut list = vec_api.get_mut(&java_path).unwrap();
                (*list).2.push(api.get_method_str());
                // list.push(api);
            } else {
                let vec = vec![api.get_method_str()];
                vec_api.insert(java_path, (api.package.clone(), api.class.clone(), vec));
            }
        }

        for (path, (pkg, class, apis)) in vec_api {
            // let mut methods = Vec::new();
            // for api in apis {
            //     methods.push(api.get_method_str())
            // }
            java_file_create(&path, &pkg, &class, apis)?;
        }

        Ok(())
    }
}

impl JniApi {
    fn new(content: &str) -> Self {
        let (pkg, class, method) = JniApi::deal_method(content);

        let params = JniApi::deal_params(content);

        let return_value = JniApi::deal_return_value(content);

        JniApi {
            package: pkg.clone(),
            class: class.clone(),
            method: method.clone(),
            params,
            return_value,
        }
    }

    fn deal_method(content: &str) -> (String, String, String) {
        // 匹配路径部分：Java_com_aisdk_assess_hotWheels_HotWheels_init
        let regex = Regex::new("Java_\\w+").unwrap();
        let path = regex.find(content).map_or_else(|| {
            ""
        }, |matched| {
            matched.as_str()
        });
        // 按照下划线切割字符串
        let split = path.split("_");
        let mut s: Vec<&str> = split.collect();

        if s.len() > 0 {
            // 移除数组中的Java
            s.remove(0);
        }

        let pkg = {
            let mut sum = String::new();
            for i in 0..(s.len() - 2) {
                let x1 = s.get(i).unwrap().trim();
                if sum.is_empty() {
                    sum = format!("{}", x1)
                } else {
                    sum = format!("{}/{}", sum, x1)
                }
            }
            sum
        };

        let class_name = s.get(s.len() - 2).unwrap().trim().to_string();

        let method_name = s.get(s.len() - 1).unwrap().trim().to_string();

        (pkg, class_name, method_name)
    }

    fn deal_params(content: &str) -> Vec<(String, String)> {
        // 匹配形参：(env: JNIEnv, _j_class: JClass, call: JObject)
        let regex = Regex::new("\\([^{]+\\)").unwrap();
        let mut str = regex.find(content).map_or_else(|| {
            "()"
        }, |matched| {
            matched.as_str()
        });
        //去除括号
        let str = &str.replace("(", "").replace(")", "");


        let split = str.split(",");
        let mut vec_str: Vec<&str> = split.collect();
        // 判断无入参情况
        if vec_str.len() < 3 {
            return Vec::new();
        }
        // 移除默认参数
        vec_str.remove(0);
        vec_str.remove(0);

        // 解析形参
        let mut vec_pul = Vec::new();
        for x in vec_str {
            let mut p_s = x.split(":");
            let x0 = p_s.next().unwrap().trim();
            let x1 = p_s.next().unwrap().trim();
            let param = JniApi::transform_param(x1);
            vec_pul.push((x0.to_string(), param.to_string()));
        }
        vec_pul
    }

    fn deal_return_value(content: &str) -> String {
        // 匹配返回值： -> jint
        let regex = Regex::new("->[^{]+").unwrap();
        let mut str = regex.find(content).map_or_else(|| {
            "->"
        }, |matched| {
            matched.as_str()
        });

        // 移除返回值符号 ->
        let str = &str.replace("->", "");

        JniApi::transform_param(str.trim()).to_string()
    }

    fn transform_param(str: &str) -> &str {
        match str {
            "JObject" => { "Object" }
            "jobject" => { "Object" }
            "JString" => { "String" }
            "jstring" => { "String" }
            "jint" => { "int" }
            "jlong" => { "long" }
            "jbyte" => { "byte" }
            "jboolean" => { "boolean" }
            "jchar" => { "char" }
            "jshort" => { "short" }
            "jfloat" => { "float" }
            "jdouble" => { "double" }
            "jsize" => { "int" }
            _ => { "" }
        }
    }

    fn get_method_str(&self) -> String {
        format!("public native static {} {}({});",
                if self.return_value.is_empty() {
                    "void".to_string()
                } else {
                    self.return_value.clone()
                },
                self.method.clone(),
                {
                    if self.params.is_empty() {
                        "".to_string()
                    } else {
                        let mut p = String::new();
                        for (name, obj) in &self.params {
                            if p.is_empty() {
                                p = format!("{} {}", obj, name)
                            } else {
                                p = format!("{}, {} {}", p, obj, name)
                            }
                        }
                        p
                    }
                })
    }
}

impl Display for JniApi {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "JniApi({} {} {} {} {})", self.package, self.class, self.method, {
            if self.params.is_empty() {
                "".to_string()
            } else {
                let mut p = String::new();
                for (name, obj) in &self.params {
                    if p.is_empty() {
                        p = format!("{}:{}", name, obj)
                    } else {
                        p = format!("{} {}:{}", p, name, obj)
                    }
                }
                p
            }
        }, self.return_value)
    }
}

fn java_file_create(java_path: &str, pkg: &str, class: &str, method: Vec<String>) -> Result<(), CliError> {
    // 添加方法
    let mut java_str = String::new();
    for m in method {
        if java_str.is_empty() {
            java_str = format!("\n\t{}\n", m);
        } else {
            java_str = format!("{}\n\t{}\n", java_str, m);
        }
    }

    //填充模板
    let java_str = format!("package {};\n\npublic class {} {{\n\n\tstatic {{\n\t\tSystem.loadLibrary(\"{}\");\n\t}}\n{}\n}}",
                           pkg.replace("/", "."), class, "", java_str);

    let folder = get_file_parent(&java_path)?;

    create_dir(&folder)?;

    write_file(&java_path, &java_str)
}


pub fn create_ability() -> Jni {
    Jni {}
}

#[test]
fn test() {
    let result = read_file("./lib").unwrap();
    let jni = create_ability();
    jni.create_target_and_write(jni.analysis(result).unwrap(), "./");
}

#[test]
fn regex() {
    let test = "I am builtiful man, and I say : \"hi\" (n:n)";
    let regex = Regex::new("\\([^)]+\\)").unwrap();
    for matched in regex.find_iter(test) {
        let content = matched.as_str();
        println!("解析内容: {}", content)
    }
}