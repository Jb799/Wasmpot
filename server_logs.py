from flask import Flask, request, jsonify
from logging.handlers import RotatingFileHandler
import os
from datetime import datetime
import csv

app = Flask(__name__)

# Configuration de la taille maximale du fichier de log en octets (ex. 5MB)
MAX_LOG_SIZE = 5 * 1024 * 1024  # 5 MB
LOG_DIR = '/home/firecracker/logs'  # Utilisez un répertoire pour stocker les logs
LOG_FILENAME_TEMPLATE = 'wp2_log_{}.log'

# Créez un gestionnaire de fichiers rotatif
def get_rotating_file_handler():
    current_date = datetime.now().strftime('%Y-%m-%d')
    log_filename = os.path.join(LOG_DIR, LOG_FILENAME_TEMPLATE.format(current_date))
    
    # Vérifiez si le fichier existe et est vide
    file_exists = os.path.isfile(log_filename)
    file_empty = os.path.getsize(log_filename) == 0 if file_exists else True

    handler = RotatingFileHandler(log_filename, maxBytes=MAX_LOG_SIZE, backupCount=5)
    
    # Si le fichier est vide, ajoutez l'en-tête
    if file_empty:
        with open(log_filename, 'a') as log_file:
            log_file.write("Timestamp,ID,FLAG,Method,Endpoint,Payload,IP,Country,Latitude_Longitude\n")
    
    return handler

@app.route('/save_log', methods=['POST'])
def save_log():
    log_line = request.get_json()
    handler = get_rotating_file_handler()

    # Convertir le log en ligne CSV
    csv_line = f'{log_line["timestamp"]},{log_line["id"]},{log_line["flag"]},"{log_line["method"]}",{log_line["path"]},"{log_line["query_string"]}","{log_line["client_ip"]}","{log_line["country"]}","{log_line["loc"]}"\n'

    print(csv_line)

    try:
        with handler.stream as log_file:
            log_file.write(csv_line)
        return jsonify({"status": "Log saved successfully"}), 200
    except Exception as e:
        return jsonify({"error": str(e)}), 500

if __name__ == '__main__':
    if not os.path.exists(LOG_DIR):
        os.makedirs(LOG_DIR)
    app.run(host='0.0.0.0', port=8068)
