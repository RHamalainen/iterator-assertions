[private]
default:
    @just --list --unsorted

run-examples:
    -cargo run --example=array1
    -cargo run --example=vector1
    -cargo run --example=vector2
