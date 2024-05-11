
docker_build(
  "yurikrupnik/first-rust-app",
  ".",
  dockerfile="./local.Dockerfile",
  target="final",
)

k8s_yaml(kustomize('k8s/base'))


k8s_resource("rust-app", port_forwards="5454:8080")