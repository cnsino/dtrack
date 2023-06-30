use std::str::FromStr;

use clap::builder::TypedValueParser as _;
use clap::Parser;
use reqwest::Url;
/// Dependency-track 的命令行程序
#[derive(Parser, Debug, Clone)]
#[command(author,version, about, long_about = None)]
pub(crate) struct Dtrack {
    /// 设置 Dependency-track 的访问地址. eg. http://dtrack.abc.local:8081.
    #[arg(
        short,
        long,
        value_name = "Url",
        default_value = "http://127.0.0.1:8081"
    )]
    pub(crate) url: Option<Url>,
    /// 设置 Dependency-track 的apikey. eg. adfadfe343g.
    #[arg(
        short,
        long,
        value_name = "Apikey",
        default_value = "Oh9LHLfrLgk77e67DEZtiitOWZwvFVXI"
    )]
    pub(crate) key: Option<String>,

    /// 设置 Dependency-track 的连接代理. eg. http://127.0.0.1:8080.
    #[arg(
        long,
        value_name = "Proxy",
    )]
    pub(crate) proxy: Option<Url>,
    /// 设置 Dependency-track 的项目名称.
    #[arg(short, long, value_name = "Project Name", default_value = "test")]
    pub(crate) project: Option<String>,
    /// 设置 Dependency-track 的项目版本.
    #[arg(short, long, value_name = "Project Version", default_value = "default")]
    pub(crate) edition: Option<String>,
    /// 设置 Dependency-track 的文件.
    #[arg(short, long, value_name = "Bom File")]
    pub(crate) file: Option<String>,
    /// 设置 Dependency-track 的规则.
    #[arg(short, long, value_name = "Rule")]
    pub(crate) rule: Option<String>,
    /// 输出 Dependency-track 版本信息.
    #[arg(short, long, default_value_t = false)]
    pub(crate) dversion: bool,
    /// 设置 Dependency-track 的扫描结果存储位置.
    #[arg(
        short,
        long,
        value_name = "Scan Result",
        default_value = "results.json"
    )]
    pub(crate) output: Option<String>,
    /// 设置输出日志的级别(选择off不输出日志)
    #[arg(
        short,
        long,
        default_value = "debug",
        value_parser = clap::builder::PossibleValuesParser::new(["off", "debug", "info", "warn", "error"])
            .map(|s| log::LevelFilter::from_str(&s).unwrap()),
    )]
    pub(crate) log_level: log::LevelFilter,
}
