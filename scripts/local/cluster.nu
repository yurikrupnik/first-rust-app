
def main [] {
    kind create cluster
    let tiltfile_path = "Tiltfile"

    if ($tiltfile_path | path exists) and (not $force) {
        print $"Tiltfile already exists. Use --force to overwrite."
        return
    }
}

def "main delete" [] {
    kind delete cluster
}

def "local create tiltfile" [
    app: string
    _host: string = "yurikrupnik"
    _app: string = "my-app"
    _port: number = 8080
    --image: string = "$_host/$app $_app"
    --port: string = "5454:8080"
    --resource-name: string = "rust-app"
    --kustomize-path: string = "k8s/base"
    --force (-f)
] {
    let tiltfile_path = "Tiltfile"
    
    if ($tiltfile_path | path exists) and (not $force) {
        print $"Tiltfile already exists. Use --force to overwrite."
        return
    }
    
    let content = $"docker_build(
  \"($image)\",
  \".\",
  target=\"final\",
)

k8s_yaml(kustomize('($kustomize_path)'))


k8s_resource(\"($resource_name)\", port_forwards=\"($port)\")"
    
    $content | save --force $tiltfile_path
    print $"Generated Tiltfile with image: ($image), port: ($port), resource: ($resource_name)"
}