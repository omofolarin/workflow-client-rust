use crate::{get_json_response, Response, Workflow, WorkflowError};
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct Buckets<'a>(pub(crate) &'a mut Workflow);

pub type FileData = Vec<u8>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct File {
    file_type: String,
    name: String,
    data: FileData,
}

impl File {
    pub fn file_type(&self) -> &String {
        &self.file_type
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn data(&self) -> &FileData {
        &self.data
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReqBodyFile {
    pub company_id: Uuid,
    pub is_public: bool,
    pub owner: (Uuid, String),
    pub description: String,
    pub folder_id: Option<Uuid>,
    pub files: Vec<File>,
}

impl Buckets<'_> {
    pub async fn upload_file<'a>(&mut self, asset: ReqBodyFile) -> Result<Response, WorkflowError> {
        let mut collect_response = Vec::new();
        let mut status_code = reqwest::StatusCode::ACCEPTED;
        let mut header = None;

        for file in asset.files {
            let extract = Workflow::extract_client_data(self.0)
                .map_err(|e| WorkflowError::InvalidHeader(e.to_string()))?;
            let company = extract.company.unwrap();
            let user = extract.user.unwrap();

            let form = reqwest::multipart::Form::new();
            let upload_data = reqwest::multipart::Part::bytes(file.data().to_vec())
                .file_name(file.name().to_owned())
                .mime_str(&file.file_type)
                .map_err(|e| WorkflowError::Format(e))?;

            let form = form
                .text("companyId", asset.company_id.to_string())
                .text("isPublic", asset.is_public.to_string())
                .text("owner", asset.owner.0.to_string())
                .text("owner", asset.owner.1.to_string())
                .part("upload", upload_data);

            let url = format!("{}/assets", extract.base_url);
            let response = extract
                .client
                .post(url)
                .header(company.0, company.1)
                .header(user.0, user.1)
                .multipart(form)
                .send()
                .await
                .map_err(|e| WorkflowError::Api(e))?;

            let res = get_json_response(response).await?;
            status_code = res.status_code;
            header = Some(res.response_headers);
            collect_response.push(res.response_body);
            if !res.status_code.is_success() {
                break;
            }
        }

        Ok(Response {
            status_code,
            response_headers: header.unwrap_or_else(|| HeaderMap::new()),
            response_body: serde_json::to_value(collect_response).unwrap(),
        })
    }

    async fn delete_file() -> Result<Response, WorkflowError> {
        unimplemented!()
    }

    async fn get_file() -> Result<Response, WorkflowError> {
        unimplemented!()
    }

    async fn create_folder() -> Result<Response, WorkflowError> {
        unimplemented!()
    }

    async fn delete_folder() -> Result<Response, WorkflowError> {
        unimplemented!()
    }

    async fn list_folders() -> Result<Response, WorkflowError> {
        unimplemented!()
    }

    async fn update_folder() -> Result<Response, WorkflowError> {
        unimplemented!()
    }
}
