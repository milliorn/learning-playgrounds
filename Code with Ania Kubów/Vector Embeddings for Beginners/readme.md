# Vector Embeddings for Beginners - 30 min CS Course | OpenAI

## Source

https://youtu.be/PR7xz5vQKGg?list=PLK4fpgXhcrIb-L7qsENA5-KiyH6Sgr-kI

## Create embeddings

https://platform.openai.com/docs/api-reference/embeddings/create

```
curl https://api.openai.com/v1/embeddings \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "input": "The food was delicious and the waiter...",
    "model": "text-embedding-ada-002"
  }'
```

## Database

https://astra.datastax.com/f598f63c-1265-4bb0-8b32-6d3161dc192e

## LangChain

https://langchain.com/
