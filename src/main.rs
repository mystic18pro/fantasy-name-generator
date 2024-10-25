use std::collections::HashMap;
use std::io::{self, Write};

fn get_required_attributes(prompt: &str, example: &str) -> String {
    loop {
        print!("{} (e.g., {}) [Required]: ", prompt, example);
        io::stdout().flush().unwrap();
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();
        let user_input = user_input.trim();
        if !user_input.is_empty() {
            return user_input.to_string();
        }
        println!("This field is required. Please provide a response.");
    }
}

fn get_optional_attributes(prompt: &str, example: &str) -> String {
    print!("{} (e.g., {}) [Optional]: ", prompt, example);
    io::stdout().flush().unwrap();
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    user_input.trim().to_string()
}

fn generate_fantasy_name_prompt() {
    let required_attributes: HashMap<&str, &str> = [
        ("typeOfName", "character, place, artifact, creature"),
        ("genreSetting", "high fantasy, dark fantasy, steampunk, sci-fi"),
        ("gender", "neutral, feminine, masculine, non-binary"),
    ]
    .iter()
    .cloned()
    .collect();

    let mut user_inputs: HashMap<&str, String> = HashMap::new();

    for (key, example) in &required_attributes {
        let user_input = get_required_attributes(&format!("Enter {}", key), example);
        user_inputs.insert(*key, user_input);
    }

    let optional_attributes: HashMap<&str, &str> = [
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
    ]
    .iter()
    .cloned()
    .collect();

    for (key, example) in &optional_attributes {
        let user_input = get_optional_attributes(&format!("Enter {}", key), example);
        if !user_input.is_empty() {
            user_inputs.insert(*key, user_input);
        }
    }

    let prompt_sections: Vec<String> = vec![
        format!("Generate 20 {} names", user_inputs["typeOfName"]),
        format!("for a {} setting", user_inputs["genreSetting"]),
    ]
    .into_iter()
    .chain(optional_attributes.keys().filter_map(|&key| {
        match key {
            "culturalInfluence" => user_inputs.get(key).map(|v| format!("with {} cultural influence", v)),
            "characterTraits" => user_inputs.get(key).map(|v| format!("The name should reflect traits like {}", v)),
            "soundStyle" => user_inputs.get(key).map(|v| format!("sound {}", v)),
            "length" => user_inputs.get(key).map(|v| format!("and be {} in length", v)),
            "alliterationOrRhyme" => user_inputs.get(key).map(|v| format!("with a preference for {}", v)),
            "mythologicalTheme" => user_inputs.get(key).map(|v| format!("and embody themes like {}", v)),
            "alignment" => user_inputs.get(key).map(|v| format!("an alignment of {}", v)),
            "symbolismMeaning" => user_inputs.get(key).map(|v| format!("and a meaning related to {}", v)),
            "historicalEra" => user_inputs.get(key).map(|v| format!("Inspired by the {} era", v)),
            "environment" => user_inputs.get(key).map(|v| format!("reflecting an environment such as {}", v)),
            "magicPowerLevel" => user_inputs.get(key).map(|v| format!("and a magic or power level of {}", v)),
            "titleStatus" => user_inputs.get(key).map(|v| format!("The name may include a title like {}", v)),
            "languageOrigin" => user_inputs.get(key).map(|v| format!("and draw inspiration from {} for linguistic flavor", v)),
            _ => None,
        }
    }))
    .collect();

    let combined_prompt = prompt_sections.join(" ") + ".";
    println!("Prompt:\n{}", combined_prompt);
    
    let response = "1. Axton Kestrel\n2. Corvus Zenith\n3. Cygnus Vance\n4. Darius Forge\n5. Ethan Solaris\n6. Flynn Nova\n7. Jace Orion\n8. Kairos Axiom\n9. Kellan Vector\n10. Leonidas Quantum\n11. Magnus Cipher\n12. Maverick Eclipse\n13. Orion Atlas\n14. Phoenix Sterling\n15. Rhys Zenith\n16. Ryder Eclipse\n17. Silas Nova\n18. Talon Obsidian\n19. Zephyr Vox\n20. Zane Quantum";

    let names: Vec<&str> = response.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.trim())
        .collect();

    for name in names {
        println!("{}", name);
    }
}

fn main() {
    generate_fantasy_name_prompt();
}
