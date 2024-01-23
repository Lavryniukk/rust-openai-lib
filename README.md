# rust-openai-lib

A brief description of what this project does and who it's for.

## Installation

Install this project with cargo

cargo install rust-openai-lib

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

| Parameter |  Type  |     Description     |
| :-------: | :----: | :-----------------: |
|  api_key  | String | Your OpenAI API key |
|   model   | Model  |  The model to use   |

Returns: Openai

### Get chat completion

```rust
let messages = vec![Message {
	"role": "user",
	"content": "Hello world"
}];
let response = openai.get_chat_completion(messages).await.unwrap();
```

| Parameter |     Type     |               Description                |
| :-------: | :----------: | :--------------------------------------: |
| messages  | Vec<Message> | List of messages for the chat completion |

Returns: Result<Value, Error>
