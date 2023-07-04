use std::{
    fmt::Display,
    fs::{self, File},
    io::{Read, Write},
    path::Path,
    process,
    time::Duration,
};

use crate::error::DtrackError;
use reqwest::{header, multipart, Client, IntoUrl, StatusCode};
use serde_json::Value;
#[derive(Debug, Clone)]
pub(crate) struct Auditor<T> {
    url: T,
    client: Client,
    pub(crate) version: Option<String>,
}

impl<T> Auditor<T>
where
    T: IntoUrl + Display,
{
    pub(crate) async fn new<U>(url: T, proxy: Option<U>, key: String) -> Result<Self, DtrackError>
    where
        U: IntoUrl + Display,
    {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::HeaderName::from_static("x-api-key"),
            header::HeaderValue::from_str(&key).map_err(|_| DtrackError::InvalidHeader(key))?,
        );
        log::debug!("构造的请求头：{headers:?}");
        let mut builder = Client::builder();
        if let Some(proxy) = proxy {
            log::debug!("使用的代理：{proxy}");
            let proxy = reqwest::Proxy::all(proxy).map_err(|_| DtrackError::ProxyBuildError)?;
            builder = builder.proxy(proxy);
        }

        let client = builder
            .http1_title_case_headers()
            .default_headers(headers)
            .danger_accept_invalid_certs(true)
            .build()
            .map_err(|_| DtrackError::ClientBuildError)?;

        let mut auditor = Self {
            url,
            client,
            version: None,
        };
        match auditor.set_version().await {
            Ok(()) => log::debug!("Version 获取结果: {}", auditor.version.is_some()),
            Err(e) => {
                log::error!("Error: {}", e);
                eprintln!("程序运行出错: {}", e);
                process::exit(1);
            }
        };
        Ok(auditor)
    }

    async fn set_version(&mut self) -> Result<(), DtrackError> {
        let version_url = format!("{}api/version", self.url);
        log::debug!("version_url{:?}", version_url);
        let response = self
            .client
            .get(&version_url)
            .send()
            .await
            .map_err(|_| DtrackError::UnableToConnect(version_url))?;

        if response.status().is_success() {
            let res = response
                .json::<Value>()
                .await
                .map_err(|_| DtrackError::UnableToParseJson)?;
            if let Some(version) = res["version"].as_str() {
                log::info!("连接服务器成功!");
                log::debug!("当前版本：{:?}", version);
                self.version = Some(version.to_string());
                return Ok(());
            } else {
                return Err(DtrackError::CustomInvalidInfo(
                    "无法从响应获取版本信息".to_owned()
                ));
            }
        } else {
            log::debug!("{}", response.status());
            log::debug!(
                "{:?}",
                response
                    .text()
                    .await
                    .map_err(|_| DtrackError::UnableToGetText)?
            );
            log::error!("连接服务器失败,请检查URL地址及key值!");
        }
        Err(DtrackError::CustomInvalidInfo(
            "连接服务器失败,请检查URL地址及key值".to_owned(),
        ))
    }

    pub(crate) async fn read_upload_bom(
        &self,
        project: &str,
        project_version: &str,
        bom_file: &str,
    ) -> Result<String, DtrackError> {
        let upload_url = format!("{}api/v1/bom", self.url);
        log::debug!("upload_url{:?}", upload_url);
        let file_path = Path::new(bom_file);
        let mut file = fs::File::open(file_path)?;
        let metadata = fs::metadata(file_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let form = multipart::Form::new()
            .text("autoCreate", "true")
            .text("projectName", project.to_string())
            .text("projectVersion", project_version.to_string())
            .part(
                "bom",
                multipart::Part::stream_with_length(buffer, metadata.len()).file_name(
                    file_path
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .into_owned(),
                ),
            );

        log::debug!("{:?}", form);
        let response = self
            .client
            .post(&upload_url)
            .multipart(form)
            .send()
            .await
            .map_err(|_| DtrackError::UnableToConnect(upload_url))?;
        if response.status().is_success() {
            log::info!("上传bom文件成功!");
            let results = response
                .json::<Value>()
                .await
                .map_err(|_| DtrackError::UnableToParseJson)?;
            log::info!("从服务端获取项目id完成!");
            if let Some(token) = results["token"].as_str() {
                return Ok(token.to_string());
            }
        } else {
            log::debug!("{}", response.status());
            log::debug!(
                "{:?}",
                response
                    .text()
                    .await
                    .map_err(|_| DtrackError::UnableToGetText)?
            );
            log::error!("上传bom文件失败,请检查URL地址及key值!");
        }
        Err(DtrackError::CustomInvalidInfo(
            "上传bom文件失败,请检查URL地址及key值".to_owned(),
        ))
    }

    pub(crate) async fn poll_bom_token_being_processed(
        &self,
        token: &str,
    ) -> Result<bool, DtrackError> {
        let token_url = format!("{}api/v1/bom/token/{}", self.url, token);
        loop {
            log::debug!("token_url{:?}", token_url);
            let response = self
                .client
                .get(&token_url)
                .send()
                .await
                .map_err(|_| DtrackError::UnableToConnect(token_url.to_string()))?
                .json::<Value>()
                .await
                .map_err(|_| DtrackError::UnableToParseJson)?;
            if let Some(process) = response["processing"].as_bool() {
                log::info!("SCA扫描正在处理中: {:?}", process);
                if !process {
                    break;
                }
            }
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
        Ok(true)
    }

    pub(crate) async fn get_project_uuid(
        &self,
        project: &str,
        project_version: &str,
    ) -> Result<String, DtrackError> {
        let project_url = format!(
            "{}api/v1/project/lookup?name={}&version={}",
            self.url, project, project_version
        );
        log::debug!("project_url{:?}", project_url);
        let response = self
            .client
            .get(&project_url)
            .send()
            .await
            .map_err(|_| DtrackError::UnableToConnect(project_url))?;
        if response.status().is_success() {
            // 解析响应体为 JSON
            let results = response
                .json::<Value>()
                .await
                .map_err(|_| DtrackError::UnableToParseJson)?;
            // 将 JSON 写入文件
            log::info!("从服务端获取项目id完成!");
            if let Some(uuid) = results["uuid"].as_str() {
                return Ok(uuid.to_string());
            }
        } else {
            log::error!("无法从服务端获取项目,请检查URL地址及key值.");
        }
        Err(DtrackError::CustomInvalidInfo(
            "无法从服务端获取项目,请检查URL地址及key值".to_owned(),
        ))
    }

    pub(crate) async fn get_project_findings(
        &self,
        project_uuid: &str,
        output: &str,
    ) -> Result<(), DtrackError> {
        let findings_url = format!("{}api/v1/finding/project/{}/export", self.url, project_uuid);
        log::debug!("findings_url{:?}", findings_url);
        let response = self
            .client
            .get(&findings_url)
            .send()
            .await
            .map_err(|_| DtrackError::UnableToConnect(findings_url))?;

        if response.status().is_success() {
            // 解析响应体为 JSON
            let results = response
                .text()
                .await
                .map_err(|_| DtrackError::UnableToGetText)?;
            // 将 JSON 写入文件
            let mut file = File::create(output)?;
            log::debug!("{:?}", file.metadata());
            file.write_all(results.as_bytes())?;
            log::info!("将扫描结果写入文件{:?}完成!", output);
            Ok(())
        } else if response.status() == StatusCode::NOT_FOUND {
            log::error!("项目名称及版本不存在");
            Err(DtrackError::CustomInvalidInfo(
                "项目名称及版本不存在".to_owned(),
            ))
        } else {
            log::error!("无法从服务端获取扫描结果,请检查URL地址及key值.");
            Err(DtrackError::CustomInvalidInfo(
                "无法从服务端获取扫描结果,请检查URL地址及key值".to_owned(),
            ))
        }
    }
}
