# Exprcess Rust SampleApp
This is a sample application that uses a library I created called [Exprocess](https://github.com/itskihaga/exprocess-rust).
Bug reports, etc. can be submitted via Github Issue.

# Development
## Install Modules
```bash
git submodule update -i
npm i
```

## Build Dev
```
npm run dev
```

## Emulate Firebase
```
npm run emu
```

## Open
localhost:8080

## Build WASM with Docker
```bash
docker-compose run wasm
wasm-pack build --release
```

# Url
https://pick-role.web.app/
