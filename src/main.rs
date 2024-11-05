use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct PromptRequest {
    pub prompt: String,
}

#[derive(Serialize, Deserialize)] // Added Deserialize trait
pub struct CompletionResponse {
    pub completion: String,
}

// New GET endpoint
#[get("/")]
pub async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hi") // Respond with "Hi"
}

#[post("/completion")]
pub async fn completion_endpoint(req: web::Json<PromptRequest>) -> impl Responder {
    let response = CompletionResponse {
        completion: format!("Received prompt: {}", req.prompt),
    };

    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(completion_endpoint)
            .service(greet)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::StatusCode, test, App};

    #[actix_rt::test]
    async fn test_completion_endpoint() {
        let app = test::init_service(App::new().service(completion_endpoint)).await;

        let req = test::TestRequest::post()
            .uri("/completion")
            .set_json(&PromptRequest {
                prompt: "Hello, world!".into(),
            })
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);

        let result: CompletionResponse = test::read_body_json(resp).await;
        assert_eq!(result.completion, "Received prompt: Hello, world!");
    }
}

