use rig::providers::openai;
use rig::completion::Prompt;
use serde::{Deserialize, Serialize};
use std::error::Error;
use plotters::prelude::*;

// System simulation
struct System {
    position: f64,
    velocity: f64,
}

impl System {
    fn new() -> Self {
        System {
            position: 0.0,
            velocity: 0.0,
        }
    }

    fn update(&mut self, force: f64, dt: f64) {
        let acceleration = force - 0.1 * self.velocity - 2.0 * self.position;
        self.velocity += acceleration * dt;
        self.position += self.velocity * dt;
    }
}

// PID Controller
struct PIDController {
    kp: f64,
    ki: f64,
    kd: f64,
    integral: f64,
    prev_error: f64,
}

impl PIDController {
    fn new(kp: f64, ki: f64, kd: f64) -> Self {
        PIDController {
            kp,
            ki,
            kd,
            integral: 0.0,
            prev_error: 0.0,
        }
    }

    fn calculate(&mut self, setpoint: f64, current_value: f64, dt: f64) -> f64 {
        let error = setpoint - current_value;
        self.integral += error * dt;
        let derivative = (error - self.prev_error) / dt;
        let output = self.kp * error + self.ki * self.integral + self.kd * derivative;
        self.prev_error = error;
        output
    }
}

// Performance metrics calculation
fn calculate_performance_metrics(response: &[f64], setpoint: f64, dt: f64) -> (f64, f64, f64) {
    let steady_state_error = (response.last().unwrap() - setpoint).abs();
    
    let mut max_overshoot = 0.0;
    for &value in response.iter() {
        let overshoot = (value - setpoint).abs();
        if overshoot > max_overshoot {
            max_overshoot = overshoot;
        }
    }
    
    let settling_time = response.len() as f64 * dt;  // Simplified

    (settling_time, max_overshoot, steady_state_error)
}

#[derive(Debug, Serialize, Deserialize)]
struct PIDParams {
    kp: f64,
    ki: f64,
    kd: f64,
}

fn generate_chart(
    responses: &[Vec<f64>],
    iteration: usize,
    pid_params: &[PIDParams],
    file_name: &str,
) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(file_name, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(format!("System Response - Iteration {}", iteration), ("sans-serif", 30).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f32..10f32, -0.5f32..1.5f32)?;

    chart.configure_mesh().draw()?;

    let colors = [RED, BLUE, GREEN, CYAN, MAGENTA, YELLOW];

    for (i, response) in responses.iter().enumerate() {
        let color = colors[i % colors.len()];
        chart.draw_series(LineSeries::new(
            response.iter().enumerate().map(|(x, y)| (x as f32 / 100.0, *y as f32)),
            color,
        ))?
        .label(format!("Iteration {} (Kp={:.2}, Ki={:.2}, Kd={:.2})",
                       i, pid_params[i].kp, pid_params[i].ki, pid_params[i].kd))
        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));
    }

    chart.configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let openai_client = openai::Client::from_env();
    let ai_tuner = openai_client.model("gpt-4").build();

    let mut all_responses = Vec::new();
    let mut all_pid_params = Vec::new();

    let setpoint = 1.0;
    let dt = 0.01;
    let simulation_steps = 1000;

    let mut pid = PIDController::new(1.0, 0.1, 0.05);  // Initial parameters
    all_pid_params.push(PIDParams { kp: pid.kp, ki: pid.ki, kd: pid.kd });

    for iteration in 0..20 {  // Reduced to 5 iterations for brevity
        let mut system = System::new();
        let mut response = Vec::new();

        // Run simulation
        for _ in 0..simulation_steps {
            let control_signal = pid.calculate(setpoint, system.position, dt);
            system.update(control_signal, dt);
            response.push(system.position);
        }

        all_responses.push(response.clone());

        let (settling_time, max_overshoot, steady_state_error) = 
            calculate_performance_metrics(&response, setpoint, dt);

        println!("Iteration {}: ST = {:.2}, MO = {:.2}, SSE = {:.4}", 
                 iteration, settling_time, max_overshoot, steady_state_error);

        // Generate chart for this iteration
        generate_chart(&all_responses, iteration, &all_pid_params, 
                       &format!("system_response_iteration_{}.png", iteration))?;

        // Ask AI to suggest new PID parameters
        let prompt = format!(
            "Current PID parameters: Kp = {:.2}, Ki = {:.2}, Kd = {:.2}\n\
            Performance metrics:\n\
            Settling Time: {:.2}\n\
            Max Overshoot: {:.2}\n\
            Steady State Error: {:.4}\n\
            Suggest new PID parameters to improve performance. \
            Respond with a JSON object containing 'kp', 'ki', and 'kd' fields.",
            pid.kp, pid.ki, pid.kd, settling_time, max_overshoot, steady_state_error
        );

        let ai_response = ai_tuner.prompt(&prompt).await?;
        let new_params: PIDParams = serde_json::from_str(&ai_response)?;

        // Update PID parameters
        pid = PIDController::new(new_params.kp, new_params.ki, new_params.kd);
        all_pid_params.push(new_params);
    }

    // Generate final overlay chart
    generate_chart(&all_responses, all_responses.len() - 1, &all_pid_params, "system_response_overlay.png")?;

    Ok(())
}