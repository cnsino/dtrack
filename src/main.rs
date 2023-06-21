use clap::Parser;
use log::{error, info, warn , debug};
/// Dependency-track 的cli程序
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // /// Name of the person to greet
    // #[arg(short, long)]
    // name: String,
    // /// Number of times to greet
    // #[arg(short, long, default_value_t = 1)]
    // count: u8,
    // #[arg(short='e', long)]
    // count2: Option<usize>,
    /// 设置dependencytrack的访问地址. eg. http://dtrack.abc.local:8080.
    #[arg(
        short,
        long,
        value_name = "Url",
        default_value = "http://127.0.0.1:8080"
    )]
    url: Option<String>,
    /// 设置dependencytrack的apikey. eg. adfadfe343g.
    #[arg(short, long, value_name = "Apikey")]
    key: Option<String>,
    /// 设置dependencytrack的项目名称.
    #[arg(short, long, value_name = "Project")]
    project: Option<String>,
    /// 设置dependencytrack的文件.
    #[arg(short, long, value_name = "File")]
    file: Option<String>,
    /// 设置dependencytrack的规则.
    #[arg(short, long, value_name = "Rule")]
    rule: Option<String>,
}


fn main()  {
    env_logger::Builder::new().filter_level(log::LevelFilter::Debug).init();
    let args = Args::parse();
    debug!("{args:?}");
    debug!("{}", args.url.unwrap());
    debug!("{}", args.key.unwrap());
     
}
