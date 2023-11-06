struct linear_regression {
    x: Vec<[f64; 1]>,
    y: Vec<[f64; 1]>,
    m: f64,
    b: f64
}

impl linear_regression {
    fn new(x: Vec<[f64; 1]>, y: Vec<[f64; 1]>) -> linear_regression {
        linear_regression { x, y, m: 5.0, b: 7.2 }
    }

    fn loss(&self) -> f64 {
        let mut loss: f64 = 0.0;
        let n: usize = self.x.len();
        for i in 0..n {
            let predicted_y: f64 = self.m * self.x[i][0] + self.b;
            loss += (predicted_y - self.y[i][0]).powi(2);
        }
        loss /= n as f64;

        loss
    }

    fn fit(&mut self, epochs: i32, alpha: f64) {
        let n: usize = self.x.len();
        for epoch in 0..epochs {
            let mut gradient_m: f64 = 0.0;
            let mut gradient_b: f64 = 0.0;

            for i in 0..n {
                let predicted_y = self.m * self.x[i][0] + self.b;
                gradient_m += 2.0 * (predicted_y - self.y[i][0]) * self.x[i][0];
                gradient_b += 2.0 * (predicted_y - self.y[i][0]);
            }

            gradient_m /= n as f64;
            gradient_b /= n as f64;

            self.m -= alpha * gradient_m;
            self.b -= alpha * gradient_b;

            println!("# {} Loss: {}", epoch+1, self.loss())
        }
    }
}

fn example_usecase() {
    let mut x: Vec<[f64; 1]> = vec![];
    let mut y: Vec<[f64; 1]> = vec![];

    for i in 0..50 {
        x.push([(i) as f64]);
        y.push([(7*i + 3) as f64]);
    }

    let mut model: linear_regression = linear_regression::new(x, y);

    model.fit(10000, 1e-3);

    println!("m: {}, b: {}", model.m, model.b);
}