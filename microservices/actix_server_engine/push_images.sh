docker build -t wasmpot-resource-server .;
docker tag wasmpot-resource-server:latest jb799/wasmpot-resource-server:keycloak;
docker push jb799/wasmpot-resource-server:keycloak;
