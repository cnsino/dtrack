// use std::{io, backtrace::Backtrace};

use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DtrackError {
    #[error("输入指定的参数不满足要求")]
    ParamMissing,
    #[error("Url: {0} 的API地址错误或无法连接")]
    UnableToConnect(String),
    #[error("无法获取Text数据")]
    UnableToGetText,
    #[error("无法解析json数据")]
    UnableToParseJson,
    #[error("{0}")]
    CustomInvalidInfo(String),
    #[error("不合法的请求头header:{0}")]
    InvalidHeader(String),
    #[error("Reqwest Client构建失败")]
    ClientBuildError,
    #[error("Reqwest Proxy构建失败")]
    ProxyBuildError,
    #[error(transparent)]
    IoError(#[from] io::Error),
}
