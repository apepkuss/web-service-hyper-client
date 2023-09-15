use hyper::{Body, Client, Request, Uri};
use serde::{Deserialize, Serialize};
use serde_json::json;

use xin::{
    chat::{
        ChatCompletionRequest, ChatCompletionRequestBuilder, ChatCompletionRequestMessage,
        ChatCompletionRequestSampling, ChatCompletionRole,
    },
    completions::{CompletionRequest, CompletionRequestBuilder},
    embeddings::{EmbeddingsRequest, EmbeddingsRequestBuilder},
};

// static URL_SEND: &str = "http://localhost:8080/send";
static URL_CHAT_COMPLETIONS: &str = "http://localhost:8080/v1/chat/completions";
static URL_COMPLETIONS: &str = "http://localhost:8080/v1/completions";
static URL_EMBEDDINGS: &str = "http://localhost:8080/v1/embeddings";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();

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
        let uri = URL_COMPLETIONS.parse::<Uri>()?;
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
        let uri = URL_EMBEDDINGS.parse::<Uri>()?;
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

    let response = client.request(request).await?;
    let status = response.status();
    let body = hyper::body::to_bytes(response.into_body()).await?;

    println!("Status: {}", status);
    println!("Body: {:?}", body);

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
