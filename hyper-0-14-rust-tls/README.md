# hyper-with-tls/hyper-0-14-rust-tls

- Hyper: v0.14 with rust-tls

## HTTP/2

```
% curl -v -k https://localhost:3000
*   Trying 127.0.0.1:3000...
* Connected to localhost (127.0.0.1) port 3000 (#0)
* ALPN: offers h2,http/1.1
* (304) (OUT), TLS handshake, Client hello (1):
* (304) (IN), TLS handshake, Server hello (2):
* (304) (IN), TLS handshake, Unknown (8):
* (304) (IN), TLS handshake, Certificate (11):
* (304) (IN), TLS handshake, CERT verify (15):
* (304) (IN), TLS handshake, Finished (20):
* (304) (OUT), TLS handshake, Finished (20):
* SSL connection using TLSv1.3 / AEAD-CHACHA20-POLY1305-SHA256
* ALPN: server accepted h2
* Server certificate:
*  subject: CN=localhost
*  start date: Nov 17 19:09:27 2023 GMT
*  expire date: Nov 16 19:09:27 2024 GMT
*  issuer: CN=localhost
*  SSL certificate verify result: self signed certificate (18), continuing anyway.
* using HTTP/2
* h2 [:method: GET]
* h2 [:scheme: https]
* h2 [:authority: localhost:3000]
* h2 [:path: /]
* h2 [user-agent: curl/8.1.2]
* h2 [accept: */*]
* Using Stream ID: 1 (easy handle 0x14d812800)
> GET / HTTP/2
> Host: localhost:3000
> User-Agent: curl/8.1.2
> Accept: */*
>
< HTTP/2 200
< date: Mon, 20 Nov 2023 17:29:11 GMT
< content-length: 12
<
* Connection #0 to host localhost left intact
Hello, World%
```

Server logs: `Error serving connection: hyper::Error(Io, Kind(UnexpectedEof))`

## HTTP/1.1

```
% curl -v -k --http1.1 https://localhost:3000
*   Trying 127.0.0.1:3000...
* Connected to localhost (127.0.0.1) port 3000 (#0)
* ALPN: offers http/1.1
* (304) (OUT), TLS handshake, Client hello (1):
* (304) (IN), TLS handshake, Server hello (2):
* (304) (IN), TLS handshake, Unknown (8):
* (304) (IN), TLS handshake, Certificate (11):
* (304) (IN), TLS handshake, CERT verify (15):
* (304) (IN), TLS handshake, Finished (20):
* (304) (OUT), TLS handshake, Finished (20):
* SSL connection using TLSv1.3 / AEAD-CHACHA20-POLY1305-SHA256
* ALPN: server accepted http/1.1
* Server certificate:
*  subject: CN=localhost
*  start date: Nov 17 19:09:27 2023 GMT
*  expire date: Nov 16 19:09:27 2024 GMT
*  issuer: CN=localhost
*  SSL certificate verify result: self signed certificate (18), continuing anyway.
* using HTTP/1.1
> GET / HTTP/1.1
> Host: localhost:3000
> User-Agent: curl/8.1.2
> Accept: */*
>
< HTTP/1.1 200 OK
< content-length: 12
< date: Mon, 20 Nov 2023 17:29:34 GMT
<
* Connection #0 to host localhost left intact
Hello, World%
```

Server doesn't log anything.
