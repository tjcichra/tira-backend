use std::env;

use lettre::{
    message::{
        header::{ContentTransferEncoding, ContentType},
        Attachment, Body, MultiPartBuilder, SinglePart,
    },
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};

use crate::models::{Ticket, User};

pub fn create_assignment_email_text(assigner: User, ticket: Ticket) -> String {
    let mut assigner_string = String::new();

    if let Some(first_name) = assigner.first_name {
        assigner_string.push_str(&first_name);
        assigner_string.push(' ');
    }

    if let Some(last_name) = assigner.last_name {
        assigner_string.push_str(&last_name);
        assigner_string.push(' ');
    }

    if assigner_string.is_empty() {
        assigner_string.push_str(&assigner.username);
    } else {
        assigner_string.push_str(&format!("({})", assigner.username));
    }

    format!(
        "<p>{} assigned you to ticket '{}'.</p><p><a href=\"{}/{}\">Link to ticket</a></p>",
        assigner_string,
        ticket.subject,
        env::var("TIRA_EMAIL_TICKET_LINK").unwrap(),
        ticket.id
    )
}

pub fn send_email(email_address: &str, body: String) {
    // let content_type = ContentType::parse("message/rfc822").unwrap();
    // let body = Body::new_with_encoding("tim".to_string().into_bytes(), ContentTransferEncoding::EightBit).unwrap();

    let email = Message::builder()
        .from(
            format!(
                "{}@{}",
                env::var("TIRA_EMAIL_USERNAME").unwrap(),
                "jrcichra.dev"
            )
            .parse()
            .unwrap(),
        )
        .to(email_address.parse().unwrap())
        .subject("This is a test, a tim test.")
        .singlepart(SinglePart::html(body))
        .unwrap();

    let creds = Credentials::new(
        env::var("TIRA_EMAIL_USERNAME").unwrap(),
        env::var("TIRA_EMAIL_PASSWORD").unwrap(),
    );

    // Open a remote connection to gmail
    let mailer = SmtpTransport::starttls_relay(&env::var("TIRA_EMAIL_URL").unwrap())
        .unwrap()
        .port(env::var("TIRA_EMAIL_SMTP_PORT").unwrap().parse().unwrap())
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}
