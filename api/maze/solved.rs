use puzzle_utils::{image_to_png_bytes, print_maze, print_maze_solution, MazeDirection, MazeNode};
use serde::{Deserialize, Serialize};
use vercel_runtime::{
    http::bad_request, Body, Error, Request, RequestPayloadExt, Response, StatusCode,
};

#[derive(Deserialize)]
struct PrintOptions {
    width: u32,
    height: u32,
    nodes: Vec<MazeNode>,
    directions: Vec<MazeDirection>,
}

#[derive(Serialize)]
struct APIError {
    message: String,
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let payload = req.payload::<PrintOptions>();

    match payload {
        Err(..) => bad_request(APIError {
            message: "Invalid payload".to_string(),
        }),
        Ok(None) => bad_request(APIError {
            message: "Empty payload".to_string(),
        }),
        Ok(Some(PrintOptions {
            width,
            height,
            nodes,
            directions,
        })) => {
            let unsolved = match print_maze(width, height, &nodes) {
                Ok(unsolved) => unsolved,
                Err(maze_error) => return bad_request(maze_error.to_string()),
            };

            let maze = match print_maze_solution(unsolved, &directions) {
                Ok(maze) => maze,
                Err(maze_error) => return bad_request(maze_error.to_string()),
            };

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "image/png")
                .body(image_to_png_bytes(&maze)?.into())?)
        }
    }
}
