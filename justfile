default:
    @just --list
dev:
    #!/bin/bash
    cd client
    npm run dev
server:
    #!/bin/bash
    cd alex
    cargo watch -x "run -- --server-only"
build:
    #!/bin/bash
    cd client
    npm run build
    cd ..
    mkdir ./alex/static
    cp -r ./client/dist/* ./alex/static
run:
    #!/bin/bash
    cd alex
    cargo run --release
test:
    #!/bin/bash
    cd alex
    cargo test
fmt:
    #!/bin/bash
    cd alex
    cargo fmt
    cd ../client
    npx prettier . --write
