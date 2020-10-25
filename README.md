# Music Store
Microservice built with REST pattern written in Rust.


![icon](https://github.com/pepitoenpeligro/music_store/blob/master/docs/imgs/icon.png)

# Build and run

```shell
cargo build 
cargo run
```

# How it works?

It accepts HTTP requests, so we can use any utility that sends HTTP packets. We recommend the use of `curl`.

Here are two examples of basic HTTP requests accepted by our microservice

__GET__ requests:
```bash
curl --location --request GET 'localhost:8000/songs' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain'
```



__POST__ requests:
```bash
curl --location --request POST 'localhost:8000/songs' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
    "title": "Physical",
    "duration": 10
}'
```

Below are screenshots of running these examples with the microservice:

![Cargo build and run](https://github.com/pepitoenpeligro/music_store/blob/master/docs/imgs/00-cargo-build-run.png)
![Curl get and post](https://github.com/pepitoenpeligro/music_store/blob/master/docs/imgs/00-curl-get-post.png)
![Web browser Get-HTTP method example](https://github.com/pepitoenpeligro/music_store/blob/master/docs/imgs/00-web-browser-get.png)


## Docker

To create docker image:
```bash
docker build --no-cache -t musicstore -f Dockerfile .
```

To start api:
```bash
docker run -d -p 8000:8000 musicstore 
```
![Docker build and run](https://github.com/pepitoenpeligro/music_store/blob/master/docs/imgs/01-docker-build-run.png)


01-docker-build-run.png

## References
* [Tokio](https://github.com/tokio-rs/tokio)
* [How Build Small and Fast Docker Images](https://shaneutt.com/blog/rust-fast-small-docker-image-builds/)