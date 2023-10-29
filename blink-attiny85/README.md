
## info
en [av-hal](https://github.com/Rahix/avr-hal/tree/main) (github) se pueden encontrar los requisitos y la forma de crear el proyecto

## requisitos 
  ```bash
xcode-select --install # if you haven't already done so
brew tap osx-cross/avr
brew install avr-gcc avrdude
cargo +stable install ravedude
  ```
## para crear un proyecto 
  ```bash
cargo install cargo-generate
cargo generate --git https://github.com/Rahix/avr-hal-template.git
  ```
	- cuando pregunte por la placa: un attiny85 equivale a un **Adafruit Trinket**
