#+title: Instrucciones para runnear blockchain en local:

- Para testear que todo funciona correctamente, puedes seguir los siguientes pasos:

- Inicia ganache-cli .
 #+begin_src sh
  ganache-cli -p 7545 -i 5777 -h 0.0.0.0 -m "tu frase semilla de ganache"
#+end_src

- Compila los contratos desde la raíz del proyecto.
#+begin_src sh
  truffle complie
#+end_src

- En una terminal, desde la raíz del proyecto, despliega los contratos en la red local.
#+begin_src sh
  truffle migrate --reset
#+end_src

- En una terminal, también desde la raíz del proyecto, inicia el servidor web.
#+begin_src sh
  npm run dev
#+end_src
