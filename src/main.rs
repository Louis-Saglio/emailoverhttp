use axum::{extract::Json, http::StatusCode, response::IntoResponse, routing::post, Router};
use lettre::{
    transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport, Message,
    Tokio1Executor,
};
use serde::Deserialize;
use std::env;
use std::net::SocketAddr;
use lettre::message::Mailbox;

#[derive(Deserialize)]
struct EmailRequest {
    using: String,
    from: String,
    subject: String,
    body: String,
}

#[tokio::main]
async fn main() {
    let port = env::var("PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(3000);

    let app = Router::new().route("/send-email", post(send_email_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn send_email_handler(Json(payload): Json<EmailRequest>) -> impl IntoResponse {
    println!("Sending email '{}' from: {}", payload.subject, payload.using);
    let token_key = format!("{}_TOKEN", payload.using.to_uppercase());
    let user_name_key = format!("{}_SMTP_USERNAME", payload.using.to_uppercase());

    let token = match env::var(&token_key) {
        Ok(v) => v,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                format!("Missing environment variable: {}", token_key),
            )
                .into_response();
        }
    };

    let user_name = match env::var(&user_name_key) {
        Ok(v) => v,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                format!("Missing environment variable: {}", user_name_key),
            )
                .into_response();
        }
    };

    let smtp_server = env::var("SMTP_SERVER").unwrap_or_else(|_| "localhost".to_string());
    let smtp_port = env::var("SMTP_PORT")
        .unwrap_or_else(|_| "587".to_string())
        .parse::<u16>()
        .unwrap_or(587);

    let email_addr: Mailbox = match user_name.parse() {
        Ok(a) => a,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Invalid sender address '{}': {}", user_name, e),
            )
                .into_response();
        }
    };

    let email = match Message::builder()
        .from(email_addr.clone())
        .to(email_addr)
        .subject(payload.subject)
        .body(format!("From: {}\n{}", payload.from, payload.body))
    {
        Ok(m) => m,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                format!("Invalid email data: {}", e),
            )
                .into_response();
        }
    };

    let creds = Credentials::new(user_name, token);

    let mailer: AsyncSmtpTransport<Tokio1Executor> =
        match AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&smtp_server) {
            Ok(m) => m.port(smtp_port).credentials(creds).build(),
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Could not create mailer: {}", e),
                )
                    .into_response();
            }
        };

    let response = match mailer.send(email).await {
        Ok(_) => (StatusCode::OK, "Email sent successfully").into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Could not send email: {}", e),
        )
            .into_response(),
    };

    println!("Email sent successfully!");

    response
}
