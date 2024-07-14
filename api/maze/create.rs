use puzzle_utils::{create_maze, MazeAlgorithm};
use serde::{Deserialize, Serialize};
use serde_json::json;
use vercel_runtime::{
    http::bad_request, Body, Error, Request, RequestPayloadExt, Response, StatusCode,
};

#[derive(Deserialize)]
struct MazeOptions {
    width: usize,
    height: usize,
}

#[derive(Serialize)]
struct APIError {
    message: String,
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let payload = req.payload::<MazeOptions>();

    match payload {
        Err(..) => bad_request(APIError {
            message: "Invalid payload".to_string(),
        }),
        Ok(None) => bad_request(APIError {
            message: "Empty payload".to_string(),
        }),
        Ok(Some(MazeOptions { width, height })) => {
            let (nodes, directions) = create_maze(width, height, MazeAlgorithm::RecursiveBacktrack);

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(
                    json!({ "nodes": nodes, "directions": directions })
                        .to_string()
                        .into(),
                )?)
        }
    }
}
