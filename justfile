default:
    @just --list
dev:
    #!/bin/bash
    cd client
    npm run dev
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
pretty:
    #!/bin/bash
    cd client
    npx prettier . --write
