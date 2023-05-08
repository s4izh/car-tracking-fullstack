# Endpoint Documentation and Testing

Use the scripts in this directory to fill the database 
with some data for testing.

The variable `BACKEND_URL` must be set, example:

```sh
export BACKEND_URL=localhost:8080
./create_user # create the user
./send_trips # send multiple trips
./get_trips # get a json with all the trips
```

## Common Endpoints

#### `api/create-user`

For creating an user.

- Returns an HTTP OK and a user created message.
- Returns an HTTP Bad Request error if the user already exists.
- Returns an HTTP Internal Server error if something fails.

```sh
curl -X POST -H "Content-Type: application/json" \
  -d '{"matricula":"100000","hash":"abcde"}' \
  http://$BACKEND_URL/api/create-user
```

#### `api/login`

For login in. 

- Returns an HTTP OK and json with the user and the hash needed to further communication.
- Returns an HTTP Bad Request error if the user doesn't exists or the password is wrong.
- Returns an HTTP Internal Server error if something fails.

```sh
curl -i -X POST -H "Content-Type: application/json" \
  -d '{"matricula":"100000","hash":"abcde"}' \
  http://$BACKEND_URL/api/login
```

## Endpoints meant to be accessed from the frontend

#### `api/frontend/get-trips`

Returns a json with all the information of all the trips of a given user.

- Returns an HTTP OK and a json with the trips.
- Returns an HTTP Bad Request error if the user doesn't exists or the hash is wrong.
- Returns an HTTP Internal Server error if something fails.

```sh
curl -X POST -H "Content-Type: application/json" \
  -d '{"matricula":"100000","hash":"abcde"}' \
  http://$BACKEND_URL/api/frontend/get-trips
```

#### `api/frontend/certificate`

It cerficates current km at the blockchain.

- Returns an HTTP OK and certificated completed message.
- Returns an HTTP Bad Request error if the user doesn't exists or the hash is wrong.
- Returns an HTTP Internal Server error if something fails.

```sh
curl -i -X POST -H "Content-Type: application/json" \
  -d '{"matricula":"100000","hash":"abcde"}' \
  http://$BACKEND_URL/api/frontend/certificate
```

#### `api/frontend/test`

For testing, always return the same response.

```sh
curl -i $BACKEND_URL/api/frontend/test
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
  http://$BACKEND_URL/api/mobile/trip
```
