import google.generativeai as genai

genai.configure(api_key='AIzaSyDR22kDc1ZFonm9GltsfkvCHQBDRUm3yzA')
model = genai.GenerativeModel("gemini-1.5-flash")


def getRequiredAttributs(prompt, example):

    while True:
        user_input = input(f"{prompt} (e.g., {example}) [Required]: ")
        if user_input.strip():
            return user_input
        print("This field is required. Please provide a response.")

def getOptionalAttributes(prompt, example):
    """Get an optional attribute with example hint."""
    return input(f"{prompt} (e.g., {example}) [Optional]: ")

def generate_fantasy_name_prompt():
    requiredAttributes = {
        "typeOfName": "character, place, artifact, creature",
        "genreSetting": "high fantasy, dark fantasy, steampunk, sci-fi",
        "gender": "neutral, feminine, masculine, non-binary"
    }
    userInputs = {key: getRequiredAttributs(f"Enter {key}", example) for key, example in requiredAttributes.items()}

    optionalAttributes = {
        "culturalInfluence": "Celtic, Norse, Japanese, Aztec, otherworldly",
        "characterTraits": "brave, mysterious, wise, fierce, ethereal",
        "soundStyle": "harsh, melodic, guttural, soft, lyrical",
        "length": "short, medium, long, single syllable, multi-syllable",
        "alliterationOrRhyme": "alliterative, rhyming, no preference",
        "mythologicalTheme": "fire, water, earth, celestial, forest",
        "alignment": "good, evil, neutral, chaotic, lawful",
        "symbolismMeaning": "shadow, dawn, strength, wisdom, fate",
        "historicalEra": "medieval, ancient, futuristic, victorian",
        "environment": "mountain, forest, sea, desert, urban",
        "magicPowerLevel": "common, mystical, legendary, ancient",
        "titleStatus": "lord, queen, protector, sage, wanderer",
        "languageOrigin": "Old English, Latin, Elvish, invented language",
    }
    userInputs.update({key: getOptionalAttributes(f"Enter {key}", example) for key, example in optionalAttributes.items()})
    
    promptSections = [
        f"Generate 20 {userInputs['typeOfName']} names",
        f"for a {userInputs['genreSetting']} setting",
        f"with {userInputs['culturalInfluence']} cultural influence" if userInputs.get("culturalInfluence") else "",
        f"The name should reflect traits like {userInputs['characterTraits']}" if userInputs.get("characterTraits") else "",
        f"sound {userInputs['soundStyle']}" if userInputs.get("soundStyle") else "",
        f"and be {userInputs['length']} in length" if userInputs.get("length") else "",
        f"It should be {userInputs['gender']}-associated",
        f"with a preference for {userInputs['alliterationOrRhyme']}" if userInputs.get("alliterationOrRhyme") else "",
        f"and embody themes like {userInputs['mythologicalTheme']}" if userInputs.get("mythologicalTheme") else "",
        f"an alignment of {userInputs['alignment']}" if userInputs.get("alignment") else "",
        f"and a meaning related to {userInputs['symbolismMeaning']}" if userInputs.get("symbolismMeaning") else "",
        f"Inspired by the {userInputs['historicalEra']} era" if userInputs.get("historicalEra") else "",
        f"reflecting an environment such as {userInputs['environment']}" if userInputs.get("environment") else "",
        f"and a magic or power level of {userInputs['magicPowerLevel']}" if userInputs.get("magicPowerLevel") else "",
        f"The name may include a title like {userInputs['titleStatus']}" if userInputs.get("titleStatus") else "",
        f"and draw inspiration from {userInputs['languageOrigin']} for linguistic flavor" if userInputs.get("languageOrigin") else "",
        "."
    ]
    
    combinedPrompt = " ".join(section for section in promptSections if section)
    print(f"Prompt:\n{combinedPrompt}")
    response = model.generate_content(combinedPrompt)
    
    names = response.text.splitlines()
    names = [name.strip().replace('**', '') for name in names if name.strip() and not name.lower().startswith('##')]

    for name in names:
        print(f"{name}")

generate_fantasy_name_prompt()
