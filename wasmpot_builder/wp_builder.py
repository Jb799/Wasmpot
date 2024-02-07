import sys
import os
import requests
from bs4 import BeautifulSoup
from urllib.parse import urljoin, urlparse
import json
import re

saved_contents = []
saved_endpoints = []
web_port = ""

def save_content(wp_folder, url, content, extension):
    if len(content) == 0:
        return
    
    if len(web_port) != 0:
        content_str = content.decode('iso-8859-1')
        modified_content = content_str.replace('localhost', '{ADDR}').replace('127.0.0.1', '{ADDR}').replace(str(web_port), '{PORT}')
        content = modified_content.encode('iso-8859-1')

    saved_contents.append(url)
    url = url.split('?')[0]

    if url == '':
        relative_path = '/index'
    else:
        relative_path = os.path.relpath(url, wp_folder).replace("../", "")

    # Si c'est un fichier HTML, sauvegarder à la racine, sinon, reproduire la structure
    if extension.lower() == 'html':
        file_name = relative_path.split('/')[-1]
        file_path = os.path.join(wp_folder + '/html', file_name)

        if not os.path.exists(wp_folder + '/html'):
            os.makedirs(wp_folder + '/html')

        if file_name[-5:] != '.html':
            file_path += '.html'

        with open(file_path, 'wb') as file:
            file.write(content)
    else:
        # Divise le chemin relatif en parties
        path_parts = os.path.split(relative_path)

        # Crée les dossiers nécessaires pour reproduire la structure dans le dossier du projet
        folder_path = os.path.join(wp_folder + '/public', *path_parts[:-1])

        if not os.path.exists(folder_path):
            os.makedirs(folder_path)

        # Sauvegarde le contenu dans le fichier
        with open(os.path.join(folder_path, path_parts[-1].replace('/', '_')), 'wb') as file:
            file.write(content)

def save_endpoint(wp_folder, url, response, type):
    saved_endpoints.append(url)

    # Sauvegarde les informations dans un fichier JSON
    if url == '':
        json_path = os.path.join(wp_folder, 'index.json')
        url = '/'
        content_file_name = 'index.html'
    else:
        if url[-1] == '/':
            url = url[:-1]
        if url[0] == '/':
            url = url[1:]
        
        content_file_name = url.split('/')[-1].split('?')[0]

        if not '.' in content_file_name:
            content_file_name += '.html'
        elif content_file_name[-5:] != '.html':
            content_file_name = url.split('?')[0]

        json_url = url.split('?')[0]
        json_url = json_url.replace("/", "_")

        json_path = os.path.join(wp_folder, f'{json_url}.json')
    
    # Adapt Port and Address
    if len(web_port) != 0:
        for key, value in response.headers.items():
            new_value = value.replace('localhost', '{ADDR}').replace('127.0.0.1', '{ADDR}').replace(str(web_port), '{PORT}')
            response.headers[key] = new_value

    with open(json_path, 'w') as json_file:
        if response.content == b'':
            json.dump({
                'url': '/' + url,
                'type': type,
                'status_code': response.status_code,
                'headers': dict(response.headers),
            }, json_file, indent=2)
            return
        
        json.dump({
            'url': '/' + url,
            'content_file': 'public/' + content_file_name,
            'type': type,
            'status_code': response.status_code,
            'headers': dict(response.headers),
        }, json_file, indent=2)

def extract_links_from_js(wp_folder, js_content):
    # Extraire les liens depuis le contenu JavaScript
    js_links = re.findall(r'\b(?:https?|ftp)://\S+\b', js_content)
    for js_link in js_links:
        if urlparse(js_link).hostname in ['localhost', '127.0.0.1']:
            explore_url(wp_folder, js_link)

def extract_links_from_css(wp_folder, url, css_content):
    # Extraire les liens depuis le contenu CSS
    css_links = re.findall(r'url\s*\((.*?)\)', css_content)
    for css_link in css_links:
        # Nettoyer les liens (enlever les guillemets et les espaces)
        css_link = css_link.strip(' \'"')
        # Ignorer les liens de données
        if not css_link.startswith('data:'):
            absolute_url = urljoin(url, css_link)
            if urlparse(absolute_url).hostname in ['localhost', '127.0.0.1']:
                explore_url(wp_folder, absolute_url)

def explore_url(wp_folder, url):
    if urlparse(url).hostname not in ['localhost', '127.0.0.1']:
        return

    saved_url = url[21:] # Remove http://localhost:8000

    if saved_url in saved_endpoints:
        return
    
    print(f"Exploring {url}...")
    
    # Faire une requête GET
    response = requests.get(url, allow_redirects=False)  # Ne pas suivre automatiquement les redirections

    # Gérer les redirections
    if response.status_code >= 300 and response.status_code < 400 and 'Location' in response.headers:
        redirect_url = urljoin(url, response.headers['Location'])
        explore_url(wp_folder, redirect_url)

    # Sauvegarder la réponse
    save_endpoint(wp_folder, saved_url, response, 'GET')

    # Si le contenu est HTML, parser avec BeautifulSoup
    if 'text/html' in response.headers.get('Content-Type', ''):
        if not saved_url in saved_contents:
            save_content(wp_folder, saved_url, response.content, 'html')

        soup = BeautifulSoup(response.text, 'html.parser')

        # Trouver tous les liens dans la page
        links = soup.find_all(['a', 'link'], href=True)

        # Trouver tous les scripts dans la page
        scripts = soup.find_all('script', src=True)

        # Trouver toutes les images dans la page
        images = soup.find_all('img', src=True)

        for link in links:
            absolute_url = urljoin(url, link['href'])
            explore_url(wp_folder, absolute_url)

        for script in scripts:
            absolute_url = urljoin(url, script['src'])
            explore_url(wp_folder, absolute_url)

        for img in images:
            absolute_url = urljoin(url, img['src'])
            explore_url(wp_folder, absolute_url)
    
    # Si le contenu est un fichier JavaScript ou CSS, analyser les liens à l'intérieur
    elif response.headers.get('Content-Type') is not None and response.headers.get('Content-Type').startswith(('text/javascript', 'text/css')):
        if not saved_url in saved_contents:
            save_content(wp_folder, saved_url, response.content, 'txt')
        
        # Extraire les liens depuis le contenu du fichier
        if response.headers.get('Content-Type').startswith('text/javascript'):
            extract_links_from_js(wp_folder, response.text)
        elif response.headers.get('Content-Type').startswith('text/css'):
            extract_links_from_css(wp_folder, url, response.text)

    else:
        if not saved_url in saved_contents:
            save_content(wp_folder, saved_url, response.content, 'other')


def create_wp(wp_name):
    wp_folder = wp_name.replace(" ", "_").replace(".", "_").replace("/", "_").replace("\\", "_")

    if not os.path.exists(wp_folder):
        os.makedirs(wp_folder)
        print(f"Project [{wp_name}] created successfully.")
    else:
        print(f"Project [{wp_name}] already exists.")
        return -1

    return wp_folder

def main():
    global web_port

    args = sys.argv[1:]
    wp_name = ""
    wp_address = ""

    if len(args) < 2 or not isinstance(args[0], str) or not isinstance(args[1], str):
        print("Usage: wp_builder.py <honeypot name> <local address>")
        return -1
    
    wp_name = args[0]
    wp_address = args[1]
    web_port = (wp_address.split(":")[2])[:4]
    print(web_port)

    wp_folder = create_wp(wp_name)

    if wp_folder == -1:
        return -1
    
    print(f"Building [{wp_name}] on {wp_address}...")
    explore_url(wp_folder, wp_address)

    print(f"Project [{wp_name}] built successfully.")
    print(f"\n\n ## {len(saved_contents)} contents saved and {len(saved_endpoints)} endpoints saved. ##\n\n")

if __name__ == "__main__":
    main()
