extern crate encoding;
use encoding::all::GBK;
use encoding::Encoding;
use std::fmt::Display;
// use encoding_rs::GBK;
use std::path::PathBuf;
pub fn get_ld_console_path(ld_dir: &PathBuf) -> PathBuf {
    ld_dir.join("ldconsole.exe")
}

#[derive(Debug)]
pub enum LdError {
    // 编码错误
    EncodingError,
    // 运行错误
    RuntimeError(std::io::Error),
}

impl Display for LdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LdError::EncodingError => write!(f, "EncodingError"),
            LdError::RuntimeError(e) => write!(f, "RuntimeError {}", e),
        }
    }
}

impl From<std::io::Error> for LdError {
    fn from(e: std::io::Error) -> Self {
        LdError::RuntimeError(e)
    }
}

#[derive(Debug)]
pub struct LdMnq {
    pub index: i32,
    pub name: String,
    pub top_handle: i64,
    pub bind_handle: i64,
    pub is_enter_system: i32,
    pub pid: i64,
    pub vbox_pid: i64,
}

// 获取雷电模拟器列表
pub fn list2(ld_dir: &PathBuf) -> Result<Vec<LdMnq>, LdError> {
    // 运行ldconsole.exe命令行
    let ld_console_path = get_ld_console_path(ld_dir);
    let output = std::process::Command::new(ld_console_path)
        .arg("list2")
        .output()?;

    // 输出结果
    let output = GBK.decode(&output.stdout, encoding::DecoderTrap::Strict);
    let output = match output {
        Ok(output) => output,
        Err(_) => return Err(LdError::EncodingError),
    };
    let output = output.split("\r\n").collect::<Vec<&str>>();
    let output = output
        .iter()
        .filter(|x| !x.is_empty())
        .map(|x| x.split(",").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    let data = output
        .iter()
        .map(|x| LdMnq {
            index: x[0].parse::<i32>().unwrap_or(-1),
            name: x[1].to_string(),
            top_handle: x[2].parse::<i64>().unwrap_or(0),
            bind_handle: x[3].parse::<i64>().unwrap_or(0),
            is_enter_system: x[4].parse::<i32>().unwrap_or(0),
            pid: x[5].parse::<i64>().unwrap_or(-1),
            vbox_pid: x[6].parse::<i64>().unwrap_or(-1),
        })
        .collect::<Vec<LdMnq>>();
    // println!("{:#?}", data);
    Ok(data)
}

// 启动雷电模拟器
pub fn launch(ld_dir: &PathBuf, index: i32) -> Result<(), LdError> {
    let ld_console_path = get_ld_console_path(ld_dir);
    let output = std::process::Command::new(ld_console_path)
        .arg("launch")
        .arg("--index")
        .arg(index.to_string())
        .output()?;
    println!("{:#?}", output);
    Ok(())
}

// 关闭雷电模拟器
pub fn close(ld_dir: &PathBuf, index: i32) -> Result<(), LdError> {
    let ld_console_path = get_ld_console_path(ld_dir);
    let output = std::process::Command::new(ld_console_path)
        .arg("quit")
        .arg("--index")
        .arg(index.to_string())
        .output()?;
    println!("{:#?}", output);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list2_test() {
        let ld_dir = PathBuf::from("C:\\leidian\\LDPlayer4");
        let _ = list2(&ld_dir);
    }

    #[test]
    fn launch_test() {
        let ld_dir = PathBuf::from("C:\\leidian\\LDPlayer4");
        let _ = launch(&ld_dir, 0);
    }

    #[test]
    fn close_test() {
        let ld_dir = PathBuf::from("C:\\leidian\\LDPlayer4");
        let _ = close(&ld_dir, 0);
    }
}
