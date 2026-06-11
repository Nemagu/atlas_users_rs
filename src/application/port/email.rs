use crate::{application::error::AppResult, domain::user::value_object::Email};

#[derive(Debug, Clone)]
pub(crate) struct EmailBody {
    pub(crate) to: Email,
    pub(crate) subject: String,
    pub(crate) html_msg: Option<String>,
    pub(crate) text_msg: Option<String>,
}

impl EmailBody {
    pub(crate) fn new(
        to: Email,
        subject: String,
        html_msg: Option<String>,
        text_msg: Option<String>,
    ) -> Self {
        Self {
            to,
            subject,
            html_msg,
            text_msg,
        }
    }
}

pub(crate) trait EmailBuilder {
    fn confirm_email_for_new_user(&self, email: Email) -> AppResult<EmailBody>;
    fn login_by_code(&self, email: Email) -> AppResult<EmailBody>;
    fn confirm_email_for_changing_email(&self, email: Email) -> AppResult<EmailBody>;
}

#[async_trait::async_trait]
pub(crate) trait EmailClient: Send + Sync {
    async fn send_email(&self, body: EmailBody) -> AppResult<()>;
    async fn send_emails(&self, body: EmailBody) -> AppResult<()>;
}
