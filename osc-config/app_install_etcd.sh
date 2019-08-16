#!/bin/bash
set -e

# ETCD configuration
ETCD_VER=v3.3.13
GITHUB_URL=https://github.com/etcd-io/etcd/releases/download
DOWNLOAD_URL=${GITHUB_URL}

rm -f /tmp/etcd-${ETCD_VER}-linux-amd64.tar.gz
rm -rf /tmp/etcd-download-test && mkdir -p /tmp/etcd-download-test

curl -L ${DOWNLOAD_URL}/${ETCD_VER}/etcd-${ETCD_VER}-linux-amd64.tar.gz -o /tmp/etcd-${ETCD_VER}-linux-amd64.tar.gz
tar xzvf /tmp/etcd-${ETCD_VER}-linux-amd64.tar.gz -C /tmp/etcd-download-test --strip-components=1
rm -f /tmp/etcd-${ETCD_VER}-linux-amd64.tar.gz

get_default_host_ip() {
    local host_ip=$1
    local af=$2
    # Search for an IP unless an explicit is set by ``HOST_IP`` environment variable
    if [ -z "$host_ip" ]; then
        host_ip=""
        # Find the interface used for the default route
        host_ip_iface=${host_ip_iface:-$(ip -f $af route | awk '/default/ {print $5}' | head -1)}
        local host_ips
        host_ips=$(LC_ALL=C ip -f $af addr show ${host_ip_iface} | sed /temporary/d |awk /$af'/ {split($2,parts,"/");  print parts[1]}')
        local ip
        for ip in $host_ips; do
            host_ip=$ip
            break;
        done
    fi
    echo $host_ip
}

# Global
HOST_IP=${HOST_IP:-}
HOST_IP=$(get_default_host_ip "$HOST_IP" "inet")
# Start etcd
nohup /tmp/etcd-download-test/etcd \
    --advertise-client-urls http://${HOST_IP}:2379 \
    --listen-client-urls http://${HOST_IP}:2379 \
    --listen-peer-urls http://${HOST_IP}:2380 >/dev/null &
