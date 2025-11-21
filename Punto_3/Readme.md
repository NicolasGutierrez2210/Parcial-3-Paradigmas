# Comparación de rendimiento: Regresióo Lineal en Rust vs Python

Este experimento mide el tiempo de ejecución de una implementación básica de **regresión lineal** en dos lenguajes: **Rust** y **Python**.  
Ambos códigos realizan exactamente la misma tarea: calcular los coeficientes `m` y `b` de una recta que mejor se ajusta a un conjunto de datos usando la **formula cerrada**.

---

##  Resultados de rendimiento

| Lenguaje | Tiempo de ejecución |
|---------|----------------------|
| **Rust** | **674.444 µs** |
| **Python** | **0.025098562240600586 s** |

---

##  Interpretación

- Rust ejecuta la operación en aproximadamente **0.000674 segundos**, es decir **unas 37 veces más rápido** que Python en esta prueba específica.
- La diferencia se debe principalmente a:
  - Compilación a binario nativo en Rust.
  - Optimización del compilador.
  - Python depende de un intérprete, lo cual introduce sobrecarga.
- Si el tamaño de los datos creciera, la ventaja de Rust sería aún mayor.

---

## Codigos utilizados





---

## Conclusion 

- Rust es significativamente más rápido en tareas numéricas intensivas debido a su modelo de compilación y optimización.
Python ofrece simplicidad y rapidez para programar, pero sacrifica rendimiento en comparación con lenguajes compilados.

Ambos lenguajes cumplen correctamente con la regresión lineal; la elección depende del equilibrio entre productividad y rendimiento.

---

## Grafico de comparacion

<img width="1375" height="780" alt="image" src="https://github.com/user-attachments/assets/76ac61bf-8363-4ead-9c76-cbfcf0ff6a84" />


