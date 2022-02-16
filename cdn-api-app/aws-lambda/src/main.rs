type Error = Box<dyn std::error::Error + Sync + Send + 'static>;
use aws_lambda::func;
use lambda_http::{handler, lambda_runtime};

#[tokio::main]
async fn main() -> Result<(), Error> {
    print!("Start handle student lambda");
    lambda_runtime::run(handler(func)).await?;
    Ok(())
}
