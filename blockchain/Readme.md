Para testear que todo funciona correctamente, puedes seguir los siguientes pasos:

-Inicia ganache-cli “./ganache…”.

-Ejecuta truffle compile para compilar los contratos.

-En una terminal, desde la raíz del proyecto, ejecuta truffle migrate --reset para desplegar los contratos en la red local.

-En otra terminal, también desde la raíz del proyecto, ejecuta npm run dev para iniciar el servidor web.

-Abre tu navegador y navega a http://localhost:3000 para acceder a la interfaz web.

-Introduce los datos de un certificado en la página web y envíalos. Deberías ver un mensaje de éxito si todo va bien.

-Para verificar que el certificado se ha almacenado en la blockchain, puedes utilizar la consola de Truffle. En la terminal donde ejecutaste truffle migrate --reset, ejecuta truffle console para abrir la consola de Truffle.

-En la consola, ejecuta let instance = await CarKilometerCertification.deployed() para obtener una instancia del contrato.

-Luego, ejecuta let certs = await instance.getCertificates() para obtener una lista de los certificados almacenados en la blockchain. Si todo ha ido bien, deberías ver una lista que incluya el certificado que acabas de crear.

-También puedes utilizar la función getCertificateDetails para obtener más información sobre un certificado específico, proporcionando su ID como parámetro. Por ejemplo, let certDetails = await instance.getCertificateDetails(1) te proporcionará los detalles del certificado con ID 1.