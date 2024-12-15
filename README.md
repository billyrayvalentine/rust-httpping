# rust-httpping
httpping - A simple ping like tool for HTTP servers

Sends a HEAD request to the given host and prints the response code, url and time taken 

Copyright (c) 2024 Ben Sampson / BillyRayValentine <github.com/billyrayvalentine>

# Usage
```shell
>http-ping -h    
A simple tool to ping HTTP servers and return a HTTP code

Usage: http-ping [OPTIONS] <DESTINATION>

Arguments:
  <DESTINATION>  Destination hostname or ip address

Options:
  -c, --count <COUNT>  Stop after <count> replies
  -r, --redirects      Follow redirects
  -h, --help           Print help
  -V, --version        Print version
```

# Examples
Ping a host forever
```shell
http-ping https://www.httpbin.org/        
200 from https://www.httpbin.org/ in 567 ms seq=1
200 from https://www.httpbin.org/ in 279 ms seq=2
200 from https://www.httpbin.org/ in 244 ms seq=3
200 from https://www.httpbin.org/ in 254 ms seq=4
200 from https://www.httpbin.org/ in 250 ms seq=5
200 from https://www.httpbin.org/ in 1148 ms seq=6
200 from https://www.httpbin.org/ in 244 ms seq=7
200 from https://www.httpbin.org/ in 247 ms seq=8
200 from https://www.httpbin.org/ in 286 ms seq=9
200 from https://www.httpbin.org/ in 277 ms seq=10
200 from https://www.httpbin.org/ in 252 ms seq=11
200 from https://www.httpbin.org/ in 280 ms seq=12
```

Ping a host three times
```shell
http-ping -c3 https://www.httpbin.org/status/401
401 from https://www.httpbin.org/status/401 in 331 ms seq=1
401 from https://www.httpbin.org/status/401 in 276 ms seq=2
401 from https://www.httpbin.org/status/401 in 279 ms seq=3
```