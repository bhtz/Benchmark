BENCHMARK
=========

Setup : 

MacBook Pro 2017
CPU : 2,3 GHz Intel Core i5 double core
RAM: 16 Go 2133 MHz LPDDR3

ApacheBench, Version 2.3

DOTNET (6.0.201):
=================

ab -n 10000 -c 100 http://127.0.0.1:5000/api/user

Requests per second:    3165.90 [#/sec] (mean)
Requests per second:    3765.36 [#/sec] (mean)
Requests per second:    3968.57 [#/sec] (mean)

MOYENNE : 3632 r/s

GOLANG (1.18 - Gin 1.7.7):
==========================

ab -n 10000 -c 100 http://127.0.0.1:5100/api/user

Requests per second:    1666.49 [#/sec] (mean)
Requests per second:    1244.32 [#/sec] (mean)
Requests per second:    1813.94 [#/sec] (mean)

MOYENNE : 1574 r/s

NODE (16.14.2):
===============

ab -n 10000 -c 100 http://127.0.0.1:5200/api/user

Requests per second:    2755.53 [#/sec] (mean)
Requests per second:    3126.53 [#/sec] (mean)
Requests per second:    2546.57 [#/sec] (mean)

MOYENNE : 2809 r/s

PYTHON (3.10.4):
========

ab -n 10000 -c 100 http://127.0.0.1:5300/api/user

Requests per second:    712.35 [#/sec] (mean)
Requests per second:    434.21 [#/sec] (mean)
Requests per second:    621.34 [#/sec] (mean)

MOYENNE : 589 r/s

JAVA (18 - StringBoot 3.0)
===============

ab -n 10000 -c 100 http://127.0.0.1:5400/api/user

Requests per second:    2560.05 [#/sec] (mean)
Requests per second:    564.67 [#/sec] (mean)
Requests per second:    3941.60 [#/sec] (mean)
Requests per second:    413.90 [#/sec] (mean)
Requests per second:    1691.86 [#/sec] (mean)
Requests per second:    513.44 [#/sec] (mean)

MOYENNE : 1613 r/s

RUBY (2.4)
==========

ab -n 10000 -c 100 http://127.0.0.1:5500/api/user

Requests per second:    614.06 [#/sec] (mean)
Requests per second:    465.21 [#/sec] (mean)
Requests per second:    515.53 [#/sec] (mean)

MOYENNE : 531 r/s

RESULTS (AVG): 
==============

* 1 : DOTNET (3632 r/s)
* 2 : NODE (2809 r/s)
* 3 : JAVA (1613 r/s)
* 4 : GOLANG (1574 r/s)
* 5 : PYTHON (589 r/s)
* 6 : RUBY (531 r/s)