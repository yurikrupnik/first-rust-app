#!/usr/bin/env just --justfile
#base on this data
#https://zerotomastery.io/blog/top-cargo-subcommands-for-rust-development/
#https://developer.1password.com/docs/cli/reference/management-commands/vault

hello:
  echo "hello world"

# Development commands
dev:
    cargo run

dev-watch:
    cargo watch -x run

# Build and test commands
build:
    cargo clippy --all-targets --all-features -- -D warnings
    cargo doc --all --no-deps --document-private-items
    cargo test --lib
    cargo build --release

build-debug:
    cargo build

# Testing commands
test:
    cargo test --lib

test-all:
    cargo test

test-integration:
    cargo test --test '*'

test-e2e:
    bun test:e2e

test-watch:
    cargo watch -x "test --lib"

# Analysis and profiling commands
fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all -- --check

clippy:
    cargo clippy --all-targets --all-features -- -D warnings

clippy-fix:
    cargo clippy --fix --all-targets --all-features

clippy-report:
    cargo clippy --all-targets --all-features --message-format=json > clippy-report.json

doc:
    cargo doc --all --no-deps --document-private-items --open

doc-private:
    cargo rustdoc --all-features -- --document-private-items

expand-macros:
    cargo expand --all-features > macro-expansion.rs

check-unused-deps:
    cargo machete

check-outdated:
    cargo outdated

security-audit:
    cargo audit

security-deny:
    cargo deny check

# Coverage and profiling
coverage:
    cargo tarpaulin --out xml --out html --output-dir coverage

coverage-open:
    cargo tarpaulin --out html --output-dir coverage --open

flamegraph:
    cargo flamegraph --bin first-rust-app

bench:
    cargo bench

bench-flamegraph:
    cargo flamegraph --bench --all-features -o flamegraph.svg

# Docker commands
docker-build:
    docker build -t ghcr.io/yurikrupnik/first-rust-app:latest .

docker-build-multi:
    docker buildx build --platform linux/amd64,linux/arm64 -t ghcr.io/yurikrupnik/first-rust-app:latest .

docker-push:
    docker push ghcr.io/yurikrupnik/first-rust-app:latest

docker-run:
    docker run -p 3000:3000 --env-file .env ghcr.io/yurikrupnik/first-rust-app:latest

# Kubernetes commands
k8s-apply-dev:
    kubectl apply -f k8s/db/
    kubectl apply -f k8s/app/

k8s-deploy-dev:
    kubectl apply -k k8s/

k8s-delete-dev:
    kubectl delete -k k8s/

k8s-logs:
    kubectl logs -f deployment/first-rust-app

k8s-port-forward:
    kubectl port-forward service/first-rust-app 3000:3000

k8s-restart:
    kubectl rollout restart deployment/first-rust-app

# Kind cluster management
kind-create:
    kind create cluster --config kind-cluster.yaml

kind-delete:
    kind delete cluster

kind-load-image:
    kind load docker-image ghcr.io/yurikrupnik/first-rust-app:latest

# Tekton pipeline commands
tekton-install:
    kubectl apply --filename https://storage.googleapis.com/tekton-releases/pipeline/latest/release.yaml
    kubectl apply --filename https://storage.googleapis.com/tekton-releases/triggers/latest/release.yaml

tekton-apply:
    kubectl apply -f k8s/tekton/

tekton-run:
    kubectl create -f k8s/tekton/pipelinerun.yaml

tekton-logs:
    tkn pipelinerun logs --last -f

tekton-list:
    tkn pipelinerun list

# FluxCD GitOps commands
flux-install:
    flux install

flux-bootstrap:
    flux bootstrap github --owner=yurikrupnik --repository=first-rust-app --branch=main --path=./k8s/gitops/flux

flux-status:
    flux get all

flux-reconcile:
    flux reconcile source git first-rust-app

# Version management commands
version-current:
    nu scripts/version-manager.nu current

version-bump-patch:
    nu scripts/version-manager.nu bump patch

version-bump-minor:
    nu scripts/version-manager.nu bump minor

version-bump-major:
    nu scripts/version-manager.nu bump major

version-tag:
    nu scripts/version-manager.nu tag

version-update-manifests tag:
    nu scripts/version-manager.nu update-manifests {{ tag }}

version-release tag:
    nu scripts/version-manager.nu release {{ tag }}

# Complete release workflow
release level:
    nu scripts/version-manager.nu bump {{ level }}
    nu scripts/version-manager.nu tag
    git push origin main --tags
    just docker-build-multi
    just docker-push

# CI/CD analysis commands
ci-analysis:
    just fmt-check
    just clippy-report
    just security-audit
    just security-deny
    just check-unused-deps
    just check-outdated
    just expand-macros
    just coverage

ci-full:
    just ci-analysis
    just test-all
    just bench
    just flamegraph
    just doc

ci-version-update:
    nu scripts/version-manager.nu ci-version-update

# Database commands
db-migrate:
    sqlx migrate run --database-url $DATABASE_URL

db-migrate-revert:
    sqlx migrate revert --database-url $DATABASE_URL

db-reset:
    sqlx database drop --database-url $DATABASE_URL -y
    sqlx database create --database-url $DATABASE_URL
    just db-migrate

# Environment setup
setup-env:
    nu scripts/setup-env.nu

setup-dev-tools:
    cargo install --locked cargo-tarpaulin cargo-flamegraph cargo-expand cargo-machete cargo-outdated cargo-audit cargo-deny cargo-watch

setup-cluster:
    just kind-create
    just tekton-install
    just flux-install

# Bootstrap complete development environment
bootstrap:
    just setup-env
    just setup-dev-tools
    just setup-cluster

# Cleanup commands
clean:
    cargo clean
    rm -rf coverage/
    rm -f clippy-report.json macro-expansion.rs flamegraph.svg

clean-all:
    just clean
    docker system prune -f
    kind delete cluster -n rust-app-cluster

# Legacy commands
nuds:
    nu config.nu app list
    
cluster:
    nu ~/configs-files/scripts/nx.nu
    nu ~/configs-files/scripts/generate-shell-configs.nu
local:
    just bootstrap

local-clean:
    just clean-all
secrets:
    op whoami
    op vault list
    teller scan
    echo 'foo: ref+gcpsecrets://playground-447016/github-secret' | vals eval -f -