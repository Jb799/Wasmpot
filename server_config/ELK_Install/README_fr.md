# Guide d'installation et de configuration d'Elasticsearch, Kibana et Logstash sur Kubernetes

Ce guide fournit des instructions pour déployer et configurer Elasticsearch, Kibana et Logstash dans un cluster Kubernetes.

## Méthode 1 : Utilisation de fichiers YAML

### Déploiement d'Elasticsearch dans le namespace `elastic-system`

1. Appliquez la configuration suivante pour déployer Elasticsearch :

    ```sh
    cat <<EOF | kubectl apply -f -
    apiVersion: elasticsearch.k8s.elastic.co/v1
    kind: Elasticsearch
    metadata:
      name: quickstart
      namespace: elastic-system
    spec:
      version: 8.13.2
      nodeSets:
      - name: default
        count: 1
        config:
          node.store.allow_mmap: false
    EOF
    ```

2. Vérifiez que le déploiement s'est bien passé :

    ```sh
    kubectl get elasticsearch -n elastic-system
    ```

    La sortie devrait ressembler à ceci :

    ```
    NAME         HEALTH   NODES   VERSION   PHASE   AGE
    quickstart   green    1       8.13.2    Ready   116s
    ```

    ```sh
    kubectl get pods -n elastic-system --selector='elasticsearch.k8s.elastic.co/cluster-name=quickstart'
    ```

    La sortie devrait ressembler à ceci :

    ```
    NAME                      READY   STATUS    RESTARTS   AGE
    quickstart-es-default-0   1/1     Running   0          4m10s
    ```

3. Vérifiez les logs en spécifiant le namespace :

    ```sh
    kubectl logs -f quickstart-es-default-0 -n elastic-system
    ```

4. Vérifiez le service Elasticsearch créé :

    ```sh
    kubectl get service quickstart-es-http -n elastic-system
    ```

    La sortie devrait ressembler à ceci :

    ```
    NAME                 TYPE        CLUSTER-IP     EXTERNAL-IP   PORT(S)    AGE
    quickstart-es-http   ClusterIP   10.43.86.180   <none>        9200/TCP   9m48s
    ```

5. Récupérez le mot de passe :

    ```sh
    echo "Récupération du mot de passe..."
    PASSWORD=$(kubectl get secret quickstart-es-elastic-user -n elastic-system -o go-template='{{.data.elastic | base64decode}}')
    echo "Mot de passe récupéré avec succès : $PASSWORD"
    ```

### Optionnel : Vérification du service via une page web

1. Modifiez le type de service pour `LoadBalancer` :

    ```sh
    kubectl patch service quickstart-es-http -n elastic-system --type='json' -p '[{"op":"replace","path":"/spec/type","value":"LoadBalancer"}]'
    ```

    La sortie devrait ressembler à ceci :

    ```
    NAME                 TYPE           CLUSTER-IP     EXTERNAL-IP   PORT(S)          AGE
    quickstart-es-http   LoadBalancer   10.43.86.180   10.10.50.83   9200:31343/TCP   16m
    ```

2. Résultat sur la page (accédez à `http://<EXTERNAL-IP>:9200`):

    ```json
    {
      "name" : "quickstart-es-default-0",
      "cluster_name" : "quickstart",
      "cluster_uuid" : "HjlyysXLSpyhdzs8Au2Lag",
      "version" : {
        "number" : "8.13.2",
        "build_flavor" : "default",
        "build_type" : "docker",
        "build_hash" : "16cc90cd2d08a3147ce02b07e50894bc060a4cbf",
        "build_date" : "2024-04-05T14:45:26.420424304Z",
        "build_snapshot" : false,
        "lucene_version" : "9.10.0",
        "minimum_wire_compatibility_version" : "7.17.0",
        "minimum_index_compatibility_version" : "7.0.0"
      },
      "tagline" : "You Know, for Search"
    }
    ```

### Déploiement de Kibana dans le namespace `elastic-system`

1. Appliquez la configuration suivante pour déployer Kibana :

    ```sh
    cat <<EOF | kubectl apply -f -
    apiVersion: kibana.k8s.elastic.co/v1
    kind: Kibana
    metadata:
      name: quickstart
      namespace: elastic-system
    spec:
      version: 8.13.2
      count: 1
      elasticsearchRef:
        name: quickstart
    EOF
    ```

2. Vérifiez que le déploiement s'est bien passé :

    ```sh
    kubectl get kibana -n elastic-system
    ```

    La sortie devrait ressembler à ceci :

    ```
    NAME         HEALTH   NODES   VERSION   AGE
    quickstart   green    1       8.13.2    3m17s
    ```

3. Récupérez le mot de passe Elasticsearch :

    ```sh
    kubectl get secret quickstart-es-elastic-user -n elastic-system -o=jsonpath='{.data.elastic}' | base64 --decode; echo
    ```

    Le mot de passe devrait être affiché.

### Déploiement de Logstash dans le namespace `elastic-system`

1. Appliquez la configuration suivante pour déployer Logstash :

    ```sh
    cat <<'EOF' | kubectl apply -f -
    apiVersion: logstash.k8s.elastic.co/v1alpha1
    kind: Logstash
    metadata:
      name: quickstart
      namespace: elastic-system
    spec:
      count: 1
      elasticsearchRefs:
        - name: quickstart
          clusterName: qs
      version: 8.13.2
      pipelines:
        - pipeline.id: main
          config.string: |
            input {
              beats {
                port => 5044
              }
            }
            output {
              elasticsearch {
                hosts => [ "${QS_ES_HOSTS}" ]
                user => "${QS_ES_USER}"
                password => "${QS_ES_PASSWORD}"
                ssl_certificate_authorities => "${QS_ES_SSL_CERTIFICATE_AUTHORITY}"
              }
            }
      services:
        - name: beats
          service:
            spec:
              type: NodePort
              ports:
                - port: 5044
                  name: "filebeat"
                  protocol: TCP
                  targetPort: 5044
    EOF
    ```

2. Vérifiez la configuration Logstash :

    ```sh
    kubectl exec -n elastic-system quickstart-ls-0 -- cat /usr/share/logstash/pipeline/logstash.conf
    ```

    La sortie devrait ressembler à ceci :

    ```
    input {
      beats {
        port => 5044
      }
    }

    output {
      stdout {
        codec => rubydebug
      }
    }
    ```

### Configuration de Logstash

1. Modification de `/usr/share/logstash/pipeline/logstash.conf` pour envoyer les logs à Elasticsearch :

    ```sh
    kubectl exec -n elastic-system quickstart-ls-0 -- vi /usr/share/logstash/pipeline/logstash.conf
    ```

    Remplacez le contenu par :

    ```conf
    input {
      beats {
        port => 5044
      }
    }

    output {
      elasticsearch {
        hosts => ["https://quickstart-es-http.elastic-system.svc:9200"]
        user => "elastic"
        password => "votre_mot_de_passe_elasticsearch"
      }
    }
    ```

2. Vérifiez les indices Elasticsearch :

    ```sh
    kubectl exec -n elastic-system quickstart-es-default-0 -- curl -k --user elastic:votre_mot_de_passe_elasticsearch -X GET "https://localhost:9200/_cat/indices?v"
    ```

    La sortie devrait ressembler à ceci :

    ```
    health status index                                                              uuid                   pri rep docs.count docs.deleted store.size pri.store.size dataset.size
    green  open   .internal.alerts-transform.health.alerts-default-000001            qCsT3pSaTReuS5dIdLcRTA   1   0          0            0       249b           249b         249b
    green  open   .internal.alerts-observability.logs.alerts-default-000001          bzaQkIpLQmqtJPhoMY8_Mw   1   0          0            0       249b           249b         249b
    ...
    ```

### Configuration de sécurité

Pour configurer des paramètres de sécurité supplémentaires pour Kibana, veuillez consulter la documentation officielle de [Secure Settings](https://www.elastic.co/guide/en/cloud-on-k8s/master/k8s-kibana-secure-settings.html).

## Méthode 2 : Utilisation de Helm

### Installation de Helm

Si Helm n'est pas déjà installé, installez-le en suivant les instructions de la [documentation officielle de Helm](https://helm.sh/docs/intro/install/).

### Ajout des repositories Helm

1. Ajoutez le repository Elastic Helm Charts :

    ```sh
    helm repo add elastic https://helm.elastic.co
    helm repo update
    ```

### Déploiement d'Elasticsearch

1. Créez un namespace pour Elasticsearch :

    ```sh
    kubectl create namespace elastic-system
    ```

2. Déployez Elasticsearch :

    ```sh
    helm install elasticsearch elastic/elasticsearch --namespace elastic-system --version 8.13.2
    ```

3. Vérifiez que le déploiement s'est bien passé :

    ```sh
    kubectl get pods -n elastic-system -l app=elasticsearch
    ```

    La sortie devrait ressembler à ceci :

    ```
    NAME                       READY   STATUS    RESTARTS   AGE
    elasticsearch-master-0     1/1     Running   0          2m
    elasticsearch-master-1     1/1     Running   0          2m
    elasticsearch-master-2     1/1     Running   0          2m
    ```

4. Récupérez le mot de passe :

    ```sh
    echo "Récupération du mot de passe..."
    PASSWORD=$(kubectl get secret elasticsearch-master-0 -n elastic-system -o go-template='{{.data.password | base64decode}}')
    echo "Mot de passe récupéré avec succès : $PASSWORD"
    ```

### Déploiement de Kibana

1. Déployez Kibana :

    ```sh
    helm install kibana elastic/kibana --namespace elastic-system --version 8.13.2
    ```

2. Vérifiez que le déploiement s'est bien passé :

    ```sh
    kubectl get pods -n elastic-system -l app=kibana
    ```

    La sortie devrait ressembler à ceci :

    ```
    NAME                       READY   STATUS    RESTARTS   AGE
    kibana-5f7b84c79f-z7mrl    1/1     Running   0          2m
    ```

### Déploiement de Logstash

1. Déployez Logstash :

    ```sh
    helm install logstash elastic/logstash --namespace elastic-system --version 8.13.2
    ```

2. Vérifiez que le déploiement s'est bien passé :

    ```sh
    kubectl get pods -n elastic-system -l app=logstash
    ```

    La sortie devrait ressembler à ceci :

    ```
    NAME                        READY   STATUS    RESTARTS   AGE
    logstash-0                  1/1     Running   0          2m
    ```

### Configuration de Logstash

1. Modifiez la configuration Logstash pour envoyer les logs à Elasticsearch :

    ```sh
    kubectl exec -n elastic-system logstash-0 -- vi /usr/share/logstash/pipeline/logstash.conf
    ```

    Remplacez le contenu par :

    ```conf
    input {
      beats {
        port => 5044
      }
    }

    output {
      elasticsearch {
        hosts => ["https://elasticsearch-master.elastic-system.svc:9200"]
        user => "elastic"
        password => "votre_mot_de_passe_elasticsearch"
      }
    }
    ```

2. Vérifiez les indices Elasticsearch :

    ```sh
    kubectl exec -n elastic-system elasticsearch-master-0 -- curl -k --user elastic:votre_mot_de_passe_elasticsearch -X GET "https://localhost:9200/_cat/indices?v"
    ```

    La sortie devrait ressembler à ceci :

    ```
    health status index                                                              uuid                   pri rep docs.count docs.deleted store.size pri.store.size dataset.size
    green  open   .internal.alerts-transform.health.alerts-default-000001            qCsT3pSaTReuS5dIdLcRTA   1   0          0            0       249b           249b         249b
    green  open   .internal.alerts-observability.logs.alerts-default-000001          bzaQkIpLQmqtJPhoMY8_Mw   1   0          0            0       249b           249b         249b
    ...
    ```

<img width="1467" alt="Capture d’écran 2024-06-04 à 21 56 02" src="https://github.com/Jb799/Wasmpot/assets/62985330/0c1509fc-de71-4acc-aeb5-725329e13738">
