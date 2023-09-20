use hyper::{Body, Client, Request, Uri};
use serde::{Deserialize, Serialize};
use serde_json::json;

use xin::{
    chat::{
        ChatCompletionRequest, ChatCompletionRequestBuilder, ChatCompletionRequestMessage,
        ChatCompletionRole,
    },
    completions::{CompletionRequest, CompletionRequestBuilder},
    embeddings::{EmbeddingsRequest, EmbeddingsRequestBuilder},
};

// static URL_ECHO: &str = "http://localhost:8080/echo";
static URL_ECHO: &str = "http://52.40.2.252:3000/echo";
// static URL_CHAT_COMPLETIONS: &str = "http://localhost:8080/v1/chat/completions";
static URL_CHAT_COMPLETIONS: &str = "http://52.40.2.252:3000/v1/chat/completions";
// static URL_OPENAI_COMPLETIONS: &str = "http://localhost:8080/openai/v1/completions";
static URL_OPENAI_COMPLETIONS: &str = "http://34.217.109.23:3000/openai/v1/completions";
// static URL_OPENAI_EMBEDDINGS: &str = "http://localhost:8080/openai/v1/embeddings";
static URL_OPENAI_EMBEDDINGS: &str = "http://34.217.109.23:3000/openai/v1/embeddings";
// static URL_OPENAI_MODELS: &str = "http://34.217.109.23:3000/openai/v1/models";
static URL_MODELS: &str = "http://localhost:8080/v1/models";
// static URL_LLAMA_CHAT_COMPLETIONS: &str = "http://localhost:8080/llama/v1/chat/completions";
static URL_LLAMA_CHAT_COMPLETIONS: &str = "http://52.40.2.252:3000/llama/v1/chat/completions";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();

    // echo test
    let _request = {
        // uri
        let uri = URL_ECHO.parse::<Uri>()?;

        // request
        Request::builder()
            .method("POST")
            .uri(uri)
            .header("CONTENT_TYPE", "application/json")
            .body(Body::empty())?
    };

    // ChatCompletionRequest
    let request = {
        // uri
        let uri = URL_CHAT_COMPLETIONS.parse::<Uri>()?;
        // data
        let data = create_chat_request();
        let data = json!(data);

        // request
        Request::builder()
            .method("POST")
            .uri(uri)
            .header("CONTENT_TYPE", "application/json")
            .body(Body::from(serde_json::to_string(&data)?))?
    };

    // CreateCompletionRequest
    let _request = {
        // uri
        let uri = URL_OPENAI_COMPLETIONS.parse::<Uri>()?;
        // data
        let data = create_completion_request();
        let data = json!(data);
        // request
        Request::builder()
            .method("POST")
            .uri(uri)
            .header("CONTENT_TYPE", "application/json")
            .body(Body::from(serde_json::to_string(&data)?))?
    };

    // CreateEmbeddingsRequest
    let _request = {
        // uri
        let uri = URL_OPENAI_EMBEDDINGS.parse::<Uri>()?;
        // data
        let data = create_embedding_request();
        let data = json!(data);
        // request
        Request::builder()
            .method("POST")
            .uri(uri)
            .header("CONTENT_TYPE", "application/json")
            .body(Body::from(serde_json::to_string(&data)?))?
    };

    // models
    let _request = {
        // uri
        let uri = URL_MODELS.parse::<Uri>()?;
        // request
        Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty())?
    };

    // LlamaChatCompletionRequest
    let _request = {
        // uri
        let uri = URL_LLAMA_CHAT_COMPLETIONS.parse::<Uri>()?;
        // data
        let data = create_llama_chat_request();
        let data = json!(data);

        // request
        Request::builder()
            .method("POST")
            .uri(uri)
            .header("CONTENT_TYPE", "application/json")
            .body(Body::from(serde_json::to_string(&data)?))?
    };

    let response = client.request(request).await?;
    let status = response.status();
    println!("Status: {}", status);

    let body_bytes = hyper::body::to_bytes(response.into_body()).await?;
    // let data: xin::chat::ChatCompletionRequest = serde_json::from_slice(&body_bytes).unwrap();
    // let model_answer = data.messages[0].content.as_str();
    let model_answer = String::from_utf8(body_bytes.to_vec()).unwrap();
    println!("model_answer: {:?}", model_answer);

    Ok(())
}

#[derive(Deserialize, Serialize)]
struct SendRequest {
    name: String,
    active: bool,
}

fn create_chat_request() -> ChatCompletionRequest {
    let model = "gpt-3.5-turbo";
    // create messages
    let mut messages: Vec<ChatCompletionRequestMessage> = vec![];
    messages.push(ChatCompletionRequestMessage {
        role: ChatCompletionRole::User,
        content: String::from("What is Bitcoin?"),
        name: None,
        function_call: None,
    });

    ChatCompletionRequestBuilder::new(model, messages).build()
}

fn create_completion_request() -> CompletionRequest {
    CompletionRequestBuilder::new("text-davinci-003", vec![String::from("Say this is a test")])
        .build()
}

fn create_embedding_request() -> EmbeddingsRequest {
    EmbeddingsRequestBuilder::new(
        "text-embedding-ada-002",
        vec![String::from("The food was delicious and the waiter...")],
    )
    .build()
}

fn create_llama_chat_request() -> ChatCompletionRequest {
    let model = "";

    // create messages
    let mut messages: Vec<ChatCompletionRequestMessage> = vec![];
    messages.push(ChatCompletionRequestMessage {
        role: ChatCompletionRole::User,
        content: String::from("What is Bitcoin?"),
        name: None,
        function_call: None,
    });

    ChatCompletionRequestBuilder::new(model, messages).build()
}
