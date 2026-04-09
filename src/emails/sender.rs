//! Send one outbound email through a Mailbox's SMTP credentials.
//! (`emails/sender.py`)
//!
//! A failed send returns `Err` and the EMAIL task is marked FAILED by the daemon,
//! then retried; the mailbox is left untouched. The raw message is assembled by
//! hand (mirroring the Python `EmailMessage`) and delivered via `send_raw`, so the
//! Message-ID, threading headers, and BCC-as-envelope-recipient are all exact.

use anyhow::{Context, Result};
use lettre::address::Envelope;
use lettre::{Address, AsyncSmtpTransport, AsyncTransport, Tokio1Executor};

use crate::models::Mailbox;

/// Send `body` from `mailbox` to `to_address`; return the Message-ID. (`send_email`)
pub async fn send_email(
    mailbox: &Mailbox,
    to_address: &str,
    subject: &str,
    body: &str,
    bcc: Option<&str>,
    in_reply_to: Option<&str>,
    references: Option<&str>,
) -> Result<String> {
    let message_id = mint_message_id(&mailbox.from_address);
    let signed = sign(body, mailbox.signature.as_deref());
    let raw = build_raw(mailbox, to_address, subject, &signed, &message_id, in_reply_to, references);

    // Envelope: BCC is a recipient but never a header (mirrors send_message stripping Bcc).
    let from: Address = mailbox.from_address.parse().context("from address")?;
    let mut recipients: Vec<Address> = vec![to_address.parse().context("to address")?];
    if let Some(b) = bcc.filter(|b| !b.is_empty()) {
        recipients.push(b.parse().context("bcc address")?);
    }
    let envelope = Envelope::new(Some(from), recipients).context("envelope")?;

    let transport = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&mailbox.host)
        .context("smtp relay")?
        .port(mailbox.port as u16)
        .credentials(lettre::transport::smtp::authentication::Credentials::new(
            mailbox.username.clone(),
            mailbox.password.clone(),
        ))
        .build();

    transport
        .send_raw(&envelope, raw.as_bytes())
        .await
        .context("smtp send")?;
    Ok(message_id)
}

/// Append the mailbox's sign-off, separated by a blank line. (`_sign`)
fn sign(body: &str, signature: Option<&str>) -> String {
    let sig = signature.unwrap_or("").trim();
    if sig.is_empty() {
        return body.to_string();
    }
    format!("{}\n\n{}\n", body.trim_end(), sig)
}

/// A unique RFC-5322 Message-ID anchored to the sending domain. (`_mint_message_id`)
fn mint_message_id(from_address: &str) -> String {
    let domain = from_address.rsplit('@').next().unwrap_or("localhost");
    let a: u64 = rand::random();
    let b: u64 = rand::random();
    format!("<{a:016x}.{b:016x}@{domain}>")
}

/// Assemble the raw RFC-822 message with threading headers. (`_build_message`)
fn build_raw(
    mailbox: &Mailbox,
    to_address: &str,
    subject: &str,
    body: &str,
    message_id: &str,
    in_reply_to: Option<&str>,
    references: Option<&str>,
) -> String {
    let date = chrono::Utc::now().to_rfc2822();
    let mut headers = vec![
        format!("Message-ID: {message_id}"),
        format!("Date: {date}"),
        format!("From: {}", mailbox.from_address),
        format!("To: {to_address}"),
        format!("Subject: {subject}"),
    ];
    if let Some(irt) = in_reply_to {
        headers.push(format!("In-Reply-To: {irt}"));
        headers.push(format!("References: {}", references.unwrap_or(irt)));
    }
    headers.push("MIME-Version: 1.0".to_string());
    headers.push("Content-Type: text/plain; charset=utf-8".to_string());
    format!("{}\r\n\r\n{}", headers.join("\r\n"), body.replace('\n', "\r\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sign_appends_or_leaves() {
        assert_eq!(sign("hi", Some("Sam")), "hi\n\nSam\n");
        assert_eq!(sign("hi", Some("")), "hi");
        assert_eq!(sign("hi", None), "hi");
    }

    #[test]
    fn message_id_anchored_to_domain() {
        let id = mint_message_id("me@acme.com");
        assert!(id.ends_with("@acme.com>"));
        assert!(id.starts_with('<'));
    }
}
