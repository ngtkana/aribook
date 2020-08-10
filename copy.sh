#! /usr/bin/sh
set -euo > /dev/null

# 使い方を印刷します。
function print_usage {
  echo -e "Usage:"
  echo -e "\t${0} SOURCE TARGET"
}

# Number of argument = 2
if [ "$#" -ne 2 ]; then
    echo "Illegal number of parameters"
    print_usage
    exit 1
fi

# Arguments
SOURCE=${1}
TARGET=${2}

# BIN_NAME は TARGET と同じです。
BIN_NAME=${TARGET}

cargo new ${TARGET}
cd ${TARGET}
cargo add proconio
cargo add proconcli --path=../proconcli
cd ..

# main.rs を作ります。
cat "${SOURCE}/src/main.rs"         \
    | sed "s/${SOURCE}/${TARGET}/g" \
    > ${TARGET}/src/main.rs

# Cargo.toml を作ります。
cat "${SOURCE}/Cargo.toml"          \
    | sed "s/${SOURCE}/${TARGET}/g" \
    > ${TARGET}/Cargo.toml

