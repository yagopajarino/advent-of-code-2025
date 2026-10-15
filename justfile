new-day number:
    #!/bin/bash
    PADDED_NUMBER=$(printf "%02d" "{{number}}")
    DIR=src/day${PADDED_NUMBER}

    mkdir -p $DIR
    touch $DIR/mod.rs
    touch $DIR/input_0.txt
    touch $DIR/input_1.txt
    touch $DIR/test_input_0.txt

run day part:
    cargo run -- -d {{day}} -p {{part}}
