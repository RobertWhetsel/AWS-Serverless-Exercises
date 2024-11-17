use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde_json::{json, Value};
use tracing_subscriber::FmtSubscriber;

async fn handler(event: LambdaEvent<Value>) -> Result<Value, Error> {
    // Log the event
    tracing::info!("Event: {:?}", event);

    // Return the response
    Ok(json!({
        "result": "Hello world!"
    }))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Initialize tracing
    let subscriber = FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)?;

    // Initialize the AWS Lambda runtime
    run(service_fn(handler)).await
}
