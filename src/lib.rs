use reqwest::{Error, Response};
use serde_json::Value;

/// Represents the available models for the OpenAI instance.
pub enum Model {
    Gpt35Turbo,
    Gpt35Turbo16k,
    Gpt35TurboInstruct,
    Gpt35Turbo1106,
    Gpt41106Preview,
    Gpt4,
    Gpt432k,
    Gpt4Instruct,
    Gpt432k0613,
}

#[derive(serde::Serialize)]
/// Represents a message with a role and content.
pub struct Message {
    /// The role of the message.
    pub role: String,
    /// The content of the message.
    pub content: String,
}

/// Represents an OpenAI instance with a model and an API key.
pub struct Openai {
    /// The model used by the OpenAI instance.
    model: Model,
    /// The API key used for authentication.
    api_key: String,
}

/// Implementation of the `Openai` struct.
impl Openai {
    /// Creates a new instance of `Openai`.
    ///
    /// # Arguments
    ///
    /// * `api_key` - The API key for accessing the OpenAI API.
    /// * `model` - The model to be used for chat completions.
    ///
    /// # Returns
    ///
    /// A new instance of `Openai`.
    ///
    /// # Example
    ///
    /// ```
    /// use rust-openai-lib::{ Model, Openai};
    ///
    /// #[tokio::main]
    /// fn main() {
    ///
    /// let api_key = String::new("your_api_key");
    ///
    /// let openai = Openai::new(api_key, Model::Gpt35Turbo);
    /// }
    /// ```
    pub fn new(api_key: String, model: Model) -> Self {
        Openai { model, api_key }
    }

    /// Sends a request to the OpenAI API to get chat completions.
    ///
    /// # Arguments
    ///
    /// * `messages` - The list of messages for the chat completion.
    ///
    /// # Returns
    ///
    /// A `Result` containing the json'ed response from the API or an error.
    ///
    /// # Example
    ///
    /// ```
    /// use rust-openai-lib::{Message, Model, Openai};
    ///
    /// #[tokio::main]
    /// fn main() {
    ///
    /// let api_key = String::new("your_api_key");
    ///
    /// let openai = Openai::new(api_key, Model::Gpt35Turbo);
    ///
    /// let messages = vec![Message {
    ///     role: "user".to_string(),
    ///     content: "Hello, I'm a user!".to_string(),
    /// }]
    ///
    /// let response = openai.get_chat_completion(messages).await.unwrap();
    /// }
    /// ```
    pub async fn get_chat_completion(&self, messages: Vec<Message>) -> Result<Value, Error> {
        let client = reqwest::Client::new();

        #[derive(serde::Serialize)]
        struct ChatCompletionBody {
            model: String,
            messages: Vec<Message>,
        }

        let body = ChatCompletionBody {
            model: self.model.format(),
            messages,
        };

        let url = "https://api.openai.com/v1/chat/completions";

        let response = client
            .post(url)
            .json(&body)
            .header("Content-Type", "application/json")
            .bearer_auth(&self.api_key)
            .send()
            .await;

        match response {
            Ok(raw_response) => {
                let parsed_response = raw_response.json::<serde_json::Value>().await;
                Ok(parsed_response?)
            }
            Err(err) => Err(err),
        }
    }
}

impl Model {
    /// Formats the model enum variant into the corresponding string representation.
    ///
    /// # Returns
    ///
    /// The formatted string representation of the model.
    fn format(&self) -> String {
        return String::from(match self {
            Model::Gpt35Turbo => "gpt-3.5-turbo",
            Model::Gpt35Turbo16k => "gpt-3.5-turbo-16k",
            Model::Gpt35TurboInstruct => "gpt-3.5-turbo-instruct",
            Model::Gpt35Turbo1106 => "gpt-3.5-turbo-1106",
            Model::Gpt41106Preview => "gpt-4-1106-preview",
            Model::Gpt4 => "gpt-4",
            Model::Gpt432k => "gpt-4-32k",
            Model::Gpt4Instruct => "gpt-4-instruct",
            Model::Gpt432k0613 => "gpt-4-32k-0613",
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::{Model, Openai};

    #[test]
    fn test_model_format() {
        assert_eq!(Model::Gpt35Turbo.format(), "gpt-3.5-turbo");
        assert_eq!(Model::Gpt35Turbo16k.format(), "gpt-3.5-turbo-16k");
        assert_eq!(Model::Gpt35TurboInstruct.format(), "gpt-3.5-turbo-instruct");
        assert_eq!(Model::Gpt35Turbo1106.format(), "gpt-3.5-turbo-1106");
        assert_eq!(Model::Gpt41106Preview.format(), "gpt-4-1106-preview");
        assert_eq!(Model::Gpt4.format(), "gpt-4");
        assert_eq!(Model::Gpt432k.format(), "gpt-4-32k");
        assert_eq!(Model::Gpt4Instruct.format(), "gpt-4-instruct");
        assert_eq!(Model::Gpt432k0613.format(), "gpt-4-32k-0613");
    }

    #[test]
    fn test_openai_new() {
        let openai = Openai::new("test_api_key".to_string(), Model::Gpt4);
        assert_eq!(openai.api_key, "test_api_key");
        assert_eq!(openai.model.format(), "gpt-4");
    }
}
