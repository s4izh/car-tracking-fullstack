# Imagen base
FROM ubuntu:latest

# Establecer el directorio de trabajo
# WORKDIR /app/workdir

RUN apt update
RUN apt install curl -y
RUN curl -sL https://deb.nodesource.com/setup_14.x | bash -
RUN apt update
RUN apt install nodejs -y
RUN apt install build-essential -y
RUN apt install tmux -y

RUN npm install -g ganache-cli
RUN npm install -g truffle@5.4.23

# CMD ganache-cli -p 7545 -i 5777 -h 0.0.0.0 -m "tu frase semilla de ganache" --db ganache-data

# CMD sleep 5

# # RUN truffle migrate --network development --reset

# CMD truffle migrate --reset

# Comando a ejecutar al arrancar el contenedor
# CMD ["npm", "run", "dev"]
