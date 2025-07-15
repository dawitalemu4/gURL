#[cfg(windows)]
use std::os::windows::process::CommandExt;

use std::{
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use miette::{Result, miette};
use regex::Regex;

use crate::{
    handlers::{ConnectionState, PathParams, RequestBody, create_request},
    models::request::Request,
};

pub async fn execute_grpcurl_request(
    State(state): ConnectionState,
    Path(path): Path<PathParams>,
    Json(request): Json<RequestBody>,
) -> Response {
    let res: Result<Response> = (async || {
        #[cfg(windows)]
        let output = Command::new("grpcurl").raw_arg(&request.command)
            .output()
            .map_err(|e| miette!("Failed to execute grpcurl command, may not be installed: {}", e))?;

        #[cfg(not(windows))]
        let output = Command::new("grpcurl").args(&request.command.split_whitespace().collect::<Vec<_>>())
            .output()
            .map_err(|e| miette!("Failed to execute grpcurl command, may not be installed: {}", e))?;

        let response = String::from_utf8_lossy(&output.stdout).to_string();
        let error = String::from_utf8_lossy(&output.stderr).to_string();

        let status_regex = Regex::new(r"Code:\s*(\w+)").map_err(|e| miette!("Could not unwrap status regex: {e}"))?;
        let status = if let Some(caps) = status_regex.captures(&error) {
            caps.get(1).map(|m| m.as_str()).unwrap_or("UNKNOWN")
        } else if output.status.success() {
            "OK"
        } else {
            "ERROR"
        };

        if response.is_empty() {
            return Ok((
                    StatusCode::OK,
                    Html(format!("$  error: {}<br /><br />status: {}", error, status)),
            )
                .into_response());
        }

        let method = if request.command.contains("list") {
            Some("list".to_string())
        } else if let Some(service) = request.command.split(".").last() {
            if service.contains("/") {
                Some(service.split("/").last().unwrap_or_default().to_string())
            } else {
                Some(service.to_string())
            }
        } else { None };

        create_request(
            State(state),
            Path(path.clone()),
            Json(Request {
                id: None,
                user_email: path.email,
                command: request.command,
                status: Some(status.to_string()),
                method,
                date: SystemTime::now()
                    .duration_since(UNIX_EPOCH).unwrap_or_default()
                    .as_millis().to_string(),
                hidden: false
            }),
        ).await;

        let exit_code = output.status.code().unwrap_or(0);
        if exit_code == 1 && error.contains("connection refused") {
            return Ok((StatusCode::OK, Html(format!(
                "<p>$  error: Connection refused, probably can't connect to gRPC server. Check if the server is running and the address is correct.</p>"
            ))).into_response());
        } else if exit_code != 0 && !output.status.success() {
            if error.contains("Unimplemented") {
                return Ok((
                    StatusCode::OK,
                    Html(format!(
                        "<p>$  error: gRPC method not implemented on the server</p>"
                    )),
                )
                    .into_response());
            } else if error.contains("InvalidArgument") {
                return Ok((
                    StatusCode::OK,
                    Html(format!(
                        "<p>$  error: Invalid arguments provided to gRPC call</p>"
                    )),
                )
                    .into_response());
            } else {
                return Ok((
                    StatusCode::OK,
                    Html(format!("<p>$  error: gRPC call failed - {}</p>", error)),
                )
                    .into_response());
            }
        }

        let error_response_regex = Regex::new(r"(?i)error|failed|exception").unwrap();
        if error_response_regex.is_match(&response) || error_response_regex.is_match(&error) {
            return Ok((
                StatusCode::OK,
                Html(format!("$  error: {}<br /><br />status: {}", error, status)),
            )
                .into_response());
        }

        let html_response = format!(
            r#"
            $  status: {status}
            <br /><br />
            <textarea id="response-textarea" readonly>{response}&#013;</textarea>
        "#
        );

        Ok((StatusCode::OK, Html(html_response)).into_response())
    })().await;

    match res {
        Ok(res) => res,
        Err(e) => panic!("{e}"),
    }
}
