Para testear que todo funciona correctamente, puedes seguir los siguientes pasos:

-Inicia ganache-cli “ganache-cli -p 7545 -i 5777 -h 0.0.0.0 -m "tu frase semilla de ganache"”.

-Ejecuta "truffle compile" en otra terminal, desde la raíz del proyecto,  para compilar los contratos.

-En una terminal, desde la raíz del proyecto, ejecuta "truffle migrate --reset" para desplegar los contratos en la red local.

-En una terminal, también desde la raíz del proyecto, ejecuta "npm run dev" para iniciar el servidor web.
