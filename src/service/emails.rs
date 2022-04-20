use std::env;

use lettre::{SmtpTransport, Transport, transport::smtp::authentication::Credentials, message::{MultiPartBuilder, header::{ContentType, ContentTransferEncoding}, Body, Attachment}, Message};

pub fn send_email(email_address: &str) {
    let content_type = ContentType::parse("message/rfc822").unwrap();
    let body = Body::new_with_encoding("tim".to_string().into_bytes(), ContentTransferEncoding::EightBit).unwrap();
    let attachment = Attachment::new("original.eml".to_string()).body(body, content_type);

    let email = Message::builder()
        .from(format!("{}@{}", env::var("TIRA_EMAIL_USERNAME").unwrap(), "jrcichra.dev").parse().unwrap())
        .to(email_address.parse().unwrap())        
        .subject("This is a test, a tim test.")
        .multipart(
            MultiPartBuilder::new()
                .kind(lettre::message::MultiPartKind::Mixed)
                .singlepart(attachment),
        )
        .unwrap();

    let creds = Credentials::new(env::var("TIRA_EMAIL_USERNAME").unwrap(), env::var("TIRA_EMAIL_PASSWORD").unwrap());

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