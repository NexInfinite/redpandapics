pub mod rejection_handler_mod {
    use std::convert::{Infallible};
    use std::error::Error;

    use warp::http::StatusCode;
    use serde_derive::{Serialize};
    use warp::{Rejection, Reply};

    #[derive(Serialize)]
    struct ErrorMessage {
        code: u16,
        message: String,
    }

    pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
        let code;
        let message;

        if err.is_not_found() {
            code = StatusCode::NOT_FOUND;
            message = "Endpoint not found, check out /endpoints for a list of endpoints";
        } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
            message = match e.source() {
                Some(cause) => {
                    if cause.to_string().contains("denom") {
                        "FIELD_ERROR: denom"
                    } else {
                        "BAD_REQUEST"
                    }
                }
                None => "BAD_REQUEST",
            };
            code = StatusCode::BAD_REQUEST;
        } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
            code = StatusCode::METHOD_NOT_ALLOWED;
            message = "METHOD_NOT_ALLOWED";
        } else {
            eprintln!("unhandled rejection: {:?}", err);
            code = StatusCode::INTERNAL_SERVER_ERROR;
            message = "UNHANDLED_REJECTION";
        }

        let json = warp::reply::json(&ErrorMessage {
            code: code.as_u16(),
            message: message.into(),
        });

        Ok(warp::reply::with_status(json, code))
    }
}