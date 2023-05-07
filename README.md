# Car Tracking

## Endpoints documentation

The endpoints documentation can be found [here](backend/test/README.md).

## Deployment

Automatic deployment of all containers can be done with the following:

```bash
docker-compose -f prod.yml build
docker-compose -f prod.yml up
```

## Running the development environment

- Run docker compose to start all the containers
- `make setup` sets up the database and runs the backend

```bash
docker-compose up -d
make setup
```

### Backend dev

After `docker-compose up -d` these are useful commands:

```bash
make backend-setup # compiles and runs the backend
make db-setup # sets up the database
make db-reset # resets the database
```

**Note** there's no need of `make setup` when developing the backend, since we do 
it manually.

#### MariaDB

Open a shell in the `mariadb` container:

```bash
docker exec -ti mariadb mysql -U db -prust
```

### Frontend dev

Run the general development enviroment commands [here](#Running-the-development-environment).

Useful commands:

```bash
make frontend-run # compiles and runs the frontend
```

### Useful docker commands 

Removing everything:

```bash
docker rm -f $(docker ps -aq)
docker rmi -f $(docker images -aq)
docker volume rm $(docker volume ls -q)
```
