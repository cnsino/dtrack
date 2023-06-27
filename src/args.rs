use clap::Parser;
/// Dependency-track 的cli程序
#[derive(Parser, Debug, Clone)]
#[command(author,version, about, long_about = None)]
pub(crate) struct Dtrack {
    /// 设置dependencytrack的访问地址. eg. http://dtrack.abc.local:8080.
    #[arg(
        short,
        long,
        value_name = "Url",
        default_value = "http://127.0.0.1:8081"
    )]
    pub(crate) url: Option<String>,
    /// 设置dependencytrack的apikey. eg. adfadfe343g.
    #[arg(
        short,
        long,
        value_name = "Apikey",
        default_value = "Oh9LHLfrLgk77e67DEZtiitOWZwvFVXI"
    )]
    pub(crate) key: Option<String>,
    /// 设置dependencytrack的项目名称.
    #[arg(short, long, value_name = "Project Name", default_value = "test")]
    pub(crate) project: Option<String>,
    /// 设置dependencytrack的项目版本.
    #[arg(short, long, value_name = "Project Version", default_value = "default")]
    pub(crate) edition: Option<String>,
    /// 设置dependencytrack的文件.
    #[arg(short, long, value_name = "Bom File")]
    pub(crate) file: Option<String>,
    /// 设置dependencytrack的规则.
    #[arg(short, long, value_name = "Rule")]
    pub(crate) rule: Option<String>,
    /// 输出dependencytrack版本信息.
    #[arg(
        short,
        long,
        value_name = "Dependency-Track Version",
        default_value_t = false
    )]
    pub(crate) dversion: bool,
    /// 设置dependencytrack的扫描结果存储位置.
    #[arg(
        short,
        long,
        value_name = "Scan Result",
        default_value = "results.json"
    )]
    pub(crate) output: Option<String>,
}
