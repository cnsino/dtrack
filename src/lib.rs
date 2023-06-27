use std::process;

use clap::Parser;
use log::{debug, info};

mod args;
mod auditor;

pub async fn builder() {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Debug)
        .init();
    let args = args::Dtrack::parse();

    let auditor = auditor::Auditor::new(args.url.unwrap(), args.key.unwrap())
        .await
        .unwrap();
    debug!("{auditor:?}");
    if args.dversion {
        println!("{}", auditor.version);
        process::exit(0);
    }

    if let (Some(project), Some(project_version), Some(bom_file)) =
        (&args.project, &args.edition, &args.file)
    {
        let token = Some(
            auditor
                .read_upload_bom(project, project_version, bom_file)
                .await
                .unwrap(),
        );
        if let Some(token) = token {
            info!("Bom: {:?}扫描开始", token);
            let scan_status = auditor
                .poll_bom_token_being_processed(token.as_str())
                .await
                .unwrap();
            if scan_status {
                info!("Bom: {:?}扫描完成", token)
            }
        }
    };
    //
    // if let  Some(output) = &args.output
    if let (Some(project), Some(project_version), Some(output)) =
        (&args.project, &args.edition, &args.output)
    {
        let project_uuid = auditor
            .get_project_uuid(project, project_version)
            .await
            .unwrap();
        info!("获取的项目id{:?}", project_uuid);
        auditor
            .get_project_findings(project_uuid.as_str(), output)
            .await
            .unwrap();
    }
}
