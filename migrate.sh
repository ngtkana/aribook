#! /usr/bin/bash
# @(#) Migration
set -euo > /dev/null

# 標準エラー出力に出力です。
function debug_echo {
    echo ${@} >&2
}

# 1 crate を移植です。
function migrate_one_crate() {
    # Arguments
    SOURCE=${1}

    CHAPTER_NAME=$(echo ${SOURCE} | grep --only-matching --perl-regex 'chap[[:digit:]]_[[:digit:]]')
    PROBLEM_NAME=$(echo ${SOURCE} | sed -e 's/chap[0-9]_[0-9]_\(.*\)/\1/g' )

    debug_echo "Migrating ${CHAPTER_NAME} ${PROBLEM_NAME}..."

    # Migrate
    mkdir "crates/${CHAPTER_NAME}" -p
    cp "${SOURCE}" "crates/${CHAPTER_NAME}/${PROBLEM_NAME}" -r

    # Amend Cargo.toml
    cat ${SOURCE}/Cargo.toml                            \
        | sed -e 's/\.\.\/proconcli/..\/..\/proconcli/g'  \
        > "crates/${CHAPTER_NAME}/${PROBLEM_NAME}/Cargo.toml"
}


migrate_one_crate $1
