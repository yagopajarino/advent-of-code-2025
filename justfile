new-day number:
    #!/bin/bash
    DIR=src/day_{{number}}

    mkdir -p $DIR
    touch $DIR/mod.rs
    touch $DIR/input_0.txt
    touch $DIR/input_1.txt
    touch $DIR/test_input_0.txt
    touch $DIR/test_input_1.txt
