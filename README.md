# Conway's Game of Life

Implementación del clásico algoritmo celular inventado por el matemático John Horton Conway, renderizado en tiempo real desde cero en Rust utilizando un Framebuffer personalizado.

## Demostración

![Game of Life Gameplay](game_of_life.gif)


## Características del Proyecto

- **Lógica de Wrap-Around (Efecto Pac-Man):** Las células que cruzan el límite derecho o inferior de la pantalla reaparecen por el lado opuesto, permitiendo que las naves floten infinitamente sin chocar con un muro.
- **Resolución Personalizada:** Mundo interno de `160x120` células, escalado dinámicamente a una ventana de `800x600` sin pérdida de aspecto
- **Colores Personalizados:** Células vivas en `0x66BAFF` y muertas en `0xFFD1FB`.

## Organismos y Patrones Incluidos

El simulador se carga inicialmente con una colección extensa de patrones que abarcan todas las familias clásicas del juego, además de algunas estructuras avanzadas generadoras de caos:

1. **Still lifes (Vidas estáticas):** Block, Bee-hive, Loaf, Boat y Tub.
2. **Oscillators (Osciladores):** Blinker, Toad, Beacon, Pulsar y Penta-decathlon.
3. **Spaceships (Naves espaciales):** Glider, Lightweight spaceship (LWSS), Middle-weight spaceship (MWSS) y Heavy-weight spaceship (HWSS).
4. **Armas y Trenes Avanzados:**
   - *Gosper Glider Gun:* Un arma infinita descubierta por Bill Gosper que dispara Gliders periódicamente.
   - *True Puffer Train:* (Gosper's Puffer 1) Una colosal estructura móvil que avanza dejando atrás humo y escombros de forma infinita.
   - *Glider Fleet:* Un escuadrón sincronizado de naves (Spaceships).

## Cómo Ejecutar

Para correr la simulación, asegúrate de tener Rust instalado y ejecuta:

```bash
cargo run
```
