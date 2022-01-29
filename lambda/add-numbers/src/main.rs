use lambda_runtime::{handler_fn, Context, Error};
use log::info;
use logic::{AddOperation, OperationResult};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn func(event: AddOperation, _: Context) -> Result<OperationResult, Error> {
    let result = event.to_output();

    info!("summing result = {}", result.get_result());

    Ok(result)
}