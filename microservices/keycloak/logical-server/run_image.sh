docker kill wasi-container;
docker rm wasi-container;
docker build -t wasmpot-logical-server .;
docker run -d -p 8000:8000 -e WASI_PORT=8000 --network wp2_network --name wasi-container wasmpot-logical-server 8000 8888 actix-container 8068 172.17.0.1;
clear;
docker logs -f wasi-container;
