use std::env;
use std::sync::mpsc::Receiver;

use lettre::{
    message::SinglePart, transport::smtp::authentication::Credentials, Message, SmtpTransport,
    Transport,
};

use crate::models::User;

pub struct Email {
    pub to: String,
    pub subject: String,
    pub body: String,
}

fn get_real_name_display(user: &User) -> String {
    let mut real_name_display = String::new();

    if let Some(first_name) = &user.first_name {
        real_name_display.push_str(first_name);

        if user.last_name.is_some() {
            real_name_display.push(' ');
        }
    }

    if let Some(last_name) = &user.last_name {
        real_name_display.push_str(last_name);
    }

    real_name_display
}

fn get_display_name(user: &User) -> String {
    let mut display_name = get_real_name_display(user);

    if display_name.is_empty() {
        display_name.push_str(&user.username);
    }

    display_name
}

pub fn create_assignment_email_body(
    assigner: &User,
    ticket_subject: &str,
    ticket_id: i64,
) -> String {
    let assigner_name = get_display_name(assigner);

    format!(
        "<p>{} assigned you to ticket '{}'.</p><p><a href=\"{}/{}\">Link to ticket</a></p>",
        assigner_name,
        ticket_subject,
        env::var("TIRA_EMAIL_TICKET_LINK").unwrap(),
        ticket_id
    )
}

pub fn create_comment_email_body(
    commenter: &User,
    comment_content: &str,
    ticket_subject: &str,
    ticket_id: i64,
) -> String {
    let commenter_name = get_display_name(commenter);

    format!(
        "<p>{} added a comment to ticket '{}'.</p><p>{}</p><p><a href=\"{}/{}\">Link to ticket</a></p>",
        commenter_name,
        ticket_subject,
        comment_content,
        env::var("TIRA_EMAIL_TICKET_LINK").unwrap(),
        ticket_id
    )
}

pub fn create_ticket_creation_email_body(
    creator: &User,
    ticket_subject: &str,
    ticket_description: &str,
    created_ticket_id: i64,
) -> String {
    let creator_name = get_display_name(creator);

    format!(
        "<p>{} created ticket '{}'.</p>{}<p><a href=\"{}/{}\">Link to ticket</a></p>",
        creator_name,
        ticket_subject,
        ticket_description,
        env::var("TIRA_EMAIL_TICKET_LINK").unwrap(),
        created_ticket_id
    )
}

pub fn handle_emails(rx: Receiver<Email>) {
    // TODO: be able to clean this up on shutdown
    loop {
        let email = rx.recv().unwrap();
        send_email(email);
    }
}

pub fn send_email(e: Email) {
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
        .to(e.to.parse().unwrap())
        .subject(e.subject)
        .singlepart(SinglePart::html(e.body))
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
