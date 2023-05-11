# Car Tracking

## Deployment

Automatic deployment of all containers can be done with the following:

```bash
docker-compose -f prod.yml build
docker-compose -f prod.yml up
```

### Blockchain

Blockchain deployment has to be done manually,
after running the previous commands run the following:

```bash
make ganache
make truffle
```

We recommend using [tmux](https://github.com/tmux/tmux/wiki) for handling multiple shells.

## Endpoints documentation

The endpoints documentation can be found [here](backend/test/README.md).

## Running the development environment

- Run docker compose to start all the containers
- `make setup` sets up the database and runs the backend

```bash
docker-compose up -d
make setup
```

### Docker

The following command will give you a shell in $CONTAINER_NAME,
check the `Makefile` since some names are shortened.

```bash
make docker-$CONTAINER_NAME
```

Remove everything in case of random cache failures and whatever:

```bash
docker system prune -a 
docker rm -f $(docker ps -aq)
docker rmi -f $(docker images -aq)
docker volume rm $(docker volume ls -q)
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

Open a shell in the `mariadb` container, with a prompt inside the database:

```bash
docker exec -ti mariadb mysql -U db -prust
```

### Frontend dev

Run the general development enviroment commands [here](#Running-the-development-environment).

Useful commands:

```bash
make docker-front # enter the frontend container
make frontend-run # compiles and runs the frontend
```

## Running the blockchain

### No containers

Info can be found [here](blockchain/Readme.org)

### In containers

We recommend the containerized version.

```bash
make ganache
make truffle
```
