use lambda_http::{
    handler, http,
    lambda_runtime::{self, Context, Error},
    IntoResponse, Request, Response,
};
use tracing::{info};
use http::StatusCode;
use logic::{AddOperation};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_ansi(false)
        .without_time()
        .init();

    lambda_runtime::run(handler(func)).await?;
    Ok(())
}

#[tracing::instrument]
async fn func(request: Request, _: Context) -> Result<impl IntoResponse, Error> {
    match serde_json::from_slice::<AddOperation>(request.body().as_ref()) {
        Ok(add) => {
            info!("all good");
            let result = add.to_output();
            Ok(serde_json::json!(result).into_response())
        }
        Err(_) => Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body("bad request".into())
            .expect("err creating response")),
    }

}
