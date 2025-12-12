# Lang API

```sh
docker pull mongodb/mongodb-community-server:latest
docker run --name mongodb -p 27017:27017 -d mongodb/mongodb-community-server:latest
docker start mongodb
```

```sh
cargo run
```

# Swagger

`http://localhost:3000/swagger-ui/`

```
http://localhost:3000/api/v1/en
http://localhost:3000/api/v1/{lang}/personal-pronouns
```