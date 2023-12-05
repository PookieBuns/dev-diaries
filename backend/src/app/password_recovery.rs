use crate::app::errors::Result;
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env;

const EMAIL_USERNAME: &str = env!("APP_SMTP_USERNAME");
const EMAIL_PASSWORD: &str = env!("APP_SMTP_PASSWORD");
const SMTP_HOST: &str = env!("APP_SMTP_HOST");

pub fn send_email(email_address: &str, subject: &str, body: &str) -> Result<()> {
    let from_mailbox: Mailbox = EMAIL_USERNAME.parse()?;
    let to_mailbox: Mailbox = email_address.parse()?;
    let email = Message::builder()
        .from(from_mailbox)
        .to(to_mailbox)
        .subject(subject)
        .body(body.to_string())?;

    let creds = Credentials::new(EMAIL_USERNAME.to_string(), EMAIL_PASSWORD.to_string());

    let mailer = SmtpTransport::relay(SMTP_HOST)?
        .credentials(creds)
        .build();

    mailer.send(&email)?;
    Ok(())
}

pub fn send_password_recovery_email(email_address: &str, token: &str) -> Result<()> {
    let subject = "Password Recovery";
    let body = format!(
        "Please click the following link to reset your password: http://localhost:3000/reset-password?token={}",
        token
    );
    send_email(email_address, subject, &body)
}
