use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use cl::ocl_v2::{BlockConfig, KB_N, OpenClBlock, PIPE_BLOCKS, PIPE_MAX_PACKETS};
use cl::server::{Payload, PayloadRequest};

#[tokio::main]
async fn main() {
    let ocl_block = OpenClBlock::new(BlockConfig {
        buffer_size: KB_N,
        pipes: KB_N,
        pipe_max_packets: PIPE_MAX_PACKETS,
    })
        .expect("OpenClBlock::new()");
    let mut pipe_blocks = ocl_block
        .generate_pipes(PIPE_BLOCKS)
        .expect("ocl_block.generate_pipes()");

    let shared_state = Arc::new(pipe_blocks);

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user))
        .with_state(shared_state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<PayloadRequest>,
) -> (StatusCode, Json<Payload>) {
    // insert your application logic here
    let p = Payload {
        index: 0,
        value: payload.value
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(p))
}
