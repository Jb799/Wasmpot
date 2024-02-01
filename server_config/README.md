Welcome to our Wasmpot2 installation :

Set up Proxmox on your server :

# Proxmox Honeypot Setup Guide

Follow this step-by-step guide to set up your honeypot using Proxmox:

## Prerequisites

* Install Proxmox and ensure it's properly configured.
* Access the Proxmox web interface.
  
## Download Ubuntu Cloud Image

1. Navigate to the Proxmox web interface.
2. Go to "Local Proxmox" > "ISO Images."
3. Click on "Download from URL" and enter the following URL:

https://cloud-images.ubuntu.com/lunar/current/lunar-server-cloudimg-amd64-disk-kvm.img

## VM Creation

Open Proxmox shell and execute all.sh.

### Modify the following elements:

Hard Disk :
- Resize the Hard Disk = 64 Gib

Cloud:Init :
- User = ubuntu
- Password = <SET-PASSWORD>
- SSH-key = the SSH public key (find in /root/.ssh/id_rsa.pub from the Proxmox server)
- Ensure DHCP (IPv4) is checked.

## Admin VM Setup

1. Create an admin VM.
2. Upload id_rsa and id_rsa.pub from the Proxmox server to /home/ubuntu in the VM.
3. Modify k3s.sh:
3.1 Get the IP of each VM and adjust worker and master configurations.
3.2 Be cautious with the LoadBalancer IP range and modify it if necessary.
4. Upload and execute k3s.sh in /home/ubuntu.

## Useful Commands

* kubectl get nodes: Show nodes.
* kubectl get pods: Show pods.

## Helm & Rancher Installation

* Refer to helm.sh and execute it on the admin VM.

## Kata Installation for k3s

* Refer to kata.sh and execute it on the admin VM.






