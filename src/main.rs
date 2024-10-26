use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{self, Write};
use dotenv::dotenv;
use std::env;
#[derive(Serialize)]
struct TextPart {
    text: String,
}

#[derive(Serialize)]
struct Content {
    parts: Vec<TextPart>,
}

#[derive(Serialize)]
struct GenerateRequest {
    contents: Vec<Content>,
}

#[derive(Debug, Deserialize)]
struct TextPartResponse {
    text: String,
}

#[derive(Debug, Deserialize)]
struct ContentResponse {
    parts: Vec<TextPartResponse>,
}

#[derive(Debug, Deserialize)]
struct Candidate {
    content: ContentResponse,
}

#[derive(Debug, Deserialize)]
struct PromptFeedback {
    #[serde(default)]
    block_reason: String,
}

#[derive(Debug, Deserialize)]
struct GenerateResponse {
    candidates: Vec<Candidate>,
    #[serde(default)]
    prompt_feedback: Option<PromptFeedback>,
}

async fn get_required_attribute(prompt: &str, example: &str) -> String {
    loop {
        print!("{} (e.g., {}) [Required]: ", prompt, example);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if !input.is_empty() {
            return input.to_string();
        }
        println!("This field is required. Please provide a response.");
    }
}

async fn get_optional_attribute(prompt: &str, example: &str) -> String {
    print!("{} (e.g., {}) [Optional]: ", prompt, example);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

async fn generate_fantasy_name_prompt(api_key: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    let required_attributes = vec![
        ("typeOfName", "character, place, artifact, creature"),
        ("genreSetting", "high fantasy, dark fantasy, steampunk, sci-fi"),
        ("gender", "neutral, feminine, masculine, non-binary"),
    ];

    let optional_attributes = vec![
        ("culturalInfluence", "Celtic, Norse, Japanese, Aztec, otherworldly"),
        ("characterTraits", "brave, mysterious, wise, fierce, ethereal"),
        ("soundStyle", "harsh, melodic, guttural, soft, lyrical"),
        ("length", "short, medium, long, single syllable, multi-syllable"),
        ("alliterationOrRhyme", "alliterative, rhyming, no preference"),
        ("mythologicalTheme", "fire, water, earth, celestial, forest"),
        ("alignment", "good, evil, neutral, chaotic, lawful"),
        ("symbolismMeaning", "shadow, dawn, strength, wisdom, fate"),
        ("historicalEra", "medieval, ancient, futuristic, victorian"),
        ("environment", "mountain, forest, sea, desert, urban"),
        ("magicPowerLevel", "common, mystical, legendary, ancient"),
        ("titleStatus", "lord, queen, protector, sage, wanderer"),
        ("languageOrigin", "Old English, Latin, Elvish, invented language"),
    ];

    let mut user_inputs = HashMap::new();

    for (key, example) in required_attributes {
        user_inputs.insert(key.to_string(), get_required_attribute(&format!("Enter {}", key), example).await);
    }

    for (key, example) in optional_attributes {
        let input = get_optional_attribute(&format!("Enter {}", key), example).await;
        if !input.is_empty() {
            user_inputs.insert(key.to_string(), input);
        }
    }

    let mut prompt_parts = Vec::new();

    prompt_parts.push(format!("Generate 20 {} names", user_inputs["typeOfName"]));
    prompt_parts.push(format!("for a {} setting", user_inputs["genreSetting"]));
    prompt_parts.push(format!("It should be {}-associated", user_inputs["gender"]));

    if let Some(v) = user_inputs.get("culturalInfluence") {
        prompt_parts.push(format!("with {} cultural influence", v));
    }
    if let Some(v) = user_inputs.get("characterTraits") {
        prompt_parts.push(format!("The name should reflect traits like {}", v));
    }
    if let Some(v) = user_inputs.get("soundStyle") {
        prompt_parts.push(format!("sound {}", v));
    }
    if let Some(v) = user_inputs.get("length") {
        prompt_parts.push(format!("and be {} in length", v));
    }
    if let Some(v) = user_inputs.get("alliterationOrRhyme") {
        prompt_parts.push(format!("with a preference for {}", v));
    }
    if let Some(v) = user_inputs.get("mythologicalTheme") {
        prompt_parts.push(format!("and embody themes like {}", v));
    }
    if let Some(v) = user_inputs.get("alignment") {
        prompt_parts.push(format!("an alignment of {}", v));
    }
    if let Some(v) = user_inputs.get("symbolismMeaning") {
        prompt_parts.push(format!("and a meaning related to {}", v));
    }
    if let Some(v) = user_inputs.get("historicalEra") {
        prompt_parts.push(format!("Inspired by the {} era", v));
    }
    if let Some(v) = user_inputs.get("environment") {
        prompt_parts.push(format!("reflecting an environment such as {}", v));
    }
    if let Some(v) = user_inputs.get("magicPowerLevel") {
        prompt_parts.push(format!("and a magic or power level of {}", v));
    }
    if let Some(v) = user_inputs.get("titleStatus") {
        prompt_parts.push(format!("The name may include a title like {}", v));
    }
    if let Some(v) = user_inputs.get("languageOrigin") {
        prompt_parts.push(format!("and draw inspiration from {} for linguistic flavor", v));
    }

    prompt_parts.push("Format the output as a numbered list.".to_string());

    let combined_prompt = prompt_parts.join(" ");
    println!("\nGenerating names with prompt:\n{}\n", combined_prompt);

    let request = GenerateRequest {
        contents: vec![Content {
            parts: vec![TextPart {
                text: combined_prompt,
            }],
        }],
    };

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}",
        api_key
    );

    let response = client
        .post(url)
        .json(&request)
        .send()
        .await?
        .error_for_status()?
        .json::<GenerateResponse>()
        .await?;

    if let Some(feedback) = response.prompt_feedback {
        if !feedback.block_reason.is_empty() {
            println!("Warning: Prompt was blocked: {}", feedback.block_reason);
            return Ok(());
        }
    }

   
    if let Some(candidate) = response.candidates.first() {
        if let Some(part) = candidate.content.parts.first() {
            println!("Generated names:\n");
          
            for line in part.text.lines() {
                let name = line.trim();
                if !name.is_empty() && !name.starts_with('#') {
                   
                    let clean_name = name.replace("**", "");
                    println!("{}", clean_name);
                }
            }
        }
    } else {
        println!("No names were generated. Please try again with different parameters.");
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file at the start of the program
    dotenv().ok();
    
    // Get API key from environment
    let api_key = env::var("GEMINI_API_KEY")
        .expect("GEMINI_API_KEY must be set in environment");
    
    generate_fantasy_name_prompt(&api_key).await?;
    Ok(())
}