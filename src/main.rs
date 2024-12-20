use ztp::startup::Application;
use ztp::{
    configuration::get_configuration,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("ztp".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration)
        .await
        .expect("Failed to build app.");

    application.run_until_stopped().await?;

    Ok(())
}
