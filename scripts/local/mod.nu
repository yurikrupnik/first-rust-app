def main [] {
    kind create cluster
    let kro_version = http get https://api.github.com/repos/kro-run/kro/releases/latest | get tag_name

    (
        helm install kro oci://ghcr.io/kro-run/kro/kro \
          --namespace kro \
          --create-namespace \
          --version=$kro_version
    )
}