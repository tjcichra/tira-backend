use std::env;

use lettre::{
    message::SinglePart, transport::smtp::authentication::Credentials, Message, SmtpTransport,
    Transport,
};

use crate::models::User;

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

pub fn send_email(email_address: &str, subject: &str, body: String) {
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
        .subject(subject)
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
