#!/bin/bash

# Docker Installation
echo -e " \033[32;5mDocker Installation !\033[0m"

sudo apt update
sudo apt install apt-transport-https ca-certificates curl software-properties-common
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg
echo "deb [arch=amd64 signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
sudo apt update 
sudo apt install docker-ce docker-ce-cli containerd.io
sudo docker --version

# Démarrage du service Docker
service docker start
sleep 10

#Creation Dossier Firecracker
mkdir /home/ubuntu/firecracker
cf /home/ubuntu/firecracker
curl -sO https://raw.githubusercontent.com/Jb799/Wasmpot/main/server_config/firecracker/configmap-firecracker.yaml
curl -sO https://raw.githubusercontent.com/Jb799/Wasmpot/main/server_config/firecracker/firecracker-vm-pod.yaml
curl -sO https://raw.githubusercontent.com/Jb799/Wasmpot/main/server_config/firecracker/firecracker-vm-service.yaml

# Pull de l'image docker de Firecracker
docker pull avhost/mesos-firecracker-executor:latest
sleep 10

kubectl apply -f configmap-firecracker.yaml
kubectl apply -f firecracker-vm-pod.yaml
kubectl apply -f firecracker-vm-service.yaml

kubectl exec -it firecracker-vm-deployment-*