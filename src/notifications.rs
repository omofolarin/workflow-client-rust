use std::collections::HashMap;

use chrono::NaiveDateTime;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{get_json_response, Response, Workflow, WorkflowError};
pub struct Notifications<'a>(pub(crate) &'a mut Workflow);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessagePlatform {
    SMS,
    Email,
    Whatsapp,
    WebPush,
    MobilePush,
    Slack,
}

impl Default for MessagePlatform {
    fn default() -> Self {
        MessagePlatform::Email
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageProviders {
    Firebase,
    Termii,
    Sendgrid,
    Twilio,
    MagicBell,
    Postmark,
    MailChimp,
    Vapid,
}

impl ToString for MessageProviders {
    fn to_string(&self) -> String {
        match self {
            MessageProviders::Firebase => "firebase".to_string(),
            MessageProviders::Termii => "termii".to_string(),
            MessageProviders::Sendgrid => "sendgrid".to_string(),
            MessageProviders::Twilio => "twilio".to_string(),
            MessageProviders::MagicBell => "magic-bell".to_string(),
            MessageProviders::Postmark => "postmark".to_string(),
            MessageProviders::MailChimp => "mail-chimp".to_string(),
            MessageProviders::Vapid => "vapid".to_string(),
        }
    }
}

pub enum NotificationErrors {
    NetworkError,
}

impl ToString for MessagePlatform {
    fn to_string(&self) -> String {
        match self {
            MessagePlatform::SMS => "sms".to_string(),
            MessagePlatform::Email => "email".to_string(),
            MessagePlatform::Whatsapp => "whatsapp".to_string(),
            MessagePlatform::WebPush => "web-push".to_string(),
            MessagePlatform::MobilePush => "mobile-push".to_string(),
            MessagePlatform::Slack => "slack".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReqBodyMessage<'a> {
    pub company_id: Uuid,
    pub from: &'a str,
    pub to: &'a str,
    pub subject: Option<&'a str>,
    pub message: Option<&'a str>,
    pub template_id: Option<Uuid>,
    pub platform: MessagePlatform,
    pub dynamic_data: Option<serde_json::Value>,
    pub should_schedule: Option<bool>,
    pub schedule_for: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReqBodyBroadCastMessage {
    company_id: Uuid,
    from: String,
    to: Vec<String>,
    subject: Option<String>,
    message: Option<String>,
    template_id: Option<Uuid>,
    channel: String,
    dynamic_data: Option<HashMap<String, String>>,
    should_schedule: Option<bool>,
    schedule_for: Option<NaiveDateTime>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReqBodyTemplate<'a> {
    company_id: Uuid,
    template_title: Option<String>,
    description: Option<String>,
    template: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReqBodyContact {
    company_id: Uuid,
    user_id: Uuid,
    role: String,
    email: String,
    sms_number: Option<String>,
    whatsapp_number: Option<String>,
    allow_push_notifications: Option<bool>,
    mobile_device_token: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResBodyContact {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResBodyBroadCastMessage {}

impl Notifications<'_> {
    pub async fn fetch_contact(&self) -> Result<Response, WorkflowError> {
        let extract = Workflow::extract_client_data(self.0)
            .map_err(|e| WorkflowError::InvalidHeader(e.to_string()))?;
        let company = extract.company.unwrap();
        let user = extract.user.unwrap();
        let url = format!("{}/contact", extract.base_url);

        let response = extract
            .client
            .get(url)
            .header(company.0, company.1)
            .header(user.0, user.1)
            .send()
            .await
            .map_err(|e| WorkflowError::Api(e))?;

        get_json_response(response).await
    }

    pub async fn send_message(&self, msg: &ReqBodyMessage<'_>) -> Result<Response, WorkflowError> {
        let extract = Workflow::extract_client_data(self.0)
            .map_err(|e| WorkflowError::InvalidHeader(e.to_string()))?;
        let company = extract.company.unwrap();
        let user = extract.user.unwrap();
        let notification = extract.notification.unwrap();

        let url = format!("{}/send-message", extract.base_url);
        let response = extract
            .client
            .post(url)
            .json(msg)
            .header(company.0, company.1)
            .header(notification.0, notification.1)
            .header(user.0, user.1)
            .send()
            .await
            .map_err(|e| WorkflowError::Api(e))?;

        get_json_response(response).await
    }

    pub async fn send_broadcast_message(
        &self,
        msg: &ReqBodyMessage<'_>,
    ) -> Result<Response, WorkflowError> {
        let extract = Workflow::extract_client_data(self.0)
            .map_err(|e| WorkflowError::InvalidHeader(e.to_string()))?;
        let company = extract.company.unwrap();
        let user = extract.user.unwrap();
        let notification = extract.notification.unwrap();

        let url = format!("{}/send-message", extract.base_url);
        let response = extract
            .client
            .post(url)
            .json(msg)
            .header(company.0, company.1)
            .header(notification.0, notification.1)
            .header(user.0, user.1)
            .send()
            .await
            .map_err(|e| WorkflowError::Api(e))?;

        get_json_response(response).await
    }

    pub async fn create_template(
        &mut self,
        template_data: &ReqBodyTemplate<'_>,
    ) -> Result<Response, WorkflowError> {
        let extract = Workflow::extract_client_data(self.0)
            .map_err(|e| WorkflowError::InvalidHeader(e.to_string()))?;
        let company = extract.company.unwrap();
        let user = extract.user.unwrap();

        let url = format!("{}/template", extract.base_url);
        let response = extract
            .client
            .post(url)
            .json(template_data)
            .header(company.0, company.1)
            .header(user.0, user.1)
            .send()
            .await
            .map_err(|e| WorkflowError::Api(e))?;

        get_json_response(response).await
    }

    pub async fn fetch_contact_list(&self) {}

    pub async fn create_contact(&mut self) {}

    pub async fn update_contact(&mut self) {}

    pub async fn delete_contact(&mut self) {}

    pub async fn fetch_template(&self) {}

    pub async fn update_template(&mut self) {}

    pub async fn delete_template(&mut self) {}
}
