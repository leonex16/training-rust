#!/bin/bash
log_info() {
  message=$1
  separator=${2-*}

  printf "\n%75s\n" | tr " " "${separator}"
  printf "${message}"
  printf "\n%75s\n" | tr " " "${separator}"
}

alias cr="cargo run"
alias crf="cargo +nightly fmt && cargo run"

log_info "Downloading necessary dependencies..."
apk add build-base
rustup default nightly
rustup component add rustfmt --toolchain nightly
cargo install cargo-watch

if [[ ! -f Cargo.toml ]]
then
  log_info "The project is being created..."
  cargo init .
else
  log_info cho "Downloading cargo dependencies..."
  cargo fetch --verbose --color always
fi

chmod u+x ./init-docker.bash
cargo watch --clear -x run


exit
