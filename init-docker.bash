#!/bin/bash

if [[ ! -f Cargo.toml ]]
then
  cargo init .
  apk add build-base
  cargo install cargo-watch
fi

chmod u+x ./init-docker.bash
cargo watch --clear -x run

exit

