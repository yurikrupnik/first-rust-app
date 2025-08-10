#!/usr/bin/env nu

use config.nu *

def delete_cluster [config: record] {
    print "🗑️ Deleting Kind cluster..."
    try {
        kind delete cluster --name $config.cluster_name
        print $"✅ Kind cluster ($config.cluster_name) deleted successfully"
    } catch {
        print $"⚠️ Failed to delete Kind cluster ($config.cluster_name) - it may not exist"
        return false
    }
    true
}

def remove_docker_images [config: record] {
    if not $config.cleanup_images {
        print "⏭️ Skipping Docker image cleanup"
        return true
    }
    
    let full_image = $"($config.image_name):($config.image_tag)"
    print $"🗑️ Removing Docker image ($full_image)..."
    
    try {
        docker rmi $full_image
        print $"✅ Docker image ($full_image) removed successfully"
    } catch {
        if $config.force_cleanup {
            try {
                docker rmi --force $full_image
                print $"✅ Docker image ($full_image) force removed"
            } catch {
                print $"⚠️ Failed to remove Docker image ($full_image) - it may not exist"
                return false
            }
        } else {
            print $"⚠️ Failed to remove Docker image ($full_image) - it may not exist"
            return false
        }
    }
    true
}

def cleanup_docker_volumes [config: record] {
    if not $config.cleanup_volumes {
        return true
    }
    
    print "🗑️ Cleaning up Docker volumes..."
    try {
        let unused_volumes = (docker volume ls --filter dangling=true --quiet)
        if ($unused_volumes | length) > 0 {
            docker volume rm ...$unused_volumes
            print "✅ Unused Docker volumes cleaned up"
        } else {
            print "ℹ️ No unused Docker volumes to clean up"
        }
    } catch {
        print "⚠️ Failed to clean up Docker volumes"
        return false
    }
    true
}

def cleanup_kubectl_resources [config: record] {
    print "🗑️ Cleaning up kubectl context..."
    try {
        let contexts = (kubectl config get-contexts --output=name | lines)
        let cluster_context = $"kind-($config.cluster_name)"
        
        if ($cluster_context in $contexts) {
            kubectl config delete-context $cluster_context
            print $"✅ Kubectl context ($cluster_context) deleted"
        } else {
            print $"ℹ️ Kubectl context ($cluster_context) not found"
        }
    } catch {
        print "⚠️ Failed to clean up kubectl context"
        return false
    }
    true
}

def print_cleanup_summary [results: record] {
    print ""
    print "📊 Cleanup Summary:"
    print $"  Kind cluster: (if $results.cluster { '✅ Deleted' } else { '❌ Failed' })"
    print $"  Docker images: (if $results.images { '✅ Cleaned' } else { '❌ Failed' })"
    print $"  Docker volumes: (if $results.volumes { '✅ Cleaned' } else { '⏭️ Skipped' })"
    print $"  Kubectl context: (if $results.kubectl { '✅ Cleaned' } else { '❌ Failed' })"
}

def main [--config-file(-c): string, --cluster-name: string, --force(-f), --volumes(-v), --no-images] {
    let base_config = (load_config $config_file)
    let runtime_config = {
        ...($base_config | merge $base_config.cleanup),
        cluster_name: ($cluster_name | default $base_config.cluster_name),
        force_cleanup: $force,
        cleanup_volumes: $volumes,
        cleanup_images: (not $no_images)
    }
    
    print "🧹 Cleaning up Kind cluster and resources"
    print $"Configuration: cluster=($runtime_config.cluster_name), force=($runtime_config.force_cleanup), volumes=($runtime_config.cleanup_volumes), images=($runtime_config.cleanup_images)"
    print ""
    
    let cluster_result = (delete_cluster $runtime_config)
    let images_result = (remove_docker_images $runtime_config)
    let volumes_result = (cleanup_docker_volumes $runtime_config)
    let kubectl_result = (cleanup_kubectl_resources $runtime_config)
    
    let results = {
        cluster: $cluster_result,
        images: $images_result,
        volumes: $volumes_result,
        kubectl: $kubectl_result
    }
    
    print_cleanup_summary $results
    
    if $cluster_result and $images_result and $volumes_result and $kubectl_result {
        print "✅ Cleanup complete!"
        0
    } else {
        print "⚠️ Cleanup completed with some warnings"
        1
    }
}