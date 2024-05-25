docker build -t wasmpot-logical-server .;
docker tag wasmpot-logical-server:latest jb799/wasmpot-logical-server:keycloak;
docker push jb799/wasmpot-logical-server:keycloak;
