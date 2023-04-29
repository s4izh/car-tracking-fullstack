# Imagen base
FROM node:14.18.1

# Establecer el directorio de trabajo
WORKDIR /app

# Instalar Ganache
RUN npm install -g ganache-cli

#Instalar Truffle
RUN npm install -g truffle@5.4.23

# Copiar los archivos necesarios
COPY package*.json ./
COPY truffle-config.js ./
COPY contracts ./
COPY migrations ./migrations
COPY src ./src

# Instalar las dependencias
RUN npm install

# Exponer el puerto necesario para la aplicación
EXPOSE 3000 7545 8545


# Compilar y desplegar los contratos inteligentes
RUN truffle compile


# Comando a ejecutar al arrancar el contenedor
CMD ["npm", "run", "dev"]
