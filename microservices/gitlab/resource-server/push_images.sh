docker build -t wasmpot-resource-server:gitlab .;
docker tag wasmpot-resource-server:gitlab jb799/wasmpot-resource-server:gitlab;
docker push jb799/wasmpot-resource-server:gitlab;
