use hyper::{Body, Client, Request, Uri};
use serde::{Deserialize, Serialize};
use serde_json::json;

mod chat;
use chat::{
    ChatCompletionRequestMessage, ChatCompletionRequestRole, CreateChatCompletionRequest,
    CreateChatCompletionRequestBuilder, CreateChatCompletionsRequestSampling,
};

static URL_SEND: &str = "http://localhost:8080/send";
static URL_CHAT_COMPLETIONS: &str = "http://localhost:8080/v1/chat/completions";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();

    // uri
    let uri = URL_CHAT_COMPLETIONS.parse::<Uri>()?;

    // create a request body
    // let data = SendRequest {
    //     name: "chip".to_string(),
    //     active: true,
    // };
    let data = create_chat_completions_request();
    let data = json!(data);

    let request = Request::builder()
        .method("POST")
        .uri(uri)
        .header("CONTENT_TYPE", "application/json")
        .body(Body::from(serde_json::to_string(&data)?))?;

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

fn create_chat_completions_request() -> CreateChatCompletionRequest {
    let model = "gpt-3.5-turbo";
    // create messages
    let mut messages: Vec<ChatCompletionRequestMessage> = vec![];
    messages.push(ChatCompletionRequestMessage {
        role: ChatCompletionRequestRole::System,
        content: String::from("You are a helpfule assistant."),
        name: None,
        function_call: None,
    });
    messages.push(ChatCompletionRequestMessage {
        role: ChatCompletionRequestRole::User,
        content: String::from("Hello!"),
        name: None,
        function_call: None,
    });
    let sampling = CreateChatCompletionsRequestSampling::Temperature(0.8);

    CreateChatCompletionRequestBuilder::new(model, messages, sampling).build()
}
