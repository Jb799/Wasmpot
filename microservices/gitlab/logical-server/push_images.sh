docker build -t wasmpot-logical-server:gitlab .;
docker tag wasmpot-logical-server:gitlab jb799/wasmpot-logical-server:gitlab;
docker push jb799/wasmpot-logical-server:gitlab;
