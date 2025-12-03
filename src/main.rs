use todo_list_v1::response::ApiResult;
use todo_list_v1::response::resp::ApiResponse;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // build our application with a route
    let app = axum::Router::new().route("/", axum::routing::get(get_current_version_handler));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

pub async fn get_current_version_handler() -> ApiResult<ApiResponse<String>> {
    // tracing::info!("Querying current version ...");
    // return the current version
    Ok(ApiResponse::success("v0.1.1".into()))
}
