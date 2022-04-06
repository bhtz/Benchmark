BENCHMARK
=========

Setup : 

MacBook Pro 2017
CPU : 2,3 GHz Intel Core i5 double core
RAM: 16 Go 2133 MHz LPDDR3

Benchmark tool : 

(Round 1) 
ApacheBench, Version 2.3

(Round 2) 
autocannon v7.8.1
node v16.14.2

(ROUND 1) APACHE BENCH (AVG): 
================================

* 1 : DOTNET (3632 r/s)
* 2 : NODE (2809 r/s)
* 3 : JAVA (1613 r/s)
* 4 : GOLANG (1574 r/s)
* 5 : PYTHON (589 r/s)
* 6 : RUBY (531 r/s)

(ROUND 2) AUTOCANNON (AVG): 
==============================

* 1 : DOTNET - asp net (12 065 r/s)
* 2 : RUST - actix (4939.7 r/s)
* 3 : JAVA - springboot (4831.78 r/s)
* 4 : NODE - express (4459.2 r/s)
* 5 : GOLANG - gin (4292.5 r/s)
* 5 : RUST - rocket (2404.31 r/s)
* 7 : PYTHON - flask (1407 r/s)
* 8 : RUBY - sinatra (525.4 r/s)

(ROUND 3) WRK (AVG): 
====================

* 1 : DOTNET - asp net    (2.36k r/s)
* 2 : RUST - actix        (1.11k r/s)
* 3 : JAVA - springboot   (1.10k r/s)
* 4 : GOLANG - gin        (745.44 r/s)
* 5 : NODE - express      (668.48 r/s)
* 5 : RUST - rocket       (532.00 r/s)
* 7 : PYTHON - flask      (183.51 r/s)
* 8 : RUBY - sinatra      (64.88 r/s)


DOTNET (6.0.201):
=================

ab -n 10000 -c 100 http://127.0.0.1:5000/api/user

...

autocannon http://localhost:5000/api/user -d 10 -c 100 -w 3

│ Stat      │ 1%      │ 2.5%    │ 50%     │ 97.5%   │ Avg     │ Stdev   │ Min     │
├───────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┤
│ Req/Sec   │ 6927    │ 6927    │ 12775   │ 13751   │ 12065   │ 2019.62 │ 6924    │
├───────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┤
│ Bytes/Sec │ 1.96 MB │ 1.96 MB │ 3.61 MB │ 3.89 MB │ 3.41 MB │ 571 kB  │ 1.96 MB │

wrk -t8 -c100 -d10s http://127.0.0.1:5000/api/user

  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    15.82ms   32.40ms 582.79ms   89.73%
    Req/Sec     2.36k   609.13     4.55k    72.25%


GOLANG (1.18 - Gin 1.7.7):
==========================

ab -n 10000 -c 100 http://127.0.0.1:5100/api/user

...

autocannon http://localhost:5100/api/user -d 10 -c 100 -w 3

│ Stat      │ 1%     │ 2.5%   │ 50%     │ 97.5%  │ Avg     │ Stdev   │ Min    │
├───────────┼────────┼────────┼─────────┼────────┼─────────┼─────────┼────────┤
│ Req/Sec   │ 1529   │ 1529   │ 4323    │ 6063   │ 4292.5  │ 1296.18 │ 1529   │
├───────────┼────────┼────────┼─────────┼────────┼─────────┼─────────┼────────┤
│ Bytes/Sec │ 379 kB │ 379 kB │ 1.07 MB │ 1.5 MB │ 1.06 MB │ 321 kB  │ 379 kB

wrk -t8 -c100 -d10s http://127.0.0.1:5100/api/user

  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    30.63ms   42.16ms 297.90ms   86.69%
    Req/Sec   745.44    315.66     1.43k    63.28%


NODE (16.14.2):
===============

ab -n 10000 -c 100 http://127.0.0.1:5200/api/user

...

autocannon http://localhost:5200/api/user -d 10 -c 100 -w 3

│ Stat      │ 1%     │ 2.5%   │ 50%     │ 97.5%   │ Avg     │ Stdev  │ Min    │
├───────────┼────────┼────────┼─────────┼─────────┼─────────┼────────┼────────┤
│ Req/Sec   │ 2413   │ 2413   │ 4563    │ 5295    │ 4459.2  │ 825.89 │ 2412   │
├───────────┼────────┼────────┼─────────┼─────────┼─────────┼────────┼────────┤
│ Bytes/Sec │ 868 kB │ 868 kB │ 1.64 MB │ 1.91 MB │ 1.61 MB │ 297 kB │ 868 kB

wrk -t8 -c100 -d10s http://127.0.0.1:5200/api/user

  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    18.95ms   10.21ms 137.17ms   90.99%
    Req/Sec   668.48    207.03     0.97k    79.75%


PYTHON (3.10.4):
========

ab -n 10000 -c 100 http://127.0.0.1:5300/api/user

...

autocannon http://localhost:5300/api/user -d 10 -c 100 -w 3

│ Stat      │ 1%     │ 2.5%   │ 50%    │ 97.5%  │ Avg    │ Stdev   │ Min    │
├───────────┼────────┼────────┼────────┼────────┼────────┼─────────┼────────┤
│ Req/Sec   │ 1071   │ 1071   │ 1364   │ 1900   │ 1407   │ 214.42  │ 1071   │
├───────────┼────────┼────────┼────────┼────────┼────────┼─────────┼────────┤
│ Bytes/Sec │ 306 kB │ 306 kB │ 390 kB │ 544 kB │ 402 kB │ 61.3 kB │ 306 kB 

wrk -t8 -c100 -d10s http://127.0.0.1:5300/api/user

  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    69.67ms   36.70ms 448.77ms   87.87%
    Req/Sec   183.51     47.45   450.00     73.61%


JAVA (18 - StringBoot 3.0)
===============

ab -n 10000 -c 100 http://127.0.0.1:5400/api/user

...

autocannon http://localhost:5400/api/user -d 10 -c 100 -w 3

│ Stat      │ 1%     │ 2.5%   │ 50%     │ 97.5%   │ Avg     │ Stdev   │ Min    │
├───────────┼────────┼────────┼─────────┼─────────┼─────────┼─────────┼────────┤
│ Req/Sec   │ 2571   │ 2571   │ 5203    │ 6423    │ 4831.78 │ 1191.28 │ 2571   │
├───────────┼────────┼────────┼─────────┼─────────┼─────────┼─────────┼────────┤
│ Bytes/Sec │ 764 kB │ 764 kB │ 1.54 MB │ 1.91 MB │ 1.43 MB │ 353 kB  │ 764 kB

wrk -t8 -c100 -d10s http://127.0.0.1:5400/api/user

  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    17.66ms   38.49ms 532.91ms   95.72%
    Req/Sec     1.10k   283.88     2.57k    72.35%

RUBY (2.4)
==========

ab -n 10000 -c 100 http://127.0.0.1:5500/api/user

...

autocannon http://localhost:5500/api/user -d 10 -c 100 -w 3

│ Stat      │ 1%     │ 2.5%   │ 50%    │ 97.5%  │ Avg    │ Stdev │ Min    │
├───────────┼────────┼────────┼────────┼────────┼────────┼───────┼────────┤
│ Req/Sec   │ 449    │ 449    │ 538    │ 586    │ 525.4  │ 38.49 │ 449    │
├───────────┼────────┼────────┼────────┼────────┼────────┼───────┼────────┤
│ Bytes/Sec │ 152 kB │ 152 kB │ 182 kB │ 198 kB │ 178 kB │ 13 kB │ 152 kB 

wrk -t8 -c100 -d10s http://127.0.0.1:5500/api/user

  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   226.52ms  218.17ms   1.26s    54.63%
    Req/Sec    64.88     56.05   535.00     89.28%

RUST (1.62 - rocket 0.4.10)
==========

autocannon http://localhost:5600/api/user -d 10 -c 100 -w 3

ROCKET (0.5) 

│ Stat      │ 1%     │ 2.5%   │ 50%    │ 97.5%   │ Avg     │ Stdev  │ Min    │
├───────────┼────────┼────────┼────────┼─────────┼─────────┼────────┼────────┤
│ Req/Sec   │ 1632   │ 1632   │ 2381   │ 2935    │ 2404.31 │ 393.37 │ 1632   │
├───────────┼────────┼────────┼────────┼─────────┼─────────┼────────┼────────┤
│ Bytes/Sec │ 575 kB │ 575 kB │ 838 kB │ 1.03 MB │ 846 kB  │ 138 kB │ 574 kB 

wrk -t8 -c100 -d10s http://127.0.0.1:5600/api/user

  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    24.93ms   18.85ms 286.37ms   90.65%
    Req/Sec   532.00    113.68   686.00     82.75%


ACTIX (4)

│ Stat      │ 1%     │ 2.5%   │ 50%     │ 97.5%   │ Avg     │ Stdev   │ Min    │
├───────────┼────────┼────────┼─────────┼─────────┼─────────┼─────────┼────────┤
│ Req/Sec   │ 2983   │ 2983   │ 4555    │ 6735    │ 4939.7  │ 1190.45 │ 2983   │
├───────────┼────────┼────────┼─────────┼─────────┼─────────┼─────────┼────────┤
│ Bytes/Sec │ 698 kB │ 698 kB │ 1.07 MB │ 1.58 MB │ 1.16 MB │ 278 kB  │ 698 kB 

wrk -t8 -c100 -d10s http://127.0.0.1:5600/api/user

  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    12.28ms   11.30ms 141.47ms   94.72%
    Req/Sec     1.11k   284.72     1.70k    69.57%