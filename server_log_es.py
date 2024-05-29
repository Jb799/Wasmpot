from flask import Flask, request, jsonify
from logging.handlers import RotatingFileHandler
import os
from datetime import datetime
import requests

app = Flask(__name__)

MAX_LOG_SIZE = 5 * 1024 * 1024  # 5 MB
LOG_DIR = '/var/log/wp2'
LOG_FILENAME_TEMPLATE = 'wp2_log_{}.log'

ELASTICSEARCH_URL = 'https://10.10.50.83:9200/'
ELASTICSEARCH_USER = 'elastic'
ELASTICSEARCH_PASSWORD = 'dWDwSP38Yrn3RC0u'

ADMIN_INDEX_LIST = []

def get_rotating_file_handler():
    current_date = datetime.now().strftime('%Y-%m-%d')
    log_filename = os.path.join(LOG_DIR, LOG_FILENAME_TEMPLATE.format(current_date))
    
    file_exists = os.path.isfile(log_filename)
    file_empty = os.path.getsize(log_filename) == 0 if file_exists else True

    handler = RotatingFileHandler(log_filename, maxBytes=MAX_LOG_SIZE, backupCount=5)
    
    if file_empty:
        with open(log_filename, 'a') as log_file:
            log_file.write("Timestamp,ID,FLAG,Method,Endpoint,Payload,User-Agent,IP,Isp,Country,City,Latitude,Longitude\n")
    
    return handler

@app.route('/save_log', methods=['POST'])
def save_log():
    log_line = request.get_json()
    handler = get_rotating_file_handler()

    if not log_line['admin_index']:
        return jsonify({"error": 'Admin index not found...'}), 500
    
    if not log_line['admin_index'] in ADMIN_INDEX_LIST:
        return jsonify({"error": 'Invalid admin index...'}), 500

    csv_line = f'{log_line["timestamp"]},{log_line["id"]},{log_line["flag"]},"{log_line["method"]}","{log_line["path"]}","{log_line["query_string"]}","{log_line["client_useragent"]}","{log_line["client_ip"]}","{log_line["isp"]}","{log_line["country"]}","{log_line["city"]}",{log_line["lat"]},{log_line["long"]}\n'

    print(csv_line)

    es_payload = {
        "@timestamp": log_line["timestamp"],
        "timestamp": log_line["timestamp"],
        "ID": log_line["id"],
        "FLAG": log_line["flag"],
        "Method": log_line["method"],
        "Endpoint": log_line["path"],
        "Payload": log_line["query_string"],
        "User-Agent": log_line["client_useragent"],
        "IP": log_line["client_ip"],
        "Isp": log_line["isp"],
        "Country": log_line["country"],
        "City": log_line["city"],
        "Latitude": log_line["lat"],
        "Longitude": log_line["long"],
        "location": {
            "lat": float(log_line["lat"]),
            "lon": float(log_line["long"])
        }
    }

    try:
        es_response = requests.post(
            'ELASTICSEARCH_URL' + log_line['admin_index'] + '/_doc',
            auth=(ELASTICSEARCH_USER, ELASTICSEARCH_PASSWORD),
            json=es_payload,
            verify=False
        )

        es_response.raise_for_status()
        with handler.stream as log_file:
            log_file.write(csv_line)
        return jsonify({"status": "Log saved successfully", "elasticsearch_response": es_response.json()}), 200
    except requests.exceptions.RequestException as e:
        print("RequestException:", e)
        return jsonify({"error": str(e)}), 500
    except Exception as e:
        print("Exception:", e)
        return jsonify({"error": str(e)}), 500

if __name__ == '__main__':
    if not os.path.exists(LOG_DIR):
        os.makedirs(LOG_DIR)
    app.run(host='0.0.0.0', port=8068)
