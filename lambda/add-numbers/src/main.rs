use lambda_runtime::{handler_fn, Context, Error};
use tracing::{info};
use logic::{AddOperation, OperationResult};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_ansi(false)
        .without_time()
        .init();
        
    let func = handler_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

#[tracing::instrument]
async fn func(event: AddOperation, _: Context) -> Result<OperationResult, Error> {
    let result = event.to_output();

    info!("summing result = {}", result.get_result());

    Ok(result)
}