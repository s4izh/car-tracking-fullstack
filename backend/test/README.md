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
curl -X POST -H "Content-Type: application/json" \
  -d '{"matricula":"9999","hash":"1234"}' \
  http://localhost:8080/api/frontend/create-user
```

#### `api/frontend/login`

For login in, it returns a json with relevant information.

```sh
curl -X POST -H "Content-Type: application/json" \
  -d '{"matricula":"9999","hash":"1234"}' \
  http://localhost:8080/api/frontend/login
```

#### `api/frontend/certificate`

It cerficates current km at the blockchain.

```sh
curl -X POST -H "Content-Type: application/json" \
  -d '{"matricula":"9999","hash":"1234"}' \
  http://localhost:8080/api/frontend/certificate
```

## Endpoints meant to be accessed from the mobile app

#### `api/mobile/create-user`

For creating an user.

- Return and HTTP OK if the user is created.
- Returns an HTTP Bad Request error if the user already exists.

```sh
curl -X POST \ -H "Content-Type: application/json" \
  -d '{"matricula":"9999","hash":"1234"}' \
  http://localhost:8080/api/mobile/login
```

#### `api/mobile/login`

For login in. 

- Return the next expected trip number.
- Returns an HTTP Bad Request error if the user doesn't exists or the password is wrong.

```sh
curl -X POST \ -H "Content-Type: application/json" \
  -d '{"matricula":"9999","hash":"1234"}' \
  http://localhost:8080/api/mobile/login
```

#### `api/mobile/trip`
For sending the info of a trip.

- Will return an OK on a succesfull post
- Returns an HTTP Bad Request error if the user doesn't exists or the password is wrong.
- Returns an HTTP Internal Server error if something fails.

```sh
curl -X POST \ -H "Content-Type: application/json" \
  -d @trip.json \
  http://localhost:8080/api/mobile/trip
```
