use lambda_http::tracing::info;
use lambda_http::{Body, Error, Request, RequestExt, Response};

use lambda_rust_api::{get_trips, get_unix_timestamp_ms};

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract input parameters from the request
    let query_params = event.query_string_parameters();

    let from_ms = query_params
        .first("from_ms")
        .and_then(|from_ms| from_ms.parse::<i64>().ok())
        .unwrap_or(get_unix_timestamp_ms() - 24 * 60 * 60 * 1000);
    let n_results = query_params
        .first("n_results")
        .and_then(|n_results| n_results.parse::<i64>().ok())
        .unwrap_or(100);

    info!("Client requested data from {from_ms} with {n_results} results");

    // Get the trips data from the database
    let trips = get_trips(from_ms, n_results, None).await?;

    // Serialize the trips data to JSON
    let message = serde_json::to_string(&trips)?;

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(message.into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_http::{Request, RequestExt};
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_success_response() {
        let mut query_string_parameters: HashMap<String, String> = HashMap::new();
        query_string_parameters.insert("from_ms".into(), "1719783621000".into());
        query_string_parameters.insert("n_results".into(), "50".into());

        let request = Request::default().with_query_string_parameters(query_string_parameters);

        let response = function_handler(request).await.unwrap();

        // Check that the response is 200 OK
        assert_eq!(response.status(), 200);
    }
}
