# A Rustful Hello World, powered by Actix

Super trivial webservice, consisting of a single endpoint that says Hi!

Demonstrates:

* getting the Git SHA and embedding in app.
* using multi-stage Docker build, and distroless, to end up with an ~31MB app

```bash
docker build -t hello-rust .
docker run -p 1111:1111 hello-rust:latest
```

