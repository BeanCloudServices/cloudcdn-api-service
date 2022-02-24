use crate::models::DistributionUpsertRequest;
use crate::{ApiNoContext, GetDistributionResponse};
use lambda_http::http::header::{HeaderName, CONTENT_TYPE};
use lambda_http::http::{method, uri::Uri, HeaderValue, StatusCode};
use lambda_http::{
    Body, Context, Context as LambdaContext, Error, IntoResponse, Request, RequestExt, Response,
};
use std::str::FromStr;
use uuid::Uuid;

pub async fn invoke_openapi_function<T, C>(
    api: T,
    request: Request,
) -> Result<impl IntoResponse, Error>
where
    T: ApiNoContext<C> + Send + 'static,
    C: Send + Sync + 'static,
{
    let request_payload_option = request.payload::<DistributionUpsertRequest>();
    let path_parameters = request.path_parameters();
    let query_string_parameters = request.query_string_parameters();
    let id_query_param_option = query_string_parameters.first("id");
    let (parts, body) = request.into_parts();
    let (method, uri, headers) = (parts.method, parts.uri, parts.headers);

    let path = paths::GLOBAL_REGEX_SET.matches(uri.path());

    let response = match method {
        // GetDistribution - GET /students/{id}
        method::Method::GET if path.matched(paths::ID_STUDENTS_ID) => {
            // Path parameters
            let path: &str = &uri.path().to_string();
            let path_params = paths::REGEX_STUDENTS_ID.captures(&path).unwrap_or_else(|| {
                panic!(
                    "Path {} matched RE STUDENTS_ID in set but failed match against \"{}\"",
                    path,
                    paths::REGEX_STUDENTS_ID.as_str()
                )
            });

            let id_param = path_parameters.first("id");
            // if let Some(id) = id_param {
            //     println!("id found");
            //     Some(uuid::Uuid::parse_str(id).unwrap())
            // } else {
            //     println!("id not found");
            //     None
            // }

            let result = api
                .get_distribution(Uuid::from_str(id_query_param_option.unwrap()).unwrap())
                .await;

            let mut response = Response::new(Body::Empty);

            match result {
                Ok(rsp) => match rsp {
                    GetDistributionResponse::SuccessfulOperation(body) => {
                        *response.status_mut() = StatusCode::from_u16(200)
                            .expect("Unable to turn 200 into a StatusCode");
                        response.headers_mut().insert(
                            CONTENT_TYPE,
                            HeaderValue::from_str("application/json")
                                .expect("Unable to create Content-Type header for GET_DISTRIBUTION_SUCCESSFUL_OPERATION"));
                        let body =
                            serde_json::to_string(&body).expect("impossible to fail to serialize");
                        *response.body_mut() = Body::from(body);
                    }
                    GetDistributionResponse::InvalidInput => {
                        *response.status_mut() = StatusCode::from_u16(400)
                            .expect("Unable to turn 400 into a StatusCode");
                    }
                },

                Err(_) => {
                    // Application code returned an error. This should not happen, as the implementation should
                    // return a valid response.
                    *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                    *response.body_mut() = Body::from("An internal error occurred");
                }
            }
            response
        }
        _ => {
            // api.create_distribution(request_payload_option.unwrap_or(None).unwrap())
            //     .await;
            let mut response = Response::new(Body::Empty);
            response
        }
    };

    Ok(response)
}

mod paths {
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref GLOBAL_REGEX_SET: regex::RegexSet = regex::RegexSet::new(vec![
            r"^/cloudcdn/latest/distributions$",
            r"^/students/(?P<id>[^/?#]*)$"
        ])
        .expect("Unable to create global regex set");
    }
    // pub(crate) static ID_DISTRIBUTIONS: usize = 0;
    pub(crate) static ID_STUDENTS_ID: usize = 1;
    lazy_static! {
        pub static ref REGEX_STUDENTS_ID: regex::Regex =
            regex::Regex::new(r"^/students/(?P<id>[^/?#]*)$")
                .expect("Unable to create regex for STUDENTS_ID");
    }
}
