
## info
en [av-hal](https://github.com/Rahix/avr-hal/tree/main) (github) se pueden encontrar los requisitos y la forma de crear el proyecto

## requisitos 
  ```bash
xcode-select --install # if you haven't already done so
brew tap osx-cross/avr
brew install avr-gcc avrdude
cargo +stable install ravedude
  ```

## run 

```bash
cargo build && avrdude -c usbtiny -p attiny84 -Uflash:w:target/avr-attiny84/debug/blink-attiny84.elf
```

tambien se puede usar `cargo run`