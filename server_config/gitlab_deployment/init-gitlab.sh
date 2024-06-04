#!/bin/bash

# Installation de Docker
apt-get update
sleep 20
apt-get install -y docker.io
sleep 10

# Démarrage du service Docker
service docker start
sleep 10

# Pull des images Docker nécessaires
docker pull jb799/wasmpot-resource-server:gitlab
sleep 10
docker pull jb799/wasmpot-logical-server:gitlab
sleep 10

# Création du réseau Docker
docker network create wp2_network
sleep 10

# Démarrage des conteneurs avec les arguments nécessaires
docker run -d -p 8000:8000 -e WASI_PORT=8000 --network wp2_network --name wasi-container jb799/wasmpot-logical-server:gitlab 8000 8888 actix-container 8068 10.10.50.100

sleep 20

docker run -d -p 8888:8888 -e ACTIX_PORT=8888 -e WASI_PORT=8000 -e WASI_ADDR=wasi-container --network wp2_network --name actix-container jb799/wasmpot-resource-server:gitlab

apt-get upgrade

# Keep the container running
    tail -f /dev/null