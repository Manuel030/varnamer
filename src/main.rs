use clap::Parser;
use confy::{self};
use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Cli {
    code: Option<String>,

    #[arg(short, long)]
    configure: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Config {
    openai_api_key: String,
}

impl std::default::Default for Config {
    fn default() -> Self { Self { openai_api_key: String::from("") } }
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiRequest {
    pub model: String,
    pub prompt: String,
    pub max_tokens: i64,
    pub temperature: f64,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Choice {
    pub text: String,
    pub index: i64,
    pub logprobs: Value,
    pub finish_reason: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: i64,
    pub completion_tokens: i64,
    pub total_tokens: i64,
}

fn main() -> Result<(), std::io::Error> {
    let cli = Cli::parse();

    if let Some(api_key) = cli.configure {
        let new_cfg = Config { openai_api_key: api_key};
        let result = confy::store("varnamer", None, new_cfg);
        
        if let Ok(_) = result {
            println!("configuration was successful");
            std::process::exit(0)
        } else {
            panic!("errored with error {:?}", result)
        }
    }

    let cfg: Config  = confy::load("varnamer", None).expect("failed to read config file");
    if cfg.openai_api_key.is_empty() {
        panic!("configure your OpenAI API key first")
    }

    if let Some(code) = cli.code {
        let res = request_openai(&code, &cfg.openai_api_key);
        match res {
            Ok(varname) => println!("{:?}", varname),
            Err(err) => panic!("errored with error {:?}", err)
        }
    }
    Ok(())
}

fn request_openai(code: &str, secret: &str) -> Result<String, ureq::Error> {
    let prompt = format!("find a good variable name for the following line of code: {}", code);
    let response = ureq::post("https://api.openai.com/v1/completions")
    .set("Content-Type", "application/json")
    .set("Authorization", &format!("Bearer {}", secret)[..])
    .send_json(ureq::json!(
        OpenAiRequest {
            model: String::from("text-davinci-003"),
            prompt: String::from(prompt),
            max_tokens: 5,
            temperature: 0.8,
        }
    ))?;
    let openai_response: OpenAiResponse = response.into_json()?;
    let varname = openai_response.choices.get(0).unwrap().text.to_owned();
    Ok(varname.replace("\n", ""))
}