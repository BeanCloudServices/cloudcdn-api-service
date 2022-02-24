use std::task::{Context, Poll};
// type Error = Box<dyn std::error::Error + Sync + Send + 'static>;
use async_trait::async_trait;
use aws_lambda::func;
// use beancloud_cloudresource_openapi_cloudcdn::lambda::invoke_openapi_function;
use lambda_http::{service_fn, Error, IntoResponse, Request, RequestExt, Response};

#[tokio::main]
async fn main() -> Result<(), Error> {
    print!("Start handle student lambda");
    lambda_http::run(service_fn(func)).await?;
    // lambda_http::run(service_fn(api_func)).await?;
    Ok(())
}
//
// pub async fn api_func(request: Request) -> Result<impl IntoResponse, Error> {
//     let api_impl = LambdaFunctionApi {
//         context: EmptyContext,
//     };
//
//     invoke_openapi_function(api_impl, request).await
// }
//
// pub struct LambdaFunctionApi<C> {
//     context: C,
// }
//
// #[async_trait]
// impl<C> ApiNoContext<C> for LambdaFunctionApi<C>
// where
//     C: Send + Sync + 'static,
// {
//     fn poll_ready(
//         &self,
//         _cx: &mut Context,
//     ) -> Poll<Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>> {
//         Poll::Ready(Ok(()))
//     }
//
//     fn context(&self) -> &C {
//         &self.context
//     }
//
//     async fn create_distribution(
//         &self,
//         distribution_upsert_request: DistributionUpsertRequest,
//     ) -> Result<CreateDistributionResponse, swagger::ApiError> {
//         todo!()
//     }
//
//     async fn get_distribution(
//         &self,
//         id: Uuid,
//     ) -> Result<GetDistributionResponse, swagger::ApiError> {
//         Ok(GetDistributionResponse::SuccessfulOperation(
//             DistributionUpsert {
//                 idempotency_key: "idemp".to_string(),
//                 id: Option::from(Uuid::new_v4()),
//             },
//         ))
//     }
// }
