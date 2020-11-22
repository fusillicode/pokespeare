# pokespeare

## Local build & run
```
RUST_LOG=info \
POKESPEARE_LISTEN_ADDR=localhost:8080 \
POKE_API_ENDPOINT=https://pokeapi.co \
FUN_TRANSLATIONS_API_ENDPOINT=https://api.funtranslations.com \
cargo run
```

## Docker build & run
```
docker build -t pokespeare && \
  docker run pokespeare \
    --env RUST_LOG=info \
    --env POKESPEARE_LISTEN_ADDR=localhost:8080 \
    --env POKE_API_ENDPOINT=https://pokeapi.co \
    --env FUN_TRANSLATIONS_API_ENDPOINT=https://api.funtranslations.com
```

## Call the service
`curl -v localhost:8080/pokemon/bulbasaur`

## Call the service & pretty print its output (requires [jq](https://stedolan.github.io/jq/download/))
`curl -v localhost:8080/pokemon/bulbasaur | jq`

## Test run
```
cargo test
```
