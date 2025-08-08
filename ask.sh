#!/bin/bash
model="gpt-4o"
prompt="Pls also write full integration for query-ng data"

repomix --ignore "ask.sh, README.md"

echo ''
echo "PLEASE WAIT FOR LIKE SOME TIME, THIS IS NOT A STREAMING SERVICE!!!"
echo ''

if [[ ! -f "repomix-output.xml" ]]; then
    echo "File repomix-output.xml not found!"
    exit 1
fi
# Read file
content=$(<repomix-output.xml)

# Check for internet connectivity
if ping -c 1 google.com &> /dev/null; then
    # Construct JSON payload
    json_payload=$(jq -n --arg prompt "$prompt" --arg content "$content" --arg model "$model" \
        '{model: $model, temperature: 0.2, messages: [{role: "system", content: $prompt}, {role: "user", content: $content}]}')

    # Send the prompt and content to the OpenAI API
    response=$(curl -s -X POST -H "Content-Type: application/json" \
        -d "$json_payload" \
        -H "Authorization: Bearer $TOKEN_GPT" https://api.openai.com/v1/chat/completions)

    # Extract and print the response content
    echo "$response" | jq -r '.choices[0].message.content'
else
    echo "No internet :("
fi

echo ''
