# Endpoint Documentation and Testing

## Endpoints meant to be accessed from the frontend

#### `api/frontend/test`

For testing, always return the same response.

```sh
curl -i localhost:8080/api/frontend/test
```

#### `api/frontend/create-user`

For creating a new user.

```sh
curl -X POST -H "Content-Type: application/json" -d \
'{"matricula":"9999","hash":"1234"}' \
http://localhost:8080/api/frontend/create-user
```

#### `api/frontend/login`

For login in, it returns a json with relevant information.

```sh
curl -X POST -H "Content-Type: application/json" -d \
'{"matricula":"9999","hash":"1234"}' \
http://localhost:8080/api/frontend/login
```

#### `api/frontend/certificate`

It cerficates current km at the blockchain.

```sh
curl -X POST -H "Content-Type: application/json" -d \
'{"matricula":"9999","hash":"1234"}' \
http://localhost:8080/api/frontend/certificate
```
