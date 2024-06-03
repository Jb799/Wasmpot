#### 🕸️ **Wasmpot2: The First Keycloak and GitLab-Based Honeypot** 🐝

### Overview 🌐
Wasmpot2 is an innovative honeypot solution utilizing Keycloak and GitLab for enhanced security. It's the first of its kind, designed to be highly effective in trapping and analyzing threats within a secure Kubernetes environment.

### Features 🚀
- **Keycloak and GitLab Integration:** Leverages Keycloak and GitLab for secure authentication and comprehensive threat simulation.
- **Advanced Threat Detection:** Efficiently identifies and analyzes potential security threats using cutting-edge technologies.
- **Multi-layered Security:** Deploys on a secure Kubernetes architecture with Firecracker microVMs and OCI containers for enhanced isolation.
- **Real-time Monitoring:** Continuous log monitoring and analysis using Elasticsearch and Kibana for swift detection and response to suspicious activities.
- **Honeypot Generator:** A Python-based program capable of creating any honeypot from a local server, making the setup versatile and customizable.

### Technologies Used 💻
- **Rust:** All services are coded in Rust, ensuring high performance and security.
- **WebAssembly:** Enhances security and performance within the honeypot environment.
- **Cloudflare:** Provides robust protection against DDoS attacks and ensures secure access to the honeypots.
- **Elasticsearch and Kibana:** Facilitates efficient log collection, monitoring, and visualization for in-depth threat analysis.
- **Proxmox and Rancher:** Used for virtualization and Kubernetes cluster management.
- **Docker:** Containers are used to run applications and microservices securely.
- **Firecracker** as an hardened VMM to host our containers (services)

### Installation 🛠️
**Prerequisites:**
- Kubernetes cluster with at least 4 Linux OS servers.
- Docker and Firecracker installed.
- Elasticsearch and Kibana setup for log analysis.
- Proxmox and Rancher configured for virtualization and cluster management.

**Instructions:**
1. **Setup Kubernetes Cluster:**
   - lien vers les script de duncan
2. **Deploy Wasmpot2:**
   - Clone the repository: `git clone https://github.com/your-repo/wasmpot2.git`
   - Navigate to the directory: `cd wasmpot2`
   - Deploy the infrastructure: `kubectl apply -f script_duncan_k8S`
   - Verify deployment: `kubectl get pods`

### Usage 📚
**Guidelines:**
1. **Access the Honeypots:**
   - Access Keycloak honeypot at: `http://your-cluster-ip/keycloak`
   - Access GitLab honeypot at: `http://your-cluster-ip/gitlab`
2. **Monitor Logs:**
   - Use Elasticsearch and Kibana for real-time log monitoring and analysis.
3. **Generate New Honeypots:**
   - Use the Python-based honeypot generator to create new honeypots from local servers.
4. **Analyze Threats:**
   - Regularly review logs and alerts to understand and mitigate potential threats.

### Architectures 🕸️

**Kubernetes:** 

<img width="1116" alt="Capture d’écran 2024-06-03 à 16 08 05" src="https://github.com/Jb799/Wasmpot/assets/62985330/9c25d1f2-85fc-41c4-9a4e-ccc6581cc9dd">


<img width="997" alt="Capture d’écran 2024-06-03 à 16 04 11" src="https://github.com/Jb799/Wasmpot/assets/62985330/03765a9e-59e8-4d5a-ae46-1b3c1105f49c">

**Servers:** 

<img width="837" alt="Capture d’écran 2024-06-03 à 16 12 36" src="https://github.com/Jb799/Wasmpot/assets/62985330/540a12f9-5c81-4a83-9b07-540c6c81b086">

<img width="524" alt="Capture d’écran 2024-06-03 à 16 13 27" src="https://github.com/Jb799/Wasmpot/assets/62985330/f1636e54-b60c-4922-aab4-740a1ed4986d">


### Documentation 📄
- **Project Charter:** Overview and objectives of Wasmpot2.
- **Requirements Collection:** Detailed requirements and specifications.
- **Risk Analysis:** Risk matrix and mitigation strategies.
- **WBS (Work Breakdown Structure):** Detailed breakdown of project tasks.
- **RACI Matrix:** Roles and responsibilities of team members.
- **Gantt Chart:** Timeline and milestones.
- **Poster:** Communication material for project awareness.

### Contributors 🤝
- Jean-Baptiste Jail.
- Duncan Sayn.
- Agathe Mullot.
- Jean-Baptiste Relave.
- François Palayer

### License 📄
Details about the project's license can be found in the `LICENSE` file in the repository.

### Contact 📩
For more information or support, please reach out to the project maintainers:
- Email: support@wasmpot2.com
- GitHub: [Wasmpot2 Repository](https://github.com/your-repo/wasmpot2)

🌟 **Star us on GitHub & follow for updates!**
