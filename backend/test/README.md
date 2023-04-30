# Endpoint Documentation and Testing

## Common Endpoints

#### `api/create-user`

For creating an user.

- Returns an HTTP OK and a login succesfull message.
- Returns an HTTP Bad Request error if the user already exists.
- Returns an HTTP Internal Server error if something fails.

```sh
curl -X POST -H "Content-Type: application/json" \
  -d '{"matricula":"9999","hash":"1234"}' \
  http://localhost:8080/api/create-user
```

#### `api/login`

For login in. 

- Returns an HTTP OK and a user created message.
- Returns an HTTP Bad Request error if the user doesn't exists or the password is wrong.
- Returns an HTTP Internal Server error if something fails.

```sh
curl -i -X POST -H "Content-Type: application/json" \
  -d '{"matricula":"9999","hash":"1234"}' \
  http://localhost:8080/api/login
```

## Endpoints meant to be accessed from the frontend

#### `api/frontend/test`

For testing, always return the same response.

```sh
curl -i localhost:8080/api/frontend/test
```

#### `api/frontend/certificate`

It cerficates current km at the blockchain.

```sh
curl -i -X POST -H "Content-Type: application/json" \
  -d '{"matricula":"9999","hash":"1234"}' \
  http://localhost:8080/api/frontend/certificate
```

## Endpoints meant to be accessed from the mobile app

#### `api/mobile/trip`
For sending the info of a trip.

- Will return an OK on a succesfull post
- Returns an HTTP Bad Request error if the user doesn't exists or the password is wrong.
- Returns an HTTP Internal Server error if something fails.

```sh
curl -i -X POST \ -H "Content-Type: application/json" \
  -d @trip.json \
  http://localhost:8080/api/mobile/trip
```
