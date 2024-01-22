WEB FRAMEWORK BENCHMARK
=======================

The objective of this benchmark is to compare the performance of several web frameworks with a minimal API approach (just for fun !)

Application :
-------------

- [x] GET / respond with json {"name": "Benchmark.MyStack"}
- [x] GET /api/user respond an object User serialized in json {"id": "uuid V4", "firstname": "toto", "lastname": "titi", "email": "toto@titi.com"}
- [ ] GET /api/articles respond a list of 50 articles stored in database and json serialized

Requirements :
--------------

* Docker

Setup : 
-------

MacBook Pro 
CPU : Apple M1 Max
RAM: 64 Go

Benchmark tool : 
----------------

WRK (https://github.com/wg/wrk)

Build & run containers : 
------------------------

```console
    cd src
    docker-compose up
```

RESULTS WITH CONTAINER :
======================

* 1 : RUST - actix        (27 739.79 requests/sec)
* 2 : GOLANG - gin        (25 793.99 requests/sec)
* 3 : DOTNET - asp net    (24 957.23 requests/sec)
* 5 : JAVA - quarkus      (18 311.27 requests/sec)
* 6 : NODE - express      (6 881.13  requests/sec)
* 8 : PYTHON - fastapi    (2 244.22  requests/sec)
* 7 : RUBY - sinatra      (1 097.36  requests/sec)
* 9 : PHP - slim          (565.35    requests/sec)

RUST (1.75 - actix 4)
====================
    cargo build --release
    ./target/release/benchmark-rust
    wrk -t8 -c100 -d10s http://127.0.0.1:5700/api/user

GOLANG (1.21.6 - Gin 1.9.1):
==========================
    go build -ldflags "-s -w" main.go
    ./main
    wrk -t8 -c100 -d10s http://127.0.0.1:5500/api/user

DOTNET (8.0.100):
=================

**Local run**
```console
    cd src/Benchmark.Dotnet
    dotnet run
```

**Run in release mode**
```console
    cd src/Benchmark.Dotnet
    dotnet publish -c Release
    ./bin/Release/Benchmark.Dotnet
```

**Build container**
```console
    cd src/Benchmark.Dotnet
    dotnet publish -r linux-arm64 -p:PublishProfile=DefaultContainer
```

**Benchmark**
```console
    wrk -t8 -c100 -d10s http://127.0.0.1:5100/api/user
```

JAVA (OpenJDK 21 - Quarkus 3.6.5) :
===================================

**Run**
```console
    cd src/Benchmark.Quarkus
    quarkus build
    java -jar target/quarkus-app/quarkus-run.jar
```

**build container**
```console
    cd src/Benchmark.Quarkus
    docker build -f src/main/docker/Dockerfile.jvm -t benchmark/java-service .
```

**Benchmark**
```console
    wrk -t8 -c100 -d10s http://127.0.0.1:5400/api/user
```

NODE (20.11 - express 4):
=========================

**Run**
```console
    cd src/Benchmark.Node
    node index.js
```
    
**Benchmark**
```console
    wrk -t8 -c100 -d10s http://127.0.0.1:5200/api/user
```

PYTHON (3.11 - FastAPI):
========================

**Local run**
```console
    python3 app.py
```

**Benchmark**
```console
    wrk -t8 -c100 -d10s http://127.0.0.1:5300/api/user
```

RUBY (3.3 - sinatra 2.2)
========================

**Local run**
```console
    ruby app.rb
```

**Benchmark**
```console
    wrk -t8 -c100 -d10s http://127.0.0.1:5500/api/user
```

PHP (8.3.1 - slim 4)
==========

**Local run**
```console
    php -S localhost:8080 -t public public/index.php
```

**Benchmark**
```console
    wrk -t8 -c100 -d10s http://127.0.0.1:5800/api/user
```



## NEXT STEP

* container app for all
* mount postgres & read data to serve in json
