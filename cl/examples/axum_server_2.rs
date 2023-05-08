use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use cl::ocl_v4::OpenClBlock;
use cl::server::{GetRequest, Payload, SaveRequest};
use opencl3::types::cl_int;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
// use axum::extract::State;

// #[derive(Clone)]
// struct AppState {}

#[tokio::main]
async fn main() {
    let ocl_block = OpenClBlock::new().expect("OpenClBlock::new()");
    let _vector_add_kernel = ocl_block.create_vector_add_kernel();
    let _vector_extract_kernel = ocl_block.create_vector_extract_kernel();

    let shared_state = Arc::new(Mutex::new(ocl_block));

    // let state = AppState {};

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // .route("/ocl_save", post(save_buffer))
        .route(
            "/ocl_save",
            post({
                let shared_state = Arc::clone(&shared_state);
                move |body| save_buffer(body, shared_state)
            }),
        )
        // .route("/ocl_get", post(get_buffer))
        .route(
            "/ocl_get",
            post({
                let shared_state = Arc::clone(&shared_state);
                move |body| get_buffer(body, shared_state)
            }),
        )
        .with_state(shared_state);
    // .with_state(state);

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

async fn save_buffer(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<SaveRequest>,
    // ocl_block: Arc<OpenClBlock>,
    ocl: Arc<Mutex<OpenClBlock>>,
    // State(state): State<OpenClBlock>,
) -> (StatusCode, Json<Payload>) {
    // println!("payload {payload:?}");
    println!("payload {:?}", payload.value.as_bytes());

    let mut p = Payload {
        index: 0,
        value: "".to_string(),
    };
    let mut ocl_block = ocl.lock().unwrap();

    let kernel = ocl_block.create_vector_add_kernel();

    // insert your application logic here
    let index: cl_int;
    match ocl_block.get_global_array_index() {
        Ok(i) => {
            index = i;
        }
        Err(_) => return (StatusCode::BAD_REQUEST, Json(p)),
    }

    match ocl_block.enqueue_buffer(&kernel, payload.value.as_bytes(), index) {
        Ok(_) => {
            // pass
        }
        Err(_) => return (StatusCode::BAD_REQUEST, Json(p)),
    }

    p.index = index;
    p.value = payload.value;

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(p))
}

async fn get_buffer(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<GetRequest>,
    // ocl_block: Arc<OpenClBlock>,
    ocl: Arc<Mutex<OpenClBlock>>,
    // State(state): State<OpenClBlock>
) -> (StatusCode, Json<Payload>) {
    // println!("payload {payload:?}");
    println!("index {:?}", payload.index);

    let mut ocl_block = ocl.lock().unwrap();
    ocl_block.show_global_arrays();

    let kernel = ocl_block.create_vector_extract_kernel();
    let vec = ocl_block.dequeue_buffer(&kernel, payload.index).unwrap();

    let p = Payload {
        index: payload.index,
        value: String::from_utf8(vec).unwrap(),
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::OK, Json(p))
}
