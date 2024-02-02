# Welcome to our Wasmpot2 installation :🐝

# Set up Proxmox on your server :

# Proxmox Honeypot Setup Guide

Follow this step-by-step guide to set up your honeypot using Proxmox:

## Prerequisites ✅

* Install Proxmox and ensure it's properly configured.
* Access the Proxmox web interface.
  
## Download Ubuntu Cloud Image ⬇️

1. Navigate to the Proxmox web interface.
2. Go to "Local Proxmox" > "ISO Images."
3. Click on "Download from URL" and enter the following URL:

https://cloud-images.ubuntu.com/lunar/current/lunar-server-cloudimg-amd64-disk-kvm.img

## VM Creation

Open Proxmox shell and execute Set_up_vm.sh.

### Modify the following elements on each vm: ⚠️

Hard Disk :
- Resize the Hard Disk = 64 Gib

Cloud:Init :
- User = ubuntu
- Password = <SET-PASSWORD>
- SSH-key = the SSH public key (find in /root/.ssh/id_rsa.pub from the Proxmox server)
- Ensure DHCP (IPv4) is checked.

## Admin VM Setup 🚀

1. Upload id_rsa and id_rsa.pub from the Proxmox server to /home/ubuntu in the VM.
2. Modify Get_ready_honeypot.sh:
3. Get the IP of each VM and adjust worker and master configurations.
4. Be cautious with the LoadBalancer IP range and modify it if necessary.
5. Upload and execute Get_ready_honeypot.sh in /home/ubuntu.

## Useful Commands 💡

* kubectl get nodes: Show nodes.
* kubectl get pods: Show pods.

## Open a firecracker vm

1. Execute Get_ready_firecracker.sh
2. In a new terminal execute Firecracker_vm.sh

## Deploy the honeypot 🚀

1. Deploy wasi_service.yaml & actix_service.yaml with rancher
2. Use the following command in the vm admin :
   kubectl apply -f /home/ubuntu/actix_deployment.yaml
   kubectl apply -f /home/ubuntu/wasi_deployment.yaml
