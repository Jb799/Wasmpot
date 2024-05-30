import os
import requests
import csv
import sys
from user_agents import parse as parse_user_agent

# Paramètres de connexion à Elasticsearch/Kibana
ELASTICSEARCH_URL = 'https://10.10.50.83:9200/wp2/_doc'
ELASTICSEARCH_USER = 'elastic'
ELASTICSEARCH_PASSWORD = 'dWDwSP38Yrn3RC0u'

ip_infos = {}

def get_ip_infos(ip_address):
    if ip_address in ip_infos:
        return ip_infos[ip_address]
    else:
        ip_api_infos = request_ip_api(ip_address)
        
        if ip_api_infos is not None:
            ip_infos[ip_address] = ip_api_infos
            return ip_api_infos
        return None

def request_ip_api(ip_address):
    response = requests.get(f'http://ip-api.com/json/{ip_address}')
    response.raise_for_status()
    
    if response.status_code == 200:
        return response.json()

def read_and_send_logs(file_path):
    if not os.path.exists(file_path):
        print(f"Le fichier {file_path} n'existe pas.")
        return

    with open(file_path, 'r') as file:
        reader = csv.reader(file)
        next(reader)
        try:
            for row in reader:
                if len(row) <= 10:
                    log_data = {
                        "@timestamp": row[0] if len(row) > 0 else "",
                        "timestamp": row[0] if len(row) > 0 else "",
                        "ID": row[1] if len(row) > 1 else "",
                        "FLAG": row[2] if len(row) > 2 else "",
                        "Method": row[3] if len(row) > 3 else "",
                        "Endpoint": row[4] if len(row) > 4 else "",
                        "Payload": row[5] if len(row) > 5 else "",
                        "User-Agent": "",
                        "OS": "",
                        "IP": row[6] if len(row) > 6 else "",
                        "Isp": "",
                        "Country": row[7] if len(row) > 7 else "",
                        "City": "",
                        "Latitude": row[8] if len(row) > 8 else "",
                        "Longitude": row[9] if len(row) > 9 else "",
                        "username": "",
                        "password": "",
                        "location": {
                            "lat": float(row[8]) if len(row) > 8 else None,
                            "lon": float(row[9]) if len(row) > 9 else None,
                        }
                    }
                elif len(row) <= 13:
                    log_data = {
                        "@timestamp": row[0] if len(row) > 0 else "",
                        "timestamp": row[0] if len(row) > 0 else "",
                        "ID": row[1] if len(row) > 1 else "",
                        "FLAG": row[2] if len(row) > 2 else "",
                        "Method": row[3] if len(row) > 3 else "",
                        "Endpoint": row[4] if len(row) > 4 else "",
                        "Payload": row[5] if len(row) > 5 else "",
                        "User-Agent": row[6] if len(row) > 6 else "",
                        "OS": "",
                        "IP": row[7] if len(row) > 7 else "",
                        "Isp": row[8] if len(row) > 8 else "",
                        "Country": row[9] if len(row) > 9 else "",
                        "City": row[10] if len(row) > 10 else "",
                        "Latitude": row[11] if len(row) > 11 else "",
                        "Longitude": row[12] if len(row) > 12 else "",
                        "username": "",
                        "password": "",
                        "location": {
                            "lat": float(row[11]) if len(row) > 11 else None,
                            "lon": float(row[12]) if len(row) > 12 else None,
                        }
                    }
                elif len(row) <= 14:
                    log_data = {
                        "@timestamp": row[0] if len(row) > 0 else "",
                        "timestamp": row[0] if len(row) > 0 else "",
                        "ID": row[1] if len(row) > 1 else "",
                        "FLAG": row[2] if len(row) > 2 else "",
                        "Method": row[3] if len(row) > 3 else "",
                        "Endpoint": row[4] if len(row) > 4 else "",
                        "Payload": row[5] if len(row) > 5 else "",
                        "User-Agent": row[6] if len(row) > 6 else "",
                        "OS": row[7] if len(row) > 7 else "",
                        "IP": row[8] if len(row) > 8 else "",
                        "Isp": row[9] if len(row) > 9 else "",
                        "Country": row[10] if len(row) > 10 else "",
                        "City": row[11] if len(row) > 11 else "",
                        "Latitude": row[12] if len(row) > 12 else "",
                        "Longitude": row[13] if len(row) > 13 else "",
                        "username": "",
                        "password": "",
                        "location": {
                            "lat": float(row[12]) if len(row) > 12 else None,
                            "lon": float(row[13]) if len(row) > 13 else None,
                        }
                    }
                else:
                    continue
                
                try:
                    if 'username=' in log_data["Payload"]:
                        payload = log_data["Payload"]
                        log_data["username"] = payload.split('username=')[1]
                        if '&' in log_data["username"]:
                            log_data["username"] = log_data["username"].split('&')[0]
                except:
                    pass
                
                try:
                    if 'password=' in log_data["Payload"]:
                        payload = log_data["Payload"]
                        log_data["password"] = payload.split('password=')[1]
                        if '&' in log_data["password"]:
                            log_data["password"] = log_data["password"].split('&')[0]
                except:
                    pass
                
                if log_data['User-Agent'] != "Unknown User-Agent" and log_data['User-Agent'] != "" and log_data['OS'] == "" :
                    user_agent = parse_user_agent(log_data['User-Agent'])
                    log_data['OS'] = user_agent.os.family
                
                if log_data['IP'] != "Unknown IP" and log_data['IP'] != "" and (( log_data['Latitude'] == 0 and log_data['Longitude'] == 0 ) or log_data['City'] == "" or log_data['City'] == "Unknown City" or log_data['Isp'] == "" or log_data['Isp'] == "Unknown Isp" or log_data['Country'] == "" or log_data['Country'] == "Unknown Country" ):
                    ip_infos = get_ip_infos(log_data['IP'])
                    
                    if ip_infos is not None and ( 'status' in ip_infos and ip_infos['status'] == 'success' ):
                        log_data['City'] = ip_infos['city']
                        log_data['Isp'] = ip_infos['isp']
                        log_data['Country'] = ip_infos['country']
                        log_data['Latitude'] = ip_infos['lat']
                        log_data['Longitude'] = ip_infos['lon']
                        log_data['location']['lat'] = ip_infos['lat']
                        log_data['location']['lon'] = ip_infos['lon']

                print(log_data)

                try:
                    response = requests.post(
                        ELASTICSEARCH_URL,
                        auth=(ELASTICSEARCH_USER, ELASTICSEARCH_PASSWORD),
                        json=log_data,
                        verify=False
                    )
                    response.raise_for_status()
                    print("1 log envoyé avec succès !")
                except requests.exceptions.RequestException as e:
                    print("Erreur lors de l'envoi du log :", e)
        except:
            pass

if __name__ == '__main__':
    if len(sys.argv) > 1:
        read_and_send_logs(sys.argv[1])
    else:
        print("Aucun argument passé.")