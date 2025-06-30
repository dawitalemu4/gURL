use std::process::Command;

use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use miette::{Result, miette};
use regex::Regex;

use crate::handlers::{ConnectionState, PathParams, RequestBody, create_request};
use crate::models::request::Request;

fn build_grpcurl_command(request: &Request) -> Command {
    let mut command = Command::new("grpcurl");

    command.arg("-v".to_string());

    if request.url.starts_with("localhost") || request.url.starts_with("127.0.0.1") {
        command.arg("-plaintext".to_string());
    }

    if let Some(metadata) = &request.metadata {
        for header in metadata.split(&['\n', ';'][..]) {
            let header = header.trim();
            if !header.is_empty() {
                command.arg("-H".to_string());
                command.arg(header.to_string());
            }
        }
    }

    if let Some(body) = &request.payload {
        command.arg("-d".to_string());
        command.arg(body.to_string());
    }

    if let Some(proto_file) = &request.proto_file {
        command.arg("-proto".to_string());
        command.arg(proto_file.to_string());
    }

    command.arg(request.url.clone());

    let service_method = if let Some(service) = &request.service {
        format!("{}/{}", service, request.method)
    } else {
        request.method.to_string()
    };
    command.arg(service_method);

    command
}

pub async fn execute_grpcurl_request(
    State(state): State<ConnectionState>,
    Path(path): Path<PathParams>,
    Json(body): Json<RequestBody>,
) -> Response {
    let res: Result<Response> = (|| {
        let request = body
            .request
            .clone()
            .ok_or_else(|| miette!("Cannot serialize Request from body"))?;

        let output = build_grpcurl_command(&request)
            .output()
            .map_err(|e| miette!("Failed to execute grpcurl command: {}", e))?;

        let response = String::from_utf8_lossy(&output.stdout).to_string();
        let error = String::from_utf8_lossy(&output.stderr).to_string();

        println!("grpcurl output - stdout: {}, stderr: {}", response, error);

        let exit_code = output.status.code().unwrap_or(0);

        let status_regex = Regex::new(r"Code:\s*(\w+)").unwrap();
        let status = if let Some(caps) = status_regex.captures(&error) {
            caps.get(1).map(|m| m.as_str()).unwrap_or("UNKNOWN")
        } else if output.status.success() {
            "OK"
        } else {
            "ERROR"
        };

        let mut final_request = request;
        final_request.status = status.to_string();

        create_request(
            state,
            Path(path),
            Json(RequestBody {
                request: Some(final_request),
                user: None,
            }),
        );

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
    })();

    match res {
        Ok(res) => res,
        Err(e) => panic!("{e}"),
    }
}
