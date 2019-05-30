#!/bin/sh

if [[ $# -ne 2 ]]; then
    echo "please specify source file and input."
    exit 1
fi

source_path="$(readlink -f $1)"
input_path="$(readlink -f $2)"

sudo docker run --volume "$source_path:/submission/main.rs" --volume "$input_path:/submission/in.txt" statiolake/atcoder_sample
