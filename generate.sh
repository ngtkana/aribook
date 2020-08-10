#! /usr/bin/sh
set -euo > /dev/null

# 標準エラー出力に出力です。
function debug_echo {
    echo ${@} >&2
}

# 使い方を印刷します。
function print_usage {
  echo -e "Usage:"
  echo -e "\t${0} CHAP_NAME PROBLEM_NAME"
}

# Number of argument = 2
if [ "$#" -ne 2 ]; then
    echo "Illegal number of parameters"
    print_usage
    exit 1
fi

# Arguments
CHAP_NAME=${1}
PROBLEM_NAME=${2}

ORIG_DIR=$(pwd)
SCRIPT_DIR=$(dirname ${0})
CHAP_DIR="${SCRIPT_DIR}/crates/${CHAP_NAME}"
PROBLEM_DIR="${CHAP_DIR}/${PROBLEM_NAME}"
SOURCE_DIR="${SCRIPT_DIR}/src"

debug_echo "ORIG_DIR = ${ORIG_DIR}"
debug_echo "SCRIPT_DIR = ${SCRIPT_DIR}"

# crate を作ります。
# cd して、また ORIG_DIR に帰ってきます。
debug_echo "Making directory ${CHAP_DIR}..."
mkdir -p ${CHAP_DIR}

debug_echo "Changing the cd to ${CHAP_DIR}..."
cd ${CHAP_DIR}

debug_echo "Initializing ${PROBLEM_NAME}"
cargo new ${PROBLEM_NAME}
cd ${PROBLEM_NAME}

debug_echo "Returning to ${ORIG_DIR}..."
cd ${ORIG_DIR}

# main.rs, Cargo.toml を作ります。
cat "${SOURCE_DIR}/template.rs"             \
    | sed "s/@CHAP/${CHAP_NAME}/g"          \
    | sed "s/@PROBLEM/${PROBLEM_NAME}/g"    \
    > ${PROBLEM_DIR}/src/main.rs

cat "${SOURCE_DIR}/Cargo.toml"              \
    | sed "s/@CHAP/${CHAP_NAME}/g"          \
    | sed "s/@PROBLEM/${PROBLEM_NAME}/g"    \
    > ${PROBLEM_DIR}/Cargo.toml
