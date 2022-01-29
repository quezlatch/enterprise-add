use lambda_http::{
    handler, http,
    lambda_runtime::{self, Context, Error},
    IntoResponse, Request, Response,
};
use log::info;
use http::StatusCode;
use logic::{AddOperation};

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(handler(func)).await?;
    Ok(())
}

async fn func(request: Request, _: Context) -> Result<impl IntoResponse, Error> {
    info!("uri = {}", request.uri());

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
