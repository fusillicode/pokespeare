# pokespeare

[![CI](https://github.com/fusillicode/pokespeare/workflows/Rust/badge.svg)](https://github.com/fusillicode/pokespeare/actions)

REST Web Service exposing a single API that, given a Pokémon name, returns its "Shakespearean" description.

Under the hood it uses [PokéAPI](https://pokeapi.co/) and [Shakespeare translator](https://funtranslations.com/api/shakespeare).

## Local build & run
Install [rustup](https://www.rust-lang.org/tools/install) and then run:
```sh
RUST_LOG=info \
POKESPEARE_LISTEN_ADDR=0.0.0.0:8080 \
POKE_API_ENDPOINT=https://pokeapi.co \
FUN_TRANSLATIONS_API_ENDPOINT=https://api.funtranslations.com \
cargo run
```

## Docker build & run
```sh
docker build . -t pokespeare && \
docker run \
  --env RUST_LOG=info \
  --env POKESPEARE_LISTEN_ADDR=0.0.0.0:8080 \
  --env POKE_API_ENDPOINT=https://pokeapi.co \
  --env FUN_TRANSLATIONS_API_ENDPOINT=https://api.funtranslations.com \
  -p 8080:8080
  pokespeare
```

## Call the service
```sh
curl -v 0.0.0.0:8080/pokemon/bulbasaur`
```

## Call the service & pretty print its output (requires [jq](https://stedolan.github.io/jq/download/))
```sh
curl -v 0.0.0.0:8080/pokemon/bulbasaur | jq
```

## Test run
```sh
cargo test
```
