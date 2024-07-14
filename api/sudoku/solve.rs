use puzzle_utils::solve_sudoku;
use serde::{Deserialize, Serialize};
use serde_json::json;
use vercel_runtime::{
    http::bad_request, Body, Error, Request, RequestPayloadExt, Response, StatusCode,
};

#[derive(Deserialize)]
struct SudokuOptions {
    entries: Vec<u8>,
}

#[derive(Serialize)]
struct APIError {
    message: String,
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let payload = req.payload::<SudokuOptions>();

    match payload {
        Err(..) => bad_request(APIError {
            message: "Invalid payload".to_string(),
        }),
        Ok(None) => bad_request(APIError {
            message: "Empty payload".to_string(),
        }),
        Ok(Some(SudokuOptions { entries })) => {
            let sudoku = match solve_sudoku(&entries) {
                Ok(sudoku) => sudoku,
                Err(sudoku_error) => {
                    return bad_request(APIError {
                        message: sudoku_error.to_string(),
                    })
                }
            };

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(json!({ "puzzle": sudoku }).to_string().into())?)
        }
    }
}
