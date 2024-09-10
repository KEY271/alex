default:
    @just --list
dev:
    #!/bin/bash
    cd client
    npm run dev
server:
    #!/bin/bash
    cargo watch -x "run --bin alex-server -- --server-only"
cli:
    #!/bin/bash
    cargo watch -x "run --bin alex-cli"
build:
    #!/bin/bash
    cd client
    npm run build
    cd ..
    mkdir ./static
    cp -r ./client/dist/* ./static
run:
    #!/bin/bash
    cargo run --bin alex-server --release
fmt:
    #!/bin/bash
    cd client
    npx prettier . --write
