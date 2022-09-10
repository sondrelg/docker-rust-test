Quick demo repo

```
> docker build . -t docker-rust-test 
> docker run docker-rust-test ./prime 11
11 is prime

❯ docker images | grep docker-rust-test      
REPOSITORY         TAG       IMAGE ID       CREATED          SIZE
docker-rust-test   latest    652e1f7c242f   10 seconds ago   3.74MB
