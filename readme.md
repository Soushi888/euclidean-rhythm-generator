# Rust Euclidean Rhythm Generator

This CLI program generates and plays euclidean rhythms.

## Usage


```sh
./rust-euclidean-rhythm-generator BEATS STEPS [OPTION]
```

### Parameters
- `BEATS`: Number of beats or one of the predefined rhythms (default: "3").
    - `tressio`: A predefined rhythm of 3 beats in 8 steps.
    - `quintillo`: A predefined rhythm of 5 beats in 8 steps.
    - `venda`: A predefined rhythm of 5 beats in 12 steps.
    - `bembe`: A predefined rhythm of 7 beats in 12 steps.
- `STEPS`: Number of steps (default: "8").

- `OPTION`: A special option to manipulate the rhythm (default: none).
    - `inverse`: Inverses the rhythm.

## Example

To play a rhythm with 5 beats over 16 steps:
    
```sh
./rust-euclidean-rhythm-generator 5 16
```

To play a predefined rhythm with 5 beats over 12 steps:

```sh
./rust-euclidean-rhythm-generator venda
```

To play a predefined rhythm with 5 beats over 12 steps, but inverse:

```sh
./rust-euclidean-rhythm-generator venda inverse
```

To play a rhythm with 7 beats over 16 steps, but inverse:

```sh
./rust-euclidean-rhythm-generator 7 16 inverse
```

## Sound

The program uses a predefined sound file (quick_fart_x.wav) to play the beats. The rests are silent.