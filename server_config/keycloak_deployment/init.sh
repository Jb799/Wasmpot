#!/bin/bash

    # Installation de Docker
    apt-get update
    apt-get install -y docker.io

    # Attendre quelques secondes pour s'assurer que l'installation est terminée
    sleep 10
    echo "ok"

    # Vérification que le service Docker est bien démarré
    while ! service docker start > /dev/null 2>&1; do
        echo "En attente que Docker démarre..."
        sleep 1
    done

    # Pull des images Docker en parallèle
    docker pull jb799/wasmpot-resource-server:keycloak &
    docker pull jb799/wasmpot-logical-server:keycloak &
    wait

    # Création du réseau Docker
    docker network create wp2_network

    # Changement de contexte utilisateur et exécution des commandes Docker
        docker run -d -p 8000:8000 -e WASI_PORT=8000 --network wp2_network --name wasi-container jb799/wasmpot-logical-server:keycloak 8000 8888 actix-container 8068 $NODE_IP wp2
        docker run -d -p 8888:8888 -e ACTIX_PORT=8888 -e WASI_ADDR=keycloak.authgates.com --network wp2_network --name actix-container jb799/wasmpot-resource-server:keycloak

    # Message de fin
    echo "Installation et démarrage des conteneurs Docker terminés avec succès."

    # Cette commande garde le script en attente si nécessaire (pour les conteneurs Docker)
    tail -f /dev/null
