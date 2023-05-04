#!/bin/bash

# Ejecutar el ganache
ganache-cli -p 7545 -i 5777 -h 0.0.0.0 -m "tu frase semilla de ganache" --db /ganache-data 

# Esperar a que Ganache se inicie correctamente
sleep 5

# Compilar contratos
truffle compile

# Ejecutar el comando "truffle migrate" para migrar los contratos a la red de Ganache
truffle migrate --reset

# Ejecutar el comando "npm run dev" para arrancar la aplicación
npm run dev
