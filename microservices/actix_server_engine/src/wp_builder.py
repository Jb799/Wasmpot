import sys
import os
import requests
from bs4 import BeautifulSoup
from urllib.parse import urljoin, urlparse
import json
import re
import gzip
import json
from urllib.parse import urljoin, urlparse

saved_contents = 0
saved_endpoints = []
web_port = ""
web_address = ""
projectName = ""
wp_folder = ""

def save_content(wp_folder, url, content, extension):
    if not content:
        return
    
    if web_port:
        pattern = re.compile(r'(localhost|127\.0\.0\.1):' + re.escape(str(web_port)))
        content = pattern.sub('{ADDR}:{PORT}', content.decode('iso-8859-1')).encode('iso-8859-1')

    url = url.split('?')[0]
    relative_path = 'index' if not url else url

    if url[-1] == '/':
        url = url[:-1]

    if "." not in url.split('/')[-1]:
        url += ".html"
        extension = "html"
    
    if extension.lower() == 'html':
        file_name = url.split('/')[-1]
        file_path = os.path.join(wp_folder, 'html', file_name)
        if not file_path.endswith('.html'):
            file_path += '.html'
    else:
        sanitized_path = relative_path.strip("/").replace("../", "")
        file_path = os.path.join(wp_folder, 'html', sanitized_path)
    
    os.makedirs(os.path.dirname(file_path), exist_ok=True)
    
    # Sauvegarde du contenu dans le fichier
    with open(file_path, 'wb') as file:
        file.write(content)

def save_endpoint(wp_folder, url, response, type):
    global saved_contents
    global saved_endpoints

    relative_path = os.path.relpath(url, wp_folder).replace("../", "")
    path_parts = os.path.split(relative_path)
    folder_path = os.path.join(wp_folder + '/html', *path_parts)

    if os.path.exists(folder_path):
        return False

    saved_endpoints.append(url)

    qparams = url.split('?')
    if len(qparams) > 1:
        url = qparams[0]
        qparams = qparams[1]
    else:
        qparams = ""

    # Sauvegarde les informations dans un fichier JSON
    if url == '/index' or url == 'index':
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

        json_url = url.replace("/", "_")

        json_path = os.path.join(wp_folder, f'{json_url}.json')
    
    # Adapt Port and Address
    if len(web_port) != 0:
        for key, value in response.headers.items():
            new_value = value.replace('localhost', '{ADDR}').replace('127.0.0.1', '{ADDR}').replace(str(web_port), '{PORT}')
            response.headers[key] = new_value

    with open(json_path, 'w') as json_file:
        if(url == "/"):
            url = ""

        if response.content == b'':
            json.dump({
                'url': '/' + url,
                'type': type,
                'query_params': qparams,
                'status_code': response.status_code,
                'headers': dict(response.headers),
            }, json_file, indent=2)
            return

        json.dump({
            'url': '/' + url,
            'content_file': 'html/' + content_file_name,
            'type': type,
            'query_params': qparams,
            'status_code': response.status_code,
            'headers': dict(response.headers),
        }, json_file, indent=2)

        saved_contents += 1

    return True

def extract_links_from_css(wp_folder, url, css_content):
    css_links = re.findall(r'url\s*\((.*?)\)', css_content)
    for css_link in css_links:
        css_link = css_link.strip(' \'"')
        if not css_link.startswith('data:'):
            absolute_url = urljoin(url, css_link)
            if urlparse(absolute_url).hostname in ['localhost', '127.0.0.1']:
                explore_url(wp_folder, absolute_url)

def extract_urls_from_json(wp_folder, url, json_content):
    # Parse the JSON content
    try:
        data = json.loads(json_content)
    except json.JSONDecodeError:
        return

    # Define a recursive function to walk through the JSON and extract URLs
    def extract_urls(obj):
        if isinstance(obj, dict):
            for value in obj.values():
                extract_urls(value)
        elif isinstance(obj, list):
            for item in obj:
                extract_urls(item)
        elif isinstance(obj, str) and obj.startswith('http'):
            absolute_url = urljoin(url, obj)
            if urlparse(absolute_url).hostname in ['localhost', '127.0.0.1']:
                explore_url(wp_folder, absolute_url)
                
    extract_urls(data)

def explore_url(wp_folder, url):
    if urlparse(url).hostname not in ['localhost', '127.0.0.1']:
        return
    
    pattern = re.compile(r'https?://[^/]+/?')
    saved_url = pattern.sub('', url)

    if not saved_url:
        saved_url = "index"

    if saved_url in saved_endpoints:
        return
    
    menu()
    print(f" ** Exploring {url}... **")
    
    # Faire une requête GET
    response = requests.get(url, allow_redirects=False)  # Ne pas suivre automatiquement les redirections

    # Gérer les redirections
    if response.status_code >= 300 and response.status_code < 400 and 'Location' in response.headers:
        redirect_url = urljoin(url, response.headers['Location'])

        explore_url(wp_folder, redirect_url)

    # Sauvegarder la réponse
    if not save_endpoint(wp_folder, saved_url, response, 'GET'):
        return -1

    # Si le contenu est HTML, parser avec BeautifulSoup
    if 'text/html' in response.headers.get('Content-Type', ''):
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
    elif response.headers.get('Content-Type') is not None and response.headers.get('Content-Type').startswith(('text/javascript', 'text/css', 'application/javascript', 'application/x-javascript', 'application/json')):
        save_content(wp_folder, saved_url, response.content, 'txt')
        
        # Extraire les liens depuis le contenu du fichier
        if response.headers.get('Content-Type').startswith('text/css'):
            extract_links_from_css(wp_folder, url, response.text)
        elif response.headers.get('Content-Type').startswith('application/json'):
            extract_urls_from_json(wp_folder, url, response.text)

    else:
        save_content(wp_folder, saved_url, response.content, 'other')


def create_wp(wp_name):
    global wp_folder

    wp_folder = wp_name.replace(" ", "_").replace(".", "_").replace("/", "_").replace("\\", "_")

    if not os.path.exists(wp_folder):
        os.makedirs(wp_folder)
    else:
        print(f"Project [{wp_name}] already exists, add contents...")

    return wp_folder

def menu():
    os.system('cls' if os.name == 'nt' else 'clear')
    print("-------------------------------------")
    print("|        WP2 HoneyPot Builder       |")
    print("-------------------------------------")

    if projectName != "":
        print(f"|   ## Project: {projectName} ##")

    if web_address != "":
        print(f"|   ## LastEndpoint: {web_address} ##")

    print(f"|   ## {saved_contents} contents & {len(saved_endpoints)} endpoints ##")

    print("\n")


def selectName():
    global projectName

    menu()
    print("Enter your project name:")
    projectName = input("  >> ")

    wp_folder = create_wp(projectName)

    if wp_folder == -1:
        return -1

    return 0


def selectEndpoint():
    global web_address
    global web_port
    global wp_folder

    menu()
    print("Enter a local server endpoint or (quit):")
    web_address = input("  >> ")

    if web_address == "quit":
        return 0

    if not "http://" in web_address[:7]:
        web_address = "http://" + web_address

    web_port = (web_address.split(":")[2])[:4]

    explore_url(wp_folder, web_address)

    selectEndpoint()


def main():
    if selectName() == 0:
        selectEndpoint()
    return 0

if __name__ == "__main__":
    main()
