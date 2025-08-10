#!/usr/bin/env nu

use config.nu *

def create_cluster [config: record] {
    print "📦 Creating Kind cluster..."
    try {
        kind create cluster --name $config.cluster_name --config $config.kind_config
    } catch {
        print $"❌ Failed to create Kind cluster ($config.cluster_name)"
        return false
    }
    true
}

def install_cnpg [config: record] {
    print "📦 Installing CNPG operator..."
    let cnpg_url = $"https://raw.githubusercontent.com/cloudnative-pg/cloudnative-pg/release-($config.cnpg_version)/releases/cnpg-($config.cnpg_version).0.yaml"
    
    try {
        kubectl apply -f $cnpg_url
        print "⏳ Waiting for CNPG operator to be ready..."
        kubectl wait --for=condition=Available deployment/cnpg-controller-manager -n cnpg-system --timeout=$config.timeout
    } catch {
        print "❌ Failed to install CNPG operator"
        return false
    }
    true
}

def setup_databases [config: record] {
    print "🗄️ Setting up databases..."
    
    try {
        kubectl create ns $config.namespace
    } catch {
        print $"⚠️ Namespace ($config.namespace) already exists, continuing..."
    }
    
    let databases = ["postgres", "mongodb", "influxdb", "redis", "neo4j"]
    let emojis = ["🗄️", "🗄️", "📊", "🔴"]
    
    for i in 0..3 {
        let db = ($databases | get $i)
        let emoji = ($emojis | get $i)
        
        print $"($emoji) Deploying ($db | str title-case)..."
        
        let manifest_path = match $db {
            "postgres" => $config.k8s_paths.cnpg
            "mongodb" => $config.k8s_paths.mongodb
            "influxdb" => $config.k8s_paths.influxdb
            "redis" => $config.k8s_paths.redis
            "neo4j" => $config.k8s_paths.neo4j
        }
        
        try {
            kubectl apply -f $manifest_path
        } catch {
            print $"❌ Failed to deploy ($db)"
            return false
        }
    }
    true
}

def wait_for_databases [config: record] {
    print "⏳ Waiting for databases to be ready..."
    
    let wait_commands = [
        $"kubectl wait --for=condition=Ready pod -l cnpg.io/cluster=($config.databases.postgres.cluster_name) --timeout=($config.timeout)"
        $"kubectl wait --for=condition=Ready pod -l app=mongodb --timeout=($config.timeout)"
        $"kubectl wait --for=condition=Ready pod -l app=influxdb --timeout=($config.timeout)"
        $"kubectl wait --for=condition=Ready pod -l app=redis --timeout=($config.timeout)"
        $"kubectl wait --for=condition=Ready pod -l app=neo4j --timeout=($config.timeout)"
    ]
    
    for cmd in $wait_commands {
        try {
            nu -c $cmd
        } catch {
            print $"❌ Database readiness check failed: ($cmd)"
            return false
        }
    }
    true
}

def build_and_deploy_app [config: record] {
    let full_image = $"($config.image_name):($config.image_tag)"
    
    print "🏗️ Building Docker image..."
    try {
        docker build -t $full_image .
    } catch {
        print "❌ Failed to build Docker image"
        return false
    }
    
    print "📦 Loading image into Kind cluster..."
    try {
        kind load docker-image $full_image --name $config.cluster_name
    } catch {
        print "❌ Failed to load image into Kind cluster"
        return false
    }
    
    print "🚀 Deploying Rust app..."
    try {
        kubectl apply -f $config.k8s_paths.app
        print "⏳ Waiting for app to be ready..."
        kubectl wait --for=condition=Ready pod -l app=rust-app --timeout=$config.timeout
    } catch {
        print "❌ Failed to deploy Rust app"
        return false
    }
    true
}

def print_connection_info [config: record] {
    print ""
    print "🔗 Port forward to access the app:"
    print "kubectl port-forward service/rust-app-service 8080:80"
    print ""
    print "📊 Database connection strings:"
    
    let postgres_conn = $"postgres://($config.databases.postgres.user):($config.databases.postgres.password)@localhost:($config.databases.postgres.port)/($config.databases.postgres.database)"
    let mongodb_conn = $"mongodb://($config.databases.mongodb.user):($config.databases.mongodb.password)@localhost:($config.databases.mongodb.port)"
    let redis_conn = $"redis://localhost:($config.databases.redis.port)"
    let influxdb_conn = $"http://localhost:($config.databases.influxdb.port) (($config.databases.influxdb.user):($config.databases.influxdb.password))"
    
    print $"PostgreSQL: ($postgres_conn)"
    print $"MongoDB: ($mongodb_conn)"
    print $"Redis: ($redis_conn)"
    print $"InfluxDB: ($influxdb_conn)"
}

def main [--config-file(-c): string, --cluster-name: string, --image-tag: string] {
    print "🚀 Setting up Rust app with Kind cluster and databases"
    
    let base_config = (load_config $config_file)
    let runtime_config = if ($cluster_name != null) or ($image_tag != null) {
        $base_config | merge {
            cluster_name: ($cluster_name | default $base_config.cluster_name)
            image_tag: ($image_tag | default $base_config.image_tag)
        }
    } else {
        $base_config
    }
    
    try {
        validate_config $runtime_config
    } catch {
        print $"❌ Configuration validation failed: ($in)"
        return 1
    }
    
    print $"Using configuration: cluster=($runtime_config.cluster_name), image=($runtime_config.image_name):($runtime_config.image_tag)"
    print ""
    
    if not (create_cluster $runtime_config) { return 1 }
    if not (install_cnpg $runtime_config) { return 1 }
    if not (setup_databases $runtime_config) { return 1 }
    if not (wait_for_databases $runtime_config) { return 1 }
    if not (build_and_deploy_app $runtime_config) { return 1 }
    
    print "✅ Setup complete!"
    print_connection_info $runtime_config
    0
}