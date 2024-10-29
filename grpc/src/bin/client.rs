use grpc::basic::QueryRequest;
use grpc::query::user_stats_client::UserStatsClient;
use tonic::transport::Endpoint;
use tonic::Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = Endpoint::from_static("http://127.0.0.1:50052");

    let mut query_cli = UserStatsClient::connect(addr).await?;
    let request = Request::new(QueryRequest {
        timestamps: Default::default(),
        ids: Default::default(),
    });
    let response = query_cli.query_once(request).await?;
    println!("query response: {:?}", response.into_inner());

    Ok(())
}
