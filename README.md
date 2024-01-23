# rust-openai-lib

This project is a Rust library for interacting with the OpenAI API. It provides a simple and intuitive interface for sending requests to the API and processing the responses. The library supports various features of the OpenAI API, including chat completions. It's designed for developers who want to integrate OpenAI's powerful AI models into their Rust applications.

## Installation

Install this project with cargo

```bash
cargo install rust-openai-lib
```

## Usage/Examples

```rust
use rust-openai-lib::{Model, Openai};

let openai = Openai::new("your-api-key", Model::Gpt35Turbo);
```

## API Reference

### Initialize OpenAI

```rust
let openai = Openai::new("your-api-key", Model::Gpt35Turbo);
```

| Parameter |   Type   |     Description     |
| :-------: | :------: | :-----------------: |
|  api_key  | `String` | Your OpenAI API key |
|   model   | `Model`  |  The model to use   |

Returns: Openai

### Get chat completion

```rust
let messages = vec![Message {
	"role": "user",
	"content": "Hello world"
}];
let response = openai.get_chat_completion(messages).await.unwrap();
```

| Parameter |      Type      |               Description                |
| :-------: | :------------: | :--------------------------------------: |
| messages  | `Vec<Message>` | List of messages for the chat completion |

Returns: `Result<Value, Error>`

Where `Value` is response object.
