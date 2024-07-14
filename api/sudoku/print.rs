use puzzle_utils::{image_to_png_bytes, print_sudoku};
use serde::{Deserialize, Serialize};
use vercel_runtime::{
    http::bad_request, Body, Error, Request, RequestPayloadExt, Response, StatusCode,
};

#[derive(Deserialize)]
struct PrintOptions {
    puzzle: Vec<u8>,
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
        Ok(Some(PrintOptions { puzzle })) => {
            let sudoku = match print_sudoku(&puzzle) {
                Ok(sudoku) => sudoku,
                Err(sudoku_error) => return bad_request(sudoku_error.to_string()),
            };

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "image/png")
                .body(image_to_png_bytes(&sudoku)?.into())?)
        }
    }
}
