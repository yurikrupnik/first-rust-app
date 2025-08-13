load('ext://helm_resource', 'helm_resource', 'helm_repo')

#k8s_yaml('k8s/cnpg/cluster.yaml')
#k8s_yaml('k8s/mongodb/deployment.yaml')
#k8s_yaml('k8s/redis/deployment.yaml')
# k8s_yaml('k8s/neo4j/deployment.yaml')
k8s_yaml(kustomize('k8s/base'))
# k8s_yaml(kustomize('manifests/db'))
#k8s_resource('postgres-cluster-1', resource_deps=['postgres-credentials'], port_forwards=['5432:5432'])
#k8s_resource('mongodb', resource_deps=[], port_forwards=['27017:27017'])
#k8s_resource('redis', resource_deps=[], port_forwards=['6379:6379'])
#k8s_resource('neo4j', resource_deps=[], port_forwards=['7474:7474', '7687:7687'])

docker_build(
  'first-rust-app',
  '.',
  dockerfile='Dockerfile',
  only=[
    './src',
    './Cargo.toml',
    './Cargo.lock',
    './migrations'
  ],
  # live_update=[
  #   sync('./src', '/app/src'),
  #   run('cargo build --release', trigger=['./src', './Cargo.toml'])
  # ]
)

# k8s_yaml('k8s/app/deployment.yaml')
k8s_resource('rust-app', 
  resource_deps=['postgres-cluster-1', 'mongodb', 'redis', 'neo4j'],
  port_forwards=['8080:8080']
)

local_resource(
  'local-applications',
  cmd='just cluster',
  #cmd='nu -c "source ~/private/lili1/scripts/nu/config.nu; get_local_applications"',
  deps=['src', 'Cargo.toml'],
  labels=['development'],
  auto_init=False
)
local_resource(
  'cargo-check',
  cmd='cargo check',
  deps=['src', 'Cargo.toml'],
  labels=['development'],
  auto_init=False
)

local_resource(
  'cargo-test', 
  cmd='cargo test --lib',
  deps=['src', 'Cargo.toml'],
  labels=['testing'],
  auto_init=False
)

local_resource(
  'cargo-doc',
  cmd='cargo doc --no-deps --document-private-items',
  deps=['src', 'Cargo.toml'],
  labels=['documentation'],
  auto_init=False
)

local_resource(
  'migrations',
  cmd='cargo run --bin migrate',
  deps=['migrations'],
  labels=['database'],
  resource_deps=['postgres-cluster-1'],
  auto_init=False
)

local_resource(
  'e2e-test',
  cmd='bun test:e2e',
  deps=['e2e-tests'],
  labels=['testing'],
  resource_deps=['rust-app'],
  auto_init=False
)
