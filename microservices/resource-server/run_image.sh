docker kill actix-container;
docker rm actix-container;
docker build -t wasmpot-resource-server .;
docker run -d -p 8888:8888 -e ACTIX_PORT=8888 -e WASI_PORT=8000 -e WASI_ADDR=localhost --network wp2_network --name actix-container wasmpot-resource-server;
clear;
docker logs -f actix-container;
