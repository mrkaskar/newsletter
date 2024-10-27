use secrecy::{ExposeSecret, SecretBox};
use sqlx::PgPool;
use std::net::TcpListener;
use ztp::{
    configuration::get_configuration,
    email_client::EmailClient,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("ztp".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect_lazy_with(configuration.database.with_db());

    let sender_email = configuration
        .email_client
        .sender()
        .expect("Invalid sender email address");
    let email_auth_token = configuration
        .email_client
        .authorization_token
        .expose_secret();

    let timeout = configuration.email_client.timeout();
    let email_client = EmailClient::new(
        configuration.email_client.base_url,
        sender_email,
        SecretBox::new(Box::new(email_auth_token.to_string())),
        timeout,
    )
    .expect("Failed to create email client");

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );

    let listener = TcpListener::bind(address).expect("Failed to bind random port");

    run(listener, connection_pool, email_client)?.await
}
