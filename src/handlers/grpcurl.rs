use std::process::Command;
use std::sync::{Arc, Mutex};

use axum::{
    extract::{self, Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    Json,
};
use miette::{miette, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::handlers::{ConnectionState, PathParams, RequestBody};
use crate::models::request::Request as RequestModel;

#[derive(Debug, Serialize, Deserialize)]
pub struct GrpcRequest {
    pub url: String,
    pub method: String,
    pub headers: Option<String>,
    pub body: Option<String>,
    pub user_email: String,
}

pub async fn execute_grpcurl_request(
    State(state): ConnectionState,
    Path(path): Path<PathParams>,
    extract::Json(body): extract::Json<RequestBody>,
) -> impl IntoResponse {
    let result: Result<Html<String>> = (|| {
        let grpc_request = body.request.ok_or_else(|| miette!("No request data provided"))?;
        let email = path.email.unwrap_or_default();
        
        // Build the grpcurl command
        let mut command = Command::new("grpcurl");
        
        // Add common flags
        command.arg("-v");
        
        // Add headers if present
        if let Some(headers) = &grpc_request.headers {
            command.arg("-H").arg(headers);
        }
        
        // Add the URL and method
        command.arg(format!("{} {}", &grpc_request.url, &grpc_request.method));
        
        // Add request data if present
        if let Some(body) = &grpc_request.body {
            command.arg("-d").arg(body);
        }
        
        // Execute the command
        let output = command.output().map_err(|e| miette!("Failed to execute command: {}", e))?;
        
        // Process the output
        let status = if output.status.success() {
            StatusCode::OK
        } else {
            StatusCode::BAD_REQUEST
        };
        
        let response = String::from_utf8_lossy(&output.stdout).to_string();
        let error = String::from_utf8_lossy(&output.stderr).to_string();
        
        // Extract status code from response
        let status_code = extract_status_code(&error).unwrap_or_else(|| "200".to_string());
        
        // Save the request to the database
        if let Err(e) = save_request(&state, &email, &grpc_request, &status_code, &response) {
            eprintln!("Failed to save gRPC request: {e:?}");
        }
        
        // Format the response as HTML
        let html_response = if !error.is_empty() {
            format!("<p>$  error: {}</p>", error)
        } else {
            format!(
                r#"
                $  status: {}<br /><br />
                <textarea id="response-textarea" readonly>{}
                </textarea>
                "#,
                status_code,
                response
            )
        };
        
        Ok(Html(html_response))
    })();
    
    match result {
        Ok(html) => html.into_response(),
        Err(e) => {
            eprintln!("Error in execute_grpc_request: {e:?}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Html("<p>$  error: Internal server error</p>")
            ).into_response()
        }
    }
}

fn extract_status_code(error_output: &str) -> Option<String> {
    let re = match Regex::new(r"Code: ([A-Za-z_]+)") {
        Ok(re) => re,
        Err(e) => {
            eprintln!("Failed to compile regex: {}", e);
            return None;
        }
    };
    re.captures(error_output)
        .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
}

fn save_request(
    state: &Arc<Mutex<rusqlite::Connection>>,
    email: &str,
    request: &RequestModel,
    status: &str,
    response: &str,
) -> Result<()> {
    let db = state.lock().map_err(|e| miette!("Failed to lock database: {}", e))?;
    
    let result = db.execute(
        "INSERT INTO request (user_email, url, method, origin, headers, body, status, date, hidden) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        rusqlite::params![
            email,
            request.url,
            request.method.to_string(),
            request.origin.as_deref().unwrap_or(""),
            request.headers.as_deref().unwrap_or(""),
            response,
            status,
            chrono::Local::now().to_rfc3339(),
            false,
        ],
    );
    
    if let Err(e) = result {
        eprintln!("Failed to save request to database: {}", e);
        return Err(miette!("Database error: {}", e));
    }
    
    Ok(())
}

