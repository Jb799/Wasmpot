# Deployment Guide for Firecracker VM Service

This guide walks you through the deployment process for a Firecracker VM service using Kubernetes. The deployment involves creating a ConfigMap for initialization scripts and deploying a Kubernetes Deployment and Service for the Firecracker VM.

## Prerequisites

- A Kubernetes cluster
- `kubectl` configured to interact with your cluster

## Steps for Deployment

### 1. Deploying the ConfigMap

The ConfigMap contains an initialization script that installs Docker, pulls necessary Docker images, and sets up Docker containers.

Create a file named `configmap-firecracker.yaml` with the following content:

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: init-script
data:
  init.sh: |
    #!/bin/bash

    # Install Docker
    apt-get update
    apt-get install -y docker.io

    # Wait a few seconds to ensure the installation is complete
    sleep 10
    echo "ok"

    # Check if the Docker service is started
    while ! service docker start > /dev/null 2>&1; do
        echo "Waiting for Docker to start..."
        sleep 1
    done

    # Pull Docker images in parallel
    docker pull jb799/wasmpot-resource-server:keycloak &
    docker pull jb799/wasmpot-logical-server:keycloak &
    wait

    # Create Docker network
    docker network create wp2_network

    # Switch user context and run Docker commands
    docker run -d -p 8000:8000 -e WASI_PORT=8000 --network wp2_network --name wasi-container jb799/wasmpot-logical-server:keycloak 8000 8888 actix-container 8068 $NODE_IP wp2
    docker run -d -p 8888:8888 -e ACTIX_PORT=8888 -e WASI_ADDR=keycloak.authgates.com --network wp2_network --name actix-container jb799/wasmpot-resource-server:keycloak

    # End message
    echo "Docker containers installation and startup completed successfully."

    # Keep the script running if necessary (for Docker containers)
    tail -f /dev/null
```

Apply the ConfigMap using kubectl:

```
kubectl apply -f configmap-firecracker.yaml
```

### 2. Deploying the Firecracker VM Pod

The deployment YAML defines a Kubernetes Deployment with an init container that copies and executes the initialization script from the ConfigMap. It also sets up a service to expose the necessary ports.

Create a file named firecracker-vm-pod.yaml with the following content:

```
apiVersion: apps/v1
kind: Deployment
metadata:
  name: firecracker-vm-deployment
spec:
  replicas: 4
  selector:
    matchLabels:
      app: firecracker-vm
  template:
    metadata:
      labels:
        app: firecracker-vm
    spec:
      initContainers:
      - name: init-script-copier
        image: busybox
        command: ['sh', '-c', 'cp /mnt/init-script/init.sh /home/ubuntu/firecracker/init.sh && chmod +x /home/ubuntu/firecracker/init.sh']
        volumeMounts:
        - name: init-script
          mountPath: /mnt/init-script
        - name: firecracker-dir
          mountPath: /home/ubuntu/firecracker
      containers:
      - name: firecracker-agent
        image: avhost/mesos-firecracker-executor
        command: ["/bin/sh", "-c", "/home/ubuntu/firecracker/init.sh"]
        volumeMounts:
        - name: firecracker-dir
          mountPath: /home/ubuntu/firecracker
        ports:
        - containerPort: 22
        - containerPort: 8888  # Adding port 8888
        securityContext:
          privileged: true
        env:
        - name: NODE_IP
          valueFrom:
            fieldRef:
              fieldPath: status.hostIP
      volumes:
      - name: init-script
        configMap:
          name: init-script
      - name: firecracker-dir
        emptyDir: {}
---
apiVersion: v1
kind: Service
metadata:
  name: firecracker-vm-service
spec:
  type: LoadBalancer
  selector:
    app: firecracker-vm
  ports:
    - name: ssh
      protocol: TCP
      port: 22
      targetPort: 22
    - name: logsserver
      protocol: TCP
      port: 8068
      targetPort: 8068
    - name: wasiserver
      protocol: TCP
      port: 8000
      targetPort: 8000
```

Apply the Deployment and Service using kubectl:
```
kubectl apply -f firecracker-vm-pod.yaml
```

##Explanation of YAML Files
ConfigMap (configmap-firecracker.yaml): This file creates a ConfigMap named init-script that contains an initialization script. The script installs Docker, pulls required Docker images, sets up a Docker network, and runs the Docker containers.

Deployment (firecracker-vm-pod.yaml): This file defines a Kubernetes Deployment for Firecracker VM. It includes:

Init Container: Copies the initialization script from the ConfigMap and makes it executable.
Main Container: Executes the initialization script to set up the environment.
Volumes: Uses an emptyDir volume to hold the initialization script.
Service: Exposes the Firecracker VM via a LoadBalancer service on ports 22 (SSH), 8068 (log server), and 8000 (WASI server).
