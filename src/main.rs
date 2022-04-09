mod utils;
use lambda_runtime::{service_fn, LambdaEvent, Error};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn func(_event: LambdaEvent<Value>) -> Result<Value, Error> {
    tokio::task::spawn_blocking(|| {
        let price_and_rates = utils::get_price_and_rates();
        utils::send_email(&price_and_rates);
    })
    .await
    .expect("Blocking task panicked");
    Ok(json!({ "message": "success" }))
}
