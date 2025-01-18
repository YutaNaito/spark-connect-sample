# spark-connect-sample

## version
- Apache Spark 4.0.0-preview1
- Python3 3.12

## Run

### Spark Connect Server
- Start Spark Connect Server (default: standalone mode)
```shell
$ start-connect-server.sh \
  --packages org.apache.spark:spark-connect_2.13:4.0.0-preview1
```

- Start Spark Connect Server using loccal cluster
```shell
$ start-connect-server.sh \
  --packages org.apache.spark:spark-connect_2.13:4.0.0-preview1 \
  --master local-cluster[2,2,2042]
```

### python client
```shell
$ python3 spark-connect.py
```

