docker kill gl-wasi-container;
docker rm gl-wasi-container;
docker build -t wasmpot-logical-server:gitlab .;
docker run -d -p 8000:8000 -e WASI_PORT=8000 --network wp2_network --name gl-wasi-container wasmpot-logical-server:gitlab 8000 8888 gl-actix-container 8068 172.17.0.1;
clear;
docker logs -f gl-wasi-container;
