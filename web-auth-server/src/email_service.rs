use lettre::transport::smtp::authentication::Credentials;
use lettre::{Address, Message, SmtpTransport, Transport};

use native_tls::{Protocol, TlsConnector};

use crate::{errors::AuthError, models::Confirmation, vars};

pub fn send_confirmation_mail(confirmation: &Confirmation) -> Result<(), AuthError> {
    let domain_url = vars::domain_url();
    let expires = confirmation
        .expires_at
        .format("%I:%M %p %A, %-d %B, %C%y")
        .to_string();
    let html_text = format!(
        "Please click on the link below to complete registration. <br/>
       <a href=\"{domain}/register?id={id}&email={email}\">Complete registration</a> <br/>
      This link expires on <strong>{expires}</strong>",
        domain = domain_url,
        id = confirmation.id,
        email = confirmation.email,
        expires = expires
    );
    let plain_text = format!(
        "Please visit the link below to complete registration:\n
      {domain}/register.html?id={id}&email={email}\n
      This link expires on {expires}.",
        domain = domain_url,
        id = confirmation.id,
        email = confirmation.email,
        expires = expires
    );
    let address = confirmation.email.clone();
    let address: Vec<&str> = address.split("@").collect();
    let email = Message::builder()
        .to(format!(
            "{} <{}>",
            address.get(0).unwrap(),
            confirmation.email.clone(),
        )
        .parse()
        .unwrap())
        .from("Noreplay <noreplay@auth-service.com>".parse().unwrap())
        .subject("Complete your registration on our one-of-a-kind Auth Service")
        .body(html_text)
        .unwrap();

    let credentials = Credentials::new(vars::smtp_username(), vars::smtp_password());
    let mailer = SmtpTransport::relay(vars::smtp_host().as_str())
        .unwrap()
        .credentials(credentials)
        .build();
    match mailer.send(&email) {
        Ok(_) => {
            println!("Email is sent");
            Ok(())
        }
        Err(err) => {
            println!("Could not send email: {:?}", err);
            Err(AuthError::ProcessError(String::from(
                "Could not send email",
            )))
        }
    }
}
