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
Este Diagrama lo encuentras aca: 
El diseño presentado en el diagrama DOT utiliza **Programación Orientada a Aspectos (AOP)** para separar responsabilidades transversales y mantener un código más limpio, modular y fácil de mantener. La arquitectura se organiza en clases principales del sistema y aspectos que encapsulan comportamientos que afectan múltiples componentes.

## Clases principales
El sistema define varias clases encargadas de la logica central, tales como:
- **Transacciones**
- **Clientes**
- **Cuentas**
- **Servicios de negocio**
- **Controladores**

Cada una se enfoca unicamente en su responsabilidad principal, evitando cudigo duplicado o logica ajena a su proposito.

## Aspectos transversales
El diagrama introduce aspectos como:
- **LoggingAspect**  
  Captura eventos importantes y registra información útil para auditoría y depuración.

- **SecurityAspect**  
  Verifica permisos, autentica operaciones y protege el acceso a los recursos del sistema.

- **TransactionAspect**  
  Controla el inicio, confirmacion o reversión de transacciones para mantener consistencia en los datos.

Estos aspectos se aplican sobre múltiples clases sin modificar directamente su codigo.

## Beneficio principal del diseño
Este enfoque permite:
- Reducir repetición de codigo.
- Mantener las clases más limpias y enfocadas.
- Centralizar la logica transversal.
- Mejorar la mantenibilidad y la escalabilidad.

En conjunto, el diagrama refleja una arquitectura robusta donde las funcionalidades transversales se desacoplan correctamente mediante el uso de aspectos.
