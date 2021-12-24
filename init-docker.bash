#!/bin/bash
log_info() {
  message=$1
  separator=${2-*}

  echo $message
  printf "${separator}%.0s" {1..100}
}

alias cwr="cargo watch -x run"

log_info "Downloading necessary dependencies..."
apk add build-base
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

