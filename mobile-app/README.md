# OBD-Data-Collector-Android
Aplicación de Android programada en Kotlin para el proyecto de especialidad (Tecnologias de la Información).

La aplicación se conecta a un lector de puerto OBD2 mediante bluetooth. Recupera datos como la velocidad actual del vehículo, rpms del motor, posición del acelerador, entre otros.
Estos datos se almacenan en un archivo JSON para su posterior subida a un servidor con backend en RUST.

Cuenta con un sistema básico de autentificación para que el backend pueda confirmar la legitimidad de los datos.
