# Punto 1 – Resumen del Diagrama de Regresion Lineal Concurrente

 El diagrama se puede ver dandole clic al siguente vienculo: [Diagrama: ](./Puntop_1.png)


El diagrama representa el **diseño concurrente de una regresion lineal distribuida** usando el paradigma de concurrencia y comunicacion por canales (inspirado en π-calculo). Los elementos principales son:

## Coordinador / Orquestador
- Controla el inicio y fin del entrenamiento (`ctrl:start` / `ctrl:stop`).  
- Supervisa los checkpoints y la terminación de cada epoch.

## Trabajadores (Trabajador 1 … Trabajador N)
- Cada uno procesa un **subconjunto de datos** `(X_i, y_i)`.  
- Calcula los **gradientes parciales** `dw_i` y `db_i` de la funcion de pérdida (MSE).  
- Envia los gradientes al Parameter Server a través del canal `grad`.  
- Recibe los parámetros actualizados `(w,b)` desde el Parameter Server a través del canal `param` para la siguiente iteracion.

## Parameter Server (PS)
- Recibe los gradientes de todos los trabajadores.  
- **Agrega** los gradientes para calcular `dw_total` y `db_total`.  
- Actualiza los parametros `(w,b)` usando **gradiente descendente**.  
- Difunde los parametros actualizados a los trabajadores mediante el canal `param`.  
- Envía métricas (MSE, parámetros) al Logger mediante el canal `log`.

## Logger / Monitor
- Recibe métricas del PS a través del canal `log`.  
- Informa al coordinador sobre la convergencia y el progreso del entrenamiento.

## Canales de comunicación
- `ctrl`: señales de control (inicio, fin, reportes).  
- `param`: transmisión de parámetros `(w,b)` hacia los trabajadores.  
- `grad`: envío de gradientes `(dw_i, db_i)` de trabajadores al PS.  
- `log`: envío de métricas y resultados hacia el Logger.

## Conclusion:  
- Este diseño permite paralelizar el cálculo de gradientes en múltiples trabajadores y actualizar los parámetros de manera coordinada, reflejando la **lógica de regresión lineal** de manera concurrente, clara y formal. Cada componente tiene un rol directo en el flujo de información y en el cálculo de la función de pérdida.
---

## Punto 2. Resumen del diseño basado en Aspectos
El diagrama se puede ver dandole clic al siguente vienculo: [Diagrama: ](./puntop_2.png)

- El diseño propuesto utiliza el paradigma de Programacion Orientada a Aspectos (AOP) para estructurar la solucion de regresion lineal mediante gradiente descendente. La arquitectura separa claramente la logica principal del modelo de los comportamientos transversales, logrando un sistema modular y facil de mantener.

- En la parte funcional, el modulo **Datos** gestiona la carga, normalizacion y provision de los vectores XXX y yyy. El **ModeloRegresion** contiene los parametros del modelo (www y bbb) y provee los metodos para predecir y actualizar dichos parametros. El componente **Entrenador** implementa el algoritmo de gradiente descendente, calculando el error, los gradientes y la actualizacion de los parametros en cada epoch.

- Los aspectos complementan este proceso sin modificar el codigo base. El **Aspecto Logging** registra eventos como el inicio y fin de cada epoch y las actualizaciones realizadas sobre el modelo. El **Aspecto Monitoreo** mide metricas importantes, como el tiempo de ejecucion por epoch y la evolucion del error (MSE), permitiendo analizar la convergencia. Por ultimo, el **Aspecto Validacion** verifica la coherencia y validez de los datos antes de iniciar el entrenamiento, evitando errores durante la ejecucion.


---

## Punto 3
- Ver reasme y implentacion de los codigos de phyton y rust de regresion lineal en el siguiente vinculo: 
