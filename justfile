#!/usr/bin/env just --justfile

hello:
  echo "hello world"
build:
    cargo clippy
    cargo doc --examples
    cargo test
    cargo build

#    docker build -t yurikrupnik/config-files .
#    nu config.nu app list
#    nu config.nu config validate
#    docker build -t yurikrupnik/config-files .
nuds:
    nu config.nu app list
cluster:
    nu ~/configs-files/scripts/nx.nu
    nu ~/configs-files/scripts/generate-shell-configs.nu
    #nu ~/configs-files/scripts/generate-shell-configs.nu
#    gcloud auth configure-docker \
#        me-west1-docker.pkg.dev

