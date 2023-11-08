Comparación de diferentes dispositivos programados con rust.

el objetivo es comprobar la portabilidad real de rust con ejemplos sencillos y publicar los resultados:

https://softwaremakeshardware.wordpress.com/2023/10/29/portabilidad-de-embedded-rust/

se han analizado los siguentes programas *blink*

- [Esp32c6](./blink-esp32c6/README.md) un esp32 con arquitectura risk
    - tambien [sin usar la libreria standar](./blink-esp32c6-nostd/README.md)
- [Attiny85](./blink-attiny85/README.md) como un representante de la familia *arduino / avr*
- [Attiny88](./blink-attini88/README.md) usando *Attiny-hal* en vez de *Arduino-hal*
- [RaspberryPi Pico W](./blink-rpipico/readme.md)
