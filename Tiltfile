
# local_resource(
#   'build-node-api-rest',
#   dir="../../..",
#   cmd='pnpm nx run node-nest-app:build',
#   deps=['.', '../../../libs/node'],
#   ignore=["k8s"],
#   env={"GOOS":"linux","GOARCH":"amd64"},
# )

docker_build(
  "yurikrupnik/first-rust-app",
  ".",
  target="final",
   build_args={"DIST_PATH":"target/release/first-rust-app"},
)

k8s_yaml(kustomize('k8s/base'))


