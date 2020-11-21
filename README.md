# pokespeare

## Local build & run
```
RUST_LOG=info \
POKESPEARE_LISTEN_ADDR=localhost:8080 \
POKE_API_ENDPOINT=https://pokeapi.co/api/v2 \
FUN_TRANSLATIONS_API_ENDPOINT=https://api.funtranslations.com/translate \
cargo run
```

## Docker build & run
```
docker build -t pokespeare && \
  docker run pokespeare \
    --env RUST_LOG=info \
    --env POKESPEARE_LISTEN_ADDR=localhost:8080 \
    --env POKE_API_ENDPOINT=https://pokeapi.co/api/v2 \
    --env FUN_TRANSLATIONS_API_ENDPOINT=https://api.funtranslations.com/translate
```

## Call the service
`curl -v localhost:8080/pokemon/bulbasaur`

## Call the service & pretty print its output (requires [jq](https://stedolan.github.io/jq/download/))
`curl -v localhost:8080/pokemon/bulbasaur | jq`

## Test run
```
cargo test
```
