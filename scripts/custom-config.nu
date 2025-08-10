# Example custom configuration file
# You can override any default values here

{
    cluster_name: "my-custom-cluster"
    image_name: "my-rust-app"
    image_tag: "v1.0.0"
    timeout: "600s"
    local: true
    databases: {
        postgres: {
            user: "myuser"
            password: "admin"
            database: "admin"
        }
        mongodb: {
            user: "admin"
            password: "admin"
        }
        influxdb: {
            user: "admin"
            password: "admin"
        }
        redis: {
            user: "admin"
            password: "admin"
        }
    }
    
    cleanup: {
        force_cleanup: true
        cleanup_volumes: true
    }
}