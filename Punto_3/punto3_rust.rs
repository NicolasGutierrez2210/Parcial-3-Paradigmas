use std::time::Instant;

fn main() {
    // Datos de ejemplo
    let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];

    // Parametros Modelo
    let mut w = 0.0_f64;
    let mut b = 0.0_f64;

    // Hiperparametros
    let learning_rate = 0.01;
    let epochs = 1000;
    let m = x.len() as f64;

    // Cronometro para medir el rendimiento
    let start = Instant::now();

    for epoch in 0..epochs {
        let mut error_sum = 0.0;
        let mut error_x_sum = 0.0;

        for i in 0..x.len() {
            let y_pred = w * x[i] + b;
            let error = y_pred - y[i];

            error_sum += error;
            error_x_sum += error * x[i];
        }

        // Gradientes
        let dw = (2.0 / m) * error_x_sum;
        let db = (2.0 / m) * error_sum;

        // Actualizacion
        w -= learning_rate * dw;
        b -= learning_rate * db;

        if (epoch + 1) % 200 == 0 {
            let mse = (0..x.len())
                .map(|i| (w * x[i] + b - y[i]).powi(2))
                .sum::<f64>() / m;
            println!("Epoch {}, MSE: {:.4}, w: {:.4}, b: {:.4}",
                    epoch + 1, mse, w, b);
        }
    }

    let duration = start.elapsed();

    println!("\nModelo entrenado:");
    println!("w ≈ {:.4}, b ≈ {:.4}", w, b);
    println!("Tiempo Rust: {:?}", duration);

    let x_nuevo = 7.0;
    println!("Para x = {}, y_pred ≈ {:.4}", x_nuevo, w * x_nuevo + b);
}