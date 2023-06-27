use std::{
    fs::{self, File},
    io::{Read, Write},
    path::Path,
    time::Duration,
};

use log::{debug, error, info};
use reqwest::{header, multipart, Client, StatusCode};
use serde_json::Value;

#[derive(Debug, Clone)]
pub(crate) struct Auditor {
    url: String,
    client: Client,
    pub(crate) version: String,
}

impl Auditor {
    pub(crate) async fn new(url: String, key: String) -> Result<Self, Box<dyn std::error::Error>> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::HeaderName::from_static("x-api-key"),
            header::HeaderValue::from_str(&key).unwrap(),
        );
        debug!("构造的请求头：{headers:?}");
        let client = Client::builder()
            .http1_title_case_headers()
            .default_headers(headers)
            .danger_accept_invalid_certs(true)
            .build()?;

        let mut auditor = Self {
            url,
            client,
            version: format!("0.0.0"),
        };
        match auditor.set_version().await {
            Ok(()) => debug!("Version: {}", auditor.version),
            Err(e) => error!("Error: {}", e),
        };
        Ok(auditor)
    }

    async fn set_version(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let version_url = format!("{}/api/version", self.url);
        let res = self
            .client
            .get(&version_url)
            .send()
            .await?
            .json::<Value>()
            .await?;
        if let Some(version) = res["version"].as_str() {
            self.version = version.to_string();
            Ok(())
        } else {
            Err("无法获取版本信息".into())
        }
    }

    pub(crate) async fn read_upload_bom(
        &self,
        project: &str,
        project_version: &str,
        bom_file: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let upload_url = format!("{}/api/v1/bom", self.url);
        debug!("upload_url{:?}", upload_url);
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

        debug!("{:?}", form);
        let response = self.client.post(&upload_url).multipart(form).send().await?;
        if response.status().is_success() {
            info!("上传bom文件成功!");
            let results = response.json::<Value>().await?;
            info!("从服务端获取项目id完成!");
            if let Some(token) = results["token"].as_str() {
                return Ok(token.to_string());
            }
        } else {
            error!("上传bom文件失败!");
        }
        Err("上传bom文件失败".into())
    }

    pub(crate) async fn poll_bom_token_being_processed(
        &self,
        token: &str,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let token_url = format!("{}/api/v1/bom/token/{}", self.url, token);
        loop {
            debug!("token_url{:?}", token_url);
            let response = self
                .client
                .get(&token_url)
                .send()
                .await?
                .json::<Value>()
                .await?;
            if let Some(process) = response["processing"].as_bool() {
                debug!("正在处理中: {:?}", process);
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
    ) -> Result<String, Box<dyn std::error::Error>> {
        let project_url = format!(
            "{}/api/v1/project/lookup?name={}&version={}",
            self.url, project, project_version
        );
        debug!("project_url{:?}", project_url);
        let response = self.client.get(&project_url).send().await?;
        if response.status().is_success() {
            // 解析响应体为 JSON
            let results = response.json::<Value>().await?;
            // 将 JSON 写入文件
            info!("从服务端获取项目id完成!");
            if let Some(uuid) = results["uuid"].as_str() {
                return Ok(uuid.to_string());
            }
        } else {
            error!("无法从服务端获取项目.");
        }
        Err("无法从服务端获取项目".into())
    }

    pub(crate) async fn get_project_findings(
        &self,
        project_uuid: &str,
        output: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let findings_url = format!(
            "{}/api/v1/finding/project/{}/export",
            self.url, project_uuid
        );
        debug!("findings_url{:?}", findings_url);
        let response = self.client.get(&findings_url).send().await?;

        if response.status().is_success() {
            // 解析响应体为 JSON
            let results = response.text().await?;
            // 将 JSON 写入文件
            let mut file = File::create(output)?;
            debug!("{:?}", file.metadata());
            file.write_all(results.as_bytes())?;
            info!("将扫描结果写入文件{:?}完成!", output);
            Ok(())
        } else if response.status() == StatusCode::NOT_FOUND {
            error!("项目名称及版本不存在");
            Err("项目名称及版本不存在".into())
        } else {
            error!("无法从服务端获取扫描结果.");
            Err("无法从服务端获取扫描结果".into())
        }
    }
}
