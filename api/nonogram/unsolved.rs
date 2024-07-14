use puzzle_utils::{image_to_png_bytes, print_nonogram};
use serde::{Deserialize, Serialize};
use vercel_runtime::{
    http::bad_request, Body, Error, Request, RequestPayloadExt, Response, StatusCode,
};

#[derive(Deserialize)]
struct PrintOptions {
    width: u32,
    height: u32,
    col: Vec<Vec<usize>>,
    row: Vec<Vec<usize>>,
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
            col,
            row,
        })) => {
            let nonogram = match print_nonogram(width, height, &col, &row) {
                Ok(nonogram) => nonogram,
                Err(nonogram_error) => return bad_request(nonogram_error.to_string()),
            };

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "image/png")
                .body(image_to_png_bytes(&nonogram)?.into())?)
        }
    }
}
