BENCHMARK
=========

Setup : 
-------

MacBook Pro 2017
CPU : 2,3 GHz Intel Core i5 double core
RAM: 16 Go 2133 MHz LPDDR3

Benchmark tool : 
----------------

WRK (https://github.com/wg/wrk)

RESULTS :
=========

* 1 : GOLANG - gin        (23 767.12 requests/sec)
* 2 : DOTNET - asp net    (20 554.76 requests/sec)
* 3 : RUST - actix        (20 298.21 requests/sec)
* 4 : RUST - rocket       (14 684.19 requests/sec)
* 5 : JAVA - springboot   (8 413.73 requests/sec)
* 6 : NODE - express      (5 361.38 requests/sec)
* 7 : PYTHON - flask      (1 458.26 requests/sec)
* 8 : RUBY - sinatra      (494.32 requests/sec)
* 9 : PHP - slim          (179.57 requests/sec)

S TIER :
--------

* GOLANG
* .NET
* RUST

A TIER :
--------

* JAVA
* NODE

B TIER :
--------

* PYTHON
* RUBY
* PHP


DOTNET (6.0.201):
=================

    wrk -t8 -c100 -d10s http://127.0.0.1:5000/api/user


GOLANG (1.18 - Gin 1.7.7):
==========================

    wrk -t8 -c100 -d10s http://127.0.0.1:5100/api/user


NODE (16.14.2):
===============

    wrk -t8 -c100 -d10s http://127.0.0.1:5200/api/user


PYTHON (3.10.4):
========

    wrk -t8 -c100 -d10s http://127.0.0.1:5300/api/user


JAVA (18 - StringBoot 3.0)
===============

    wrk -t8 -c100 -d10s http://127.0.0.1:5400/api/user


RUBY (2.4)
==========

    wrk -t8 -c100 -d10s http://127.0.0.1:5500/api/user

RUST (1.62 - rocket 0.4.10 / actix 4)
==========

    wrk -t8 -c100 -d10s http://127.0.0.1:5600/api/user

PHP (8.1.4 - slim 4)
==========

    wrk -t8 -c100 -d10s http://127.0.0.1:5700/api/user
