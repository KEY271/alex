default:
    @just --list
run:
    #!/bin/bash
    cd server
    cargo run
test:
    #!/bin/bash
    cd server
    cargo test
