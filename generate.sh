#! /usr/bin/sh
set -euo > /dev/null

# 使い方を印刷します。
function print_usage {
  echo -e "Usage:"
  echo -e "\t${0} CRATE_NAME"
}

# Number of argument = 1
if [ "$#" -ne 1 ]; then
    echo "Illegal number of parameters"
    print_usage
    exit 1
fi

# Arguments
CRATE_NAME=${1}

# BIN_NAME は CRATE_NAME と同じです。
BIN_NAME=${CRATE_NAME}

cargo new ${1}
cd ${1}
cargo add proconio
cargo add proconcli --path=../proconcli
cd ..

# main.rs を作ります。
cat src/template.rs                 \
    | sed "s/@BIN/${BIN_NAME}/g"    \
    > ${CRATE_NAME}/src/main.rs

