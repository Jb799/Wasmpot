# Guide for Installing and Configuring Elasticsearch, Kibana, and Logstash on Kubernetes

This guide provides instructions for deploying and configuring Elasticsearch, Kibana, and Logstash in a Kubernetes cluster.

## Method 1: Using YAML Files

### Deploying Elasticsearch in the `elastic-system` Namespace

1. Apply the following configuration to deploy Elasticsearch:

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

2. Verify the deployment:

    ```sh
    kubectl get elasticsearch -n elastic-system
    ```

    The output should be:

    ```
    NAME         HEALTH   NODES   VERSION   PHASE   AGE
    quickstart   green    1       8.13.2    Ready   116s
    ```

    ```sh
    kubectl get pods -n elastic-system --selector='elasticsearch.k8s.elastic.co/cluster-name=quickstart'
    ```

    The output should be:

    ```
    NAME                      READY   STATUS    RESTARTS   AGE
    quickstart-es-default-0   1/1     Running   0          4m10s
    ```

3. Check the logs specifying the namespace:

    ```sh
    kubectl logs -f quickstart-es-default-0 -n elastic-system
    ```

4. Verify the created Elasticsearch service:

    ```sh
    kubectl get service quickstart-es-http -n elastic-system
    ```

    The output should be:

    ```
    NAME                 TYPE        CLUSTER-IP     EXTERNAL-IP   PORT(S)    AGE
    quickstart-es-http   ClusterIP   10.43.86.180   <none>        9200/TCP   9m48s
    ```

5. Retrieve the password:

    ```sh
    echo "Retrieving password..."
    PASSWORD=$(kubectl get secret quickstart-es-elastic-user -n elastic-system -o go-template='{{.data.elastic | base64decode}}')
    echo "Password retrieved successfully: $PASSWORD"
    ```

### Optional: Verifying the Service with a Web Page

1. Modify the service type to `LoadBalancer`:

    ```sh
    kubectl patch service quickstart-es-http -n elastic-system --type='json' -p '[{"op":"replace","path":"/spec/type","value":"LoadBalancer"}]'
    ```

    The output should be:

    ```
    NAME                 TYPE           CLUSTER-IP     EXTERNAL-IP   PORT(S)          AGE
    quickstart-es-http   LoadBalancer   10.43.86.180   10.10.50.83   9200:31343/TCP   16m
    ```

2. Result on the page (access `http://<EXTERNAL-IP>:9200`):

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

### Deploying Kibana in the `elastic-system` Namespace

1. Apply the following configuration to deploy Kibana:

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

2. Verify the deployment:

    ```sh
    kubectl get kibana -n elastic-system
    ```

    The output should be:

    ```
    NAME         HEALTH   NODES   VERSION   AGE
    quickstart   green    1       8.13.2    3m17s
    ```

3. Retrieve the Elasticsearch password:

    ```sh
    kubectl get secret quickstart-es-elastic-user -n elastic-system -o=jsonpath='{.data.elastic}' | base64 --decode; echo
    ```

    The password should be displayed.

### Deploying Logstash in the `elastic-system` Namespace

1. Apply the following configuration to deploy Logstash:

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

2. Verify the Logstash configuration:

    ```sh
    kubectl exec -n elastic-system quickstart-ls-0 -- cat /usr/share/logstash/pipeline/logstash.conf
    ```

    The output should be:

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

### Configuring Logstash

1. Modify `/usr/share/logstash/pipeline/logstash.conf` to send logs to Elasticsearch:

    ```sh
    kubectl exec -n elastic-system quickstart-ls-0 -- vi /usr/share/logstash/pipeline/logstash.conf
    ```

    Replace the content with:

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
        password => "your_elasticsearch_password"
      }
    }
    ```

2. Verify the Elasticsearch indices:

    ```sh
    kubectl exec -n elastic-system quickstart-es-default-0 -- curl -k --user elastic:your_elasticsearch_password -X GET "https://localhost:9200/_cat/indices?v"
    ```

    The output should be:

    ```
    health status index                                                              uuid                   pri rep docs.count docs.deleted store.size pri.store.size dataset.size
    green  open   .internal.alerts-transform.health.alerts-default-000001            qCsT3pSaTReuS5dIdLcRTA   1   0          0            0       249b           249b         249b
    green  open   .internal.alerts-observability.logs.alerts-default-000001          bzaQkIpLQmqtJPhoMY8_Mw   1   0          0            0       249b           249b         249b
    ...
    ```

### Security Configuration

For additional security settings for Kibana, refer to the official [Secure Settings documentation](https://www.elastic.co/guide/en/cloud-on-k8s/master/k8s-kibana-secure-settings.html).

## Method 2: Using Helm

### Installing Helm

If Helm is not already installed, follow the instructions in the [official Helm documentation](https://helm.sh/docs/intro/install/).

### Adding Helm Repositories

1. Add the Elastic Helm Charts repository:

    ```sh
    helm repo add elastic https://helm.elastic.co
    helm repo update
    ```

### Deploying Elasticsearch

1. Create a namespace for Elasticsearch:

    ```sh
    kubectl create namespace elastic-system
    ```

2. Deploy Elasticsearch:

    ```sh
    helm install elasticsearch elastic/elasticsearch --namespace elastic-system --version 8.13.2
    ```

3. Verify the deployment:

    ```sh
    kubectl get pods -n elastic-system -l app=elasticsearch
    ```

    The output should be:

    ```
    NAME                       READY   STATUS    RESTARTS   AGE
    elasticsearch-master-0     1/1     Running   0          2m
    elasticsearch-master-1     1/1     Running   0          2m
    elasticsearch-master-2     1/1     Running   0          2m
    ```

4. Retrieve the password:

    ```sh
    echo "Retrieving password..."
    PASSWORD=$(kubectl get secret elasticsearch-master-0 -n elastic-system -o go-template='{{.data.password | base64decode}}')
    echo "Password retrieved successfully: $PASSWORD"
    ```

### Deploying Kibana

1. Deploy Kibana:

    ```sh
    helm install kibana elastic/kibana --namespace elastic-system --version 8.13.2
    ```

2. Verify the deployment:

    ```sh
    kubectl get pods -n elastic-system -l app=kibana
    ```

    The output should be:

    ```
    NAME                       READY   STATUS    RESTARTS   AGE
    kibana-5f7b84c79f-z7mrl    1/1     Running   0          2m
    ```

### Deploying Logstash

1. Deploy Logstash:

    ```sh
    helm install logstash elastic/logstash --namespace elastic-system --version 8.13.2
    ```

2. Verify the deployment:

    ```sh
    kubectl get pods -n elastic-system -l app=logstash
    ```

    The output should be:

    ```
    NAME                        READY   STATUS    RESTARTS   AGE
    logstash-0                  1/1     Running   0          2m
    ```

### Configuring Logstash

1. Modify the Logstash configuration to send logs to Elasticsearch:

    ```sh
    kubectl exec -n elastic-system logstash-0 -- vi /usr/share/logstash/pipeline/logstash.conf
    ```

    Replace the content with:

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
        password => "your_elasticsearch_password"
      }
    }
    ```

2. Verify the Elasticsearch indices:

    ```sh
    kubectl exec -n elastic-system elasticsearch-master-0 -- curl -k --user elastic:your_elasticsearch_password -X GET "https://localhost:9200/_cat/indices?v"
    ```

    The output should be:

    ```
    health status index                                                              uuid                   pri rep docs.count docs.deleted store.size pri.store.size dataset.size
    green  open   .internal.alerts-transform.health.alerts-default-000001            qCsT3pSaTReuS5dIdLcRTA   1   0          0            0       249b           249b         249b
    green  open   .internal.alerts-observability.logs.alerts-default-000001          bzaQkIpLQmqtJPhoMY8_Mw   1   0          0            0       249b           249b         249b
    ...
    ```
