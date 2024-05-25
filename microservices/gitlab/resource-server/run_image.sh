docker kill gl-actix-container;
docker rm gl-actix-container;
docker build -t wasmpot-resource-server:gitlab .;
docker run -d -p 8888:8888 -e ACTIX_PORT=8888 -e WASI_PORT=8000 -e WASI_ADDR=localhost --network wp2_network --name gl-actix-container wasmpot-resource-server:gitlab;
clear;
docker logs -f gl-actix-container;
