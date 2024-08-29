default:
    @just --list
dev:
    #!/bin/bash
    cd client
    npm run dev
server:
    #!/bin/bash
    cd server
    cargo watch -x "run -- --server-only"
build:
    #!/bin/bash
    cd client
    npm run build
    cd ..
    mkdir ./server/static
    cp -r ./client/dist/* ./server/static
run:
    #!/bin/bash
    cd server
    cargo run
test:
    #!/bin/bash
    cd server
    cargo test
fmt:
    #!/bin/bash
    cd server
    cargo fmt
    cd ../client
    npx prettier . --write
