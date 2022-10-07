use lambda_runtime::{handler_fn, Context, Error};
use serde_json::{json, Value};

use crate::get_points;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn func(event: Value, context: Context) -> Result<Value, Error> {
    Ok(event)
}
