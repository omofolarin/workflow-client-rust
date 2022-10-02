use reqwest::header::{self, InvalidHeaderValue};

use uuid::Uuid;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub mod buckets;
pub mod notifications;

pub async fn get_json_response(response: reqwest::Response) -> Result<Response, WorkflowError> {
    let status_code = response.status();
    let response_headers = response.headers().to_owned();
    let response_body = response
        .json::<serde_json::Value>()
        .await
        .map_err(|e| WorkflowError::Format(e))?;
    Ok(Response {
        status_code,
        response_headers,
        response_body: response_body.to_owned(),
    })
}

#[derive(Debug)]
pub struct Response {
    pub status_code: reqwest::StatusCode,
    pub response_headers: reqwest::header::HeaderMap,
    pub response_body: serde_json::Value,
}

#[derive(Debug)]
pub enum WorkflowError {
    InvalidHeader(String),
    Api(reqwest::Error),
    Format(reqwest::Error),
}

impl ToString for WorkflowError {
    fn to_string(&self) -> String {
        match self {
            WorkflowError::InvalidHeader(e) => e.to_string(),
            WorkflowError::Api(e) => e.to_string(),
            WorkflowError::Format(e) => e.to_string(),
        }
    }
}
#[derive(Debug)]
pub struct Workflow {
    http_client: reqwest::Client,
    base_url: String,
    x_company_id: Option<Uuid>,
    x_user_id: Option<Uuid>,
    x_notification_id: Option<Uuid>,
    x_storage_id: Option<Uuid>,
    x_webhook_key: Option<String>,
    api_token: Option<String>,
}

pub struct WorkflowExtract<'a> {
    client: &'a reqwest::Client,
    base_url: &'a str,
    company: Option<(&'a str, header::HeaderValue)>,
    user: Option<(&'a str, header::HeaderValue)>,
    notification: Option<(&'a str, header::HeaderValue)>,
}

impl Workflow {
    pub fn new(base_url: &str) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            base_url: base_url.to_owned(),
            x_company_id: None,
            x_user_id: None,
            x_notification_id: None,
            x_storage_id: None,
            api_token: None,
            x_webhook_key: None,
        }
    }

    pub fn set_company(&mut self, company_id: Uuid) -> &mut Self {
        self.x_company_id = Some(company_id);
        self
    }

    pub fn set_user(&mut self, user_id: Uuid) -> &mut Self {
        self.x_user_id = Some(user_id);
        self
    }

    pub fn set_api_token(&mut self, api_token: Option<String>) -> &mut Self {
        self.api_token = api_token;
        self
    }

    pub fn set_notification_config(&mut self, config_id: Option<Uuid>) -> &mut Self {
        self.x_notification_id = config_id;
        self
    }

    pub fn set_storage_config(&mut self, config_id: Option<Uuid>) -> &mut Self {
        self.x_storage_id = config_id;
        self
    }

    pub fn set_web_hook_config(&mut self, config_id: Option<String>) -> &mut Self {
        self.x_webhook_key = config_id;
        self
    }

    pub fn notifications(&mut self) -> notifications::Notifications {
        notifications::Notifications(self)
    }

    pub fn buckets(&mut self) -> buckets::Buckets {
        buckets::Buckets(self)
    }

    pub(crate) fn extract_client_data(&self) -> Result<WorkflowExtract, InvalidHeaderValue> {
        let client = &self.http_client;
        let base_url = self.base_url.as_str();

        let company = match self.x_company_id {
            Some(company) => Some((
                "x-company-id",
                header::HeaderValue::from_str(company.to_string().as_str())?,
            )),
            None => None,
        };

        let user = match self.x_user_id {
            Some(user) => Some((
                "x-user-id",
                header::HeaderValue::from_str(user.to_string().as_str())?,
            )),
            None => None,
        };

        let notification = match self.x_notification_id {
            Some(notification) => Some((
                "x-notification-id",
                header::HeaderValue::from_str(notification.to_string().as_str())?,
            )),
            None => None,
        };

        Ok(WorkflowExtract {
            client,
            base_url,
            company,
            user,
            notification,
        })
    }
}

// fn cool() {
//     let a = Workflow::new("https://github.com/facebook")
//         .set_company("")
//         .set_notification_config(Some(""))
//         .notifications()
//         .send_message(msg)
//         .await
//         .unwrap();
// }

// #[cfg(test)]
// mod tests {
//     use crate::notifications::ReqBodyMessage;

//     use super::*;

//     #[test]
//     async fn send_notifications() {
//         let a = Workflow::new("https://github.com/facebook")
//             .set_company(Uuid::new_v4())
//             .set_notification_config(Some(Uuid::new_v4()))
//             .notifications()
//             .send_message(&ReqBodyMessage {
//                 company_id: todo!(),
//                 from: todo!(),
//                 to: todo!(),
//                 subject: todo!(),
//                 message: todo!(),
//                 template_id: todo!(),
//                 channel: todo!(),
//                 dynamic_data: todo!(),
//                 should_schedule: todo!(),
//                 schedule_for: todo!(),
//             })
//             .await
//             .unwrap();
//         assert_eq!(
//             a,
//             Response {
//                 status_code: 200,
//                 response_headers: header::HeaderMap::new(),
//                 response_body: serde_json::json!({})
//             }
//         )
//     }
// }
