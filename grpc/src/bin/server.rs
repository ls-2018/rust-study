use futures_util::StreamExt;
use grpc::basic::{QueryRequest, User};
use grpc::query::user_stats_server::{UserStats, UserStatsServer};
#[allow(unused)]
use std::pin::Pin;
use tokio::sync::mpsc;
use tonic::codegen::tokio_stream;
use tonic::codegen::tokio_stream::Stream;
use tonic::codegen::tokio_stream::wrappers::ReceiverStream;
use tonic::transport::Server;
use tonic::{Request, Response, Status, Streaming};

const CHANNEL_SIZE: usize = 1024;

type ResponseStream = Pin<Box<dyn Stream<Item = Result<User, Status>> + Send>>;

#[derive(Default)]
pub struct UserService {}

#[tonic::async_trait]
impl UserStats for crate::UserService {
    async fn query_once(&self, request: Request<QueryRequest>) -> Result<Response<User>, Status> {
        println!("{:?}", request);
        // Err(Status::new(tonic::Code::Unimplemented, "not implemented yet"))
        Ok(Response::new(User {
            name: "test".to_string(),
            email: "test@test.com".to_string(),
        }))
    }

    type QueryStream = ResponseStream;

    async fn query(&self, request: Request<QueryRequest>) -> Result<Response<Self::QueryStream>, Status> {
        let stream = tokio_stream::iter(vec![Ok(User {
            email: "".to_string(),
            name: format!("{:?}", request.into_inner().timestamps),
        })]);
        Ok(Response::new(Box::pin(stream)))
    }

    type QueryStreamStream = ResponseStream;

    async fn query_stream(&self, request: Request<Streaming<QueryRequest>>) -> Result<Response<Self::QueryStreamStream>, Status> {
        let (tx, rx) = mpsc::channel(CHANNEL_SIZE);
        let mut req = request.into_inner();
        tokio::spawn(async move {
            while let Some(Ok(req)) = req.next().await {
                tx.send(Ok(User {
                    email: "email".to_string(),
                    name: format!("{:?}", req.timestamps),
                }))
                .await
                .unwrap();
            }
        });
        let stream = ReceiverStream::new(rx);
        Ok(Response::new(Box::pin(stream)))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50052".parse()?;
    println!("server starting at: {}", addr);
    Server::builder()
        // .tls_config(ServerTlsConfig::new()
        //     .identity(Identity::from_pem(&cert, &key))
        // )?
        .concurrency_limit_per_connection(256)
        .add_service(UserStatsServer::new(UserService::default()))
        .serve(addr)
        .await?;

    Ok(())
}
