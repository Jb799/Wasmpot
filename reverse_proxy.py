import http.server
import http.client

# SERVERS mapping:
SERVERS = {
    'keycloak.authgates.com': {'ip': '10.10.50.85', 'port': 8000},
    'gitlab.authgates.com': {'ip': '10.10.50.87', 'port': 8002},
}

class ProxyHandler(http.server.BaseHTTPRequestHandler):
    def handle_request(self, method):
        host = self.headers.get('Host')
        if host in SERVERS:
            target = SERVERS[host]
            conn = http.client.HTTPConnection(target['ip'], target['port'])
            
            # Read the content length for body data forwarding
            content_length = self.headers.get('Content-Length')
            if content_length:
                body = self.rfile.read(int(content_length))
            else:
                body = None

            # Forward the request to the target server
            conn.request(method, self.path, body=body, headers=self.headers)

            # Receive the response from the target server
            response = conn.getresponse()
            
            # Send the response status and headers back to the client
            self.send_response(response.status)
            for header, value in response.getheaders():
                self.send_header(header, value)
            self.end_headers()
            
            # Send the response body back to the client
            self.wfile.write(response.read())
            conn.close()
        else:
            # Handle unknown host
            self.send_error(404, "Unknown host")

    def do_GET(self):
        self.handle_request('GET')

    def do_POST(self):
        self.handle_request('POST')

    def do_PUT(self):
        self.handle_request('PUT')

    def do_DELETE(self):
        self.handle_request('DELETE')

    def do_HEAD(self):
        self.handle_request('HEAD')

if __name__ == '__main__':
    server_port = 8000
    server_address = ('0.0.0.0', server_port)
    httpd = http.server.HTTPServer(server_address, ProxyHandler)
    print(f'Reverse proxy server running on port {server_port}...')
    httpd.serve_forever()