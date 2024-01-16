WEB FRAMEWORK BENCHMARK
=======================

The objective of this benchmark is to compare the performance of several web frameworks with a minimal API approach (just for fun !)

Setup : 
-------

MacBook Pro 
CPU : Apple M1 Max
RAM: 64 Go

Benchmark tool : 
----------------

WRK (https://github.com/wg/wrk)

RESULTS 2024 :
=========

* 1 : RUST - actix        (131 545.40 requests/sec)
* 2 : GOLANG - gin        (128 505.33 requests/sec)
* 3 : DOTNET - asp net    (126 291.80 requests/sec)
* 5 : JAVA - quarkus      (120 657.98 requests/sec)
* 6 : NODE - express      (18 225.17 requests/sec)
* 7 : RUBY - sinatra      (4 925.61 requests/sec)
* 8 : PYTHON - flask      (1 623.43 requests/sec)
* 9 : PHP - slim          (465.38 requests/sec)

S TIER :
--------

* RUST
* GOLANG
* .NET
* JAVA

A TIER :
--------

* NODE

B TIER :
--------

* RUBY
* PYTHON

C TIER : 
--------

* PHP

DOTNET (8.0.100):
=================
    dotnet publish -c Release
    ./bin/Release/Benchmark.Dotnet
    wrk -t8 -c100 -d10s http://127.0.0.1:5000/api/user

GOLANG (1.18 - Gin 1.7.7):
==========================
    go build -ldflags "-s -w" main.go
    ./main
    wrk -t8 -c100 -d10s http://127.0.0.1:5100/api/user

QUARKUS (OpenJDK 21 - Quarkus 3.6.5)
===============
    quarkus build
    java -jar target/quarkus-app/quarkus-run.jar
    wrk -t8 -c100 -d10s http://127.0.0.1:8080/api/user

NODE (20.11):
===============
    node index.js
    wrk -t8 -c100 -d10s http://127.0.0.1:5200/api/user

PYTHON (3.11):
========
    python3 app.py
    wrk -t8 -c100 -d10s http://127.0.0.1:5300/api/user

RUBY (3.3)
==========
    ruby app.rb
    wrk -t8 -c100 -d10s http://127.0.0.1:5500/api/user

RUST (1.75 - actix 4)
==========
    cargo build --release
    ./target/release/benchmark-rust
    wrk -t8 -c100 -d10s http://127.0.0.1:5600/api/user

PHP (8.3.1 - slim 4)
==========

    wrk -t8 -c100 -d10s http://127.0.0.1:5700/api/user
