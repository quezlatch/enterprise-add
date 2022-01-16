use lambda_runtime::{handler_fn, Context, Error};
use log::info;
#[macro_use]
extern crate serde_derive;

#[derive(Deserialize, Clone)]
struct AddEvent {
    numbers: Vec<i32,>
}

#[derive(Serialize, Clone)]
struct Output {
    result: i32
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn func(event: AddEvent, _: Context) -> Result<Output, Error> {
    let result: i32 = event.numbers.iter().sum();

    info!("summing result = {}", result);

    Ok(Output {
        result: result
    })
}