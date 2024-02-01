qm create 100 --memory 16384 --sockets 4 --core 4 --name wasmpot-k3s-01 --net0 virtio,bridge=vmbr0 --cpu host --numa 1
cd /var/lib/vz/template/iso/
qm importdisk 100 lunar-server-cloudimg-amd64-disk-kvm.img local-lvm
qm set 100 --scsihw virtio-scsi-pci --scsi0 local-lvm:vm-100-disk-0
qm set 100 --ide2 local-lvm:cloudinit
qm set 100 --boot c --bootdisk scsi0
qm set 100 --serial0 socket --vga serial0
qm set 100 --balloon 0

qm create 101 --memory 16384 --sockets 4 --core 4 --name wasmpot-k3s-02 --net0 virtio,bridge=vmbr0 --cpu host --numa 1
cd /var/lib/vz/template/iso/
qm importdisk 101 lunar-server-cloudimg-amd64-disk-kvm.img local-lvm
qm set 101 --scsihw virtio-scsi-pci --scsi0 local-lvm:vm-101-disk-0
qm set 101 --ide2 local-lvm:cloudinit
qm set 101 --boot c --bootdisk scsi0
qm set 101 --serial0 socket --vga serial0
qm set 101 --balloon 0

qm create 102 --memory 16384 --sockets 4 --core 4 --name wasmpot-k3s-03 --net0 virtio,bridge=vmbr0 --cpu host --numa 1
cd /var/lib/vz/template/iso/
qm importdisk 102 lunar-server-cloudimg-amd64-disk-kvm.img local-lvm
qm set 102 --scsihw virtio-scsi-pci --scsi0 local-lvm:vm-102-disk-0
qm set 102 --ide2 local-lvm:cloudinit
qm set 102 --boot c --bootdisk scsi0
qm set 102 --serial0 socket --vga serial0
qm set 102 --balloon 0

qm create 103 --memory 16384 --sockets 4 --core 4 --name wasmpot-k3s-04 --net0 virtio,bridge=vmbr0 --cpu host --numa 1
cd /var/lib/vz/template/iso/
qm importdisk 103 lunar-server-cloudimg-amd64-disk-kvm.img local-lvm
qm set 103 --scsihw virtio-scsi-pci --scsi0 local-lvm:vm-103-disk-0
qm set 103 --ide2 local-lvm:cloudinit
qm set 103 --boot c --bootdisk scsi0
qm set 103 --serial0 socket --vga serial0
qm set 103 --balloon 0

qm create 104 --memory 16384 --sockets 4 --core 4 --name wasmpot-k3s-05 --net0 virtio,bridge=vmbr0 --cpu host --numa 1
cd /var/lib/vz/template/iso/
qm importdisk 104 lunar-server-cloudimg-amd64-disk-kvm.img local-lvm
qm set 104 --scsihw virtio-scsi-pci --scsi0 local-lvm:vm-104-disk-0
qm set 104 --ide2 local-lvm:cloudinit
qm set 104 --boot c --bootdisk scsi0
qm set 104 --serial0 socket --vga serial0
qm set 104 --balloon 0

qm create 105 --memory 16384 --sockets 4 --core 4 --name wasmpot-k3s-admin --net0 virtio,bridge=vmbr0 --cpu host --numa 1
cd /var/lib/vz/template/iso/
qm importdisk 105 lunar-server-cloudimg-amd64-disk-kvm.img local-lvm
qm set 105 --scsihw virtio-scsi-pci --scsi0 local-lvm:vm-105-disk-0
qm set 105 --ide2 local-lvm:cloudinit
qm set 105 --boot c --bootdisk scsi0
qm set 105 --serial0 socket --vga serial0
qm set 105 --balloon 0

echo "                  "
echo "                  "
cat /root/.ssh/id_rsa.pub
echo "                  "
echo "---------------------------"
echo "                  "
cat /root/.ssh/id_rsa