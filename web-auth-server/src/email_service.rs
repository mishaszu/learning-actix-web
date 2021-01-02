// use lettre::transport::smtp::authentication::Credentials;
// use lettre::{Address, Message, SmtpTransport, Transport};

use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::{extension::ClientId, ConnectionReuseParameters};
use lettre::{
    ClientSecurity, ClientTlsParameters, SendableEmail, SmtpClient, SmtpTransport, Transport,
};
use lettre_email::EmailBuilder;
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
       <a href=\"{domain}/register/{id}\">Complete registration</a> <br/>
      This link expires on <strong>{expires}</strong>",
        domain = domain_url,
        id = confirmation.id,
        expires = expires
    );
    let plain_text = format!(
        "Please visit the link below to complete registration:\n
      {domain}/register/{id}\n
      This link expires on {expires}.",
        domain = domain_url,
        id = confirmation.id,
        expires = expires
    );

    let mut tls_builder = TlsConnector::builder();
    tls_builder.min_protocol_version(Some(Protocol::Sslv3));
    tls_builder.use_sni(false);
    tls_builder.danger_accept_invalid_certs(true);
    tls_builder.danger_accept_invalid_hostnames(true);
    let tls_parameters = ClientTlsParameters::new(vars::smtp_host(), tls_builder.build().unwrap());

    let mut mailer = SmtpClient::new(
        (vars::smtp_host(), 587),
        ClientSecurity::Required(tls_parameters),
    )
    .unwrap()
    .authentication_mechanism(Mechanism::Plain) // Mechanism::Login does not work either
    .hello_name(ClientId::Domain(vars::smtp_host()))
    .credentials(Credentials::new(
        vars::smtp_username(),
        vars::smtp_password(),
    ))
    .connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
    .transport();

    println!("Email: {}", confirmation.email.clone());
    let email = EmailBuilder::new()
        .to(confirmation.email.clone())
        .from("ff0c51932d-e4f719@inbox.mailtrap.io")
        .subject("Complete your registration on our one-of-a-kind Auth Service")
        .html(html_text)
        .text(plain_text)
        .build()
        .unwrap();
    let email: SendableEmail = email.into();

    match mailer.send(email) {
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
