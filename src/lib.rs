use clap::Parser;
use error::DtrackError;
use std::process;

mod args;
mod auditor;
pub mod error;
pub async fn builder() -> Result<(), DtrackError> {
    let args = args::Dtrack::parse();
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Off)
        .filter_module("dtrack", args.log_level)
        .init();

    let auditor = auditor::Auditor::new(args.url.unwrap(), args.proxy, args.key.unwrap()).await?;
    log::debug!("{auditor:?}");
    if args.dversion {
        if let Some(version) = auditor.version {
            println!("当前 Dependency-Track的安装版本是：{}", version);
        } else {
            println!("未能获取到Dependency-Track的安装版本");
        }
        process::exit(0);
    }

    if let (Some(project), Some(project_version), Some(bom_file), Some(output)) =
        (&args.project, &args.edition, &args.file, &args.output)
    {
        log::debug!("输入的参数：项目：{project}, 版本：{project_version}, Bom文件：{bom_file}, 输出文件：{output}");
        let token = Some(
            auditor
                .read_upload_bom(project, project_version, bom_file)
                .await?,
        );
        if let Some(token) = token {
            log::info!("Bom: {:?}扫描开始", token);
            let scan_status = auditor
                .poll_bom_token_being_processed(token.as_str())
                .await?;
            if scan_status {
                log::info!("Bom: {:?}扫描完成", token)
            }
        }
        let project_uuid = auditor.get_project_uuid(project, project_version).await?;
        log::info!("获取的项目id{:?}", project_uuid);
        auditor
            .get_project_findings(project_uuid.as_str(), output)
            .await?;
    } else {
        log::error!("{}", DtrackError::ParamMissing);
        return Err(DtrackError::ParamMissing);
    }
    Ok(())
}
