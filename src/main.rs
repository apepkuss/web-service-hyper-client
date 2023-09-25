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

static URL_CHAT_COMPLETIONS: &str = "http://localhost:8080/v1/chat/completions";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();

    let system_prompt = String::from("<<SYS>>\nYou are a helpful, respectful and honest assistant. Always answer as short as possible, while being safe. <</SYS>>\n\n");
    let mut prompt = String::new();

    println!("Enter some text (or press Ctrl + Q to exit):");

    loop {
        println!("[Question]");

        let mut user_message = String::new();
        std::io::stdin()
            .read_line(&mut user_message)
            .ok()
            .expect("Failed to read line");

        if user_message.trim() == "\u{11}" {
            break;
        }

        if user_message.is_empty() || user_message == "\n" || user_message == "\r\n" {
            continue;
        }

        // dbg!(user_message.trim());

        if prompt == "" {
            prompt = format!(
                "<s>[INST] {} {} [/INST]",
                system_prompt,
                user_message.trim()
            );
        } else {
            prompt = format!("{}<s>[INST] {} [/INST]", prompt, user_message.trim());
        }

        // println!("\n*** [prompt begin] ***");
        // println!("{}", &prompt);
        // println!("*** [prompt end] ***\n\n");

        // ChatCompletionRequest
        let request = {
            // uri
            let uri = URL_CHAT_COMPLETIONS.parse::<Uri>()?;
            // data
            let data = create_chat_request(&prompt);
            let data = json!(data);

            // request
            Request::builder()
                .method("POST")
                .uri(uri)
                .header("CONTENT_TYPE", "application/json")
                .body(Body::from(serde_json::to_string(&data)?))?
        };

        let response = client.request(request).await?;

        let body_bytes = hyper::body::to_bytes(response.into_body()).await?;
        let model_answer = String::from_utf8(body_bytes.to_vec()).unwrap();
        let answer = model_answer.trim();
        println!("[answer] {answer}", answer = answer);

        prompt = format!("{} {} </s>", prompt, model_answer.trim());
    }

    Ok(())
}

#[derive(Deserialize, Serialize)]
struct SendRequest {
    name: String,
    active: bool,
}

fn create_chat_request(prompt: &str) -> ChatCompletionRequest {
    let model = "gpt-3.5-turbo";
    // create messages
    let mut messages: Vec<ChatCompletionRequestMessage> = vec![];
    messages.push(ChatCompletionRequestMessage {
        role: ChatCompletionRole::User,
        content: String::from(prompt),
        name: None,
        function_call: None,
    });

    ChatCompletionRequestBuilder::new(model, messages).build()
}

fn _create_completion_request() -> CompletionRequest {
    CompletionRequestBuilder::new("text-davinci-003", vec![String::from("Say this is a test")])
        .build()
}

fn _create_embedding_request() -> EmbeddingsRequest {
    EmbeddingsRequestBuilder::new(
        "text-embedding-ada-002",
        vec![String::from("The food was delicious and the waiter...")],
    )
    .build()
}

fn _create_llama_chat_request() -> ChatCompletionRequest {
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

fn _read_input() -> String {
    loop {
        let mut user_message = String::new();
        std::io::stdin()
            .read_line(&mut user_message)
            .ok()
            .expect("Failed to read line");

        if !user_message.is_empty() && user_message != "\n" && user_message != "\r\n" {
            return user_message;
        }
    }
}
