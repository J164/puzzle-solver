use puzzle_utils::{parse_nonogram_rules, solve_nonogram};
use serde::{Deserialize, Serialize};
use serde_json::json;
use vercel_runtime::{
    http::bad_request, Body, Error, Request, RequestPayloadExt, Response, StatusCode,
};

#[derive(Deserialize)]
struct NonogramOptions {
    width: usize,
    height: usize,
    col: String,
    row: String,
}

#[derive(Serialize)]
struct APIError {
    message: String,
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let payload = req.payload::<NonogramOptions>();

    match payload {
        Err(..) => bad_request(APIError {
            message: "Invalid payload".to_string(),
        }),
        Ok(None) => bad_request(APIError {
            message: "Empty payload".to_string(),
        }),
        Ok(Some(NonogramOptions {
            width,
            height,
            col,
            row,
        })) => {
            let col = match parse_nonogram_rules(&col, width) {
                Ok(col) => col,
                Err(nonogram_error) => {
                    return bad_request(APIError {
                        message: nonogram_error.to_string(),
                    })
                }
            };

            let row = match parse_nonogram_rules(&row, height) {
                Ok(row) => row,
                Err(nonogram_error) => {
                    return bad_request(APIError {
                        message: nonogram_error.to_string(),
                    })
                }
            };

            let nonogram = match solve_nonogram(&col, &row) {
                Ok(row) => row,
                Err(nonogram_error) => {
                    return bad_request(APIError {
                        message: nonogram_error.to_string(),
                    })
                }
            };

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(json!({ "grid": nonogram }).to_string().into())?)
        }
    }
}
