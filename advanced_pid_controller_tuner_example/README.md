# Adaptive PID Controller Tuner with Charts using Rig

## README

### Introduction

Welcome to the Adaptive PID Controller Tuner with Charts example using Rig! This project demonstrates how to leverage Rig, a powerful Rust library for building LLM-powered applications, to create an AI agent that tunes a PID controller. We've enhanced this example with visual feedback, allowing you to see the impact of AI-suggested tuning in real-time. Whether you're new to control systems or looking to explore AI-enhanced engineering applications, this example provides an excellent starting point.

### What is a PID Controller?

Before we dive in, let's explain what a PID controller is:

A PID (Proportional-Integral-Derivative) controller is a control loop mechanism widely used in industrial systems. It continuously calculates an error value as the difference between a desired setpoint and a measured process variable and applies a correction based on proportional, integral, and derivative terms.

Imagine you're trying to maintain a constant water level in a tank:
- The Proportional term (P) is like how quickly you open or close the tap based on how far the water level is from your target.
- The Integral term (I) is like your memory of past errors, helping you make fine adjustments if the level has been consistently off.
- The Derivative term (D) is like your anticipation of future changes based on how quickly the water level is changing.

Tuning these three parameters (Kp, Ki, Kd) is crucial for optimal system performance, which is where our AI comes in!

### Prerequisites

Before you begin, make sure you have the following installed:

- Rust (latest stable version)
- Cargo (Rust's package manager)

You'll also need an OpenAI API key. If you don't have one, you can sign up at [OpenAI's website](https://openai.com).

### Setup

1. Create a new Rust project:
   ```
   cargo new rig-pid-tuner-charts
   cd rig-pid-tuner-charts
   ```

2. Add the following dependencies to your `Cargo.toml`:
   ```toml
   [dependencies]
   rig-core = "0.1.0"
   serde = { version = "1.0", features = ["derive"] }
   serde_json = "1.0"
   tokio = { version = "1.0", features = ["full"] }
   plotters = "0.3"
   ```

3. Set your OpenAI API key as an environment variable:
   ```
   export OPENAI_API_KEY=your_api_key_here
   ```

### Code Overview

The main components of this example are:

1. `System`: A struct simulating a simple second-order system.
2. `PIDController`: A struct implementing a basic PID controller.
3. Performance metric calculations (settling time, overshoot, steady-state error).
4. An AI agent using Rig to suggest PID parameter improvements.
5. A charting function to visualize system responses.
6. A main loop simulating the system, allowing the AI to tune the controller, and generating charts.

### Running the Example

1. Copy the provided code into your `src/main.rs` file.
2. Run the example using:
   ```
   cargo run
   ```
3. After running, you'll find PNG images in your project directory showing the system responses for each iteration and a final overlay chart.

### Understanding the Code

Let's break down the key parts of the code:

1. **System Simulation**: 
   We simulate a simple second-order system. Think of this as a simplified model of a physical system, like a spring-mass-damper system.

   ```rust
   struct System {
       position: f64,
       velocity: f64,
   }
   ```

2. **PID Controller**:
   This struct implements the PID control algorithm. It calculates the control output based on the error between the setpoint and the current value.

   ```rust
   struct PIDController {
       kp: f64,
       ki: f64,
       kd: f64,
       integral: f64,
       prev_error: f64,
   }
   ```

3. **Performance Metrics**:
   We calculate three key metrics:
   - Settling Time: How long it takes for the system to reach and stay within a certain range of the setpoint.
   - Max Overshoot: The maximum amount the system exceeds the setpoint.
   - Steady-State Error: The final difference between the system's output and the setpoint.

4. **AI Tuner**:
   We use Rig to create an AI agent that suggests improvements to the PID parameters based on the current performance metrics.

   ```rust
   let ai_tuner = openai_client.model("gpt-4").build();
   ```

5. **Charting Function**:
   We use the `plotters` library to generate visual representations of our system's response. This function creates charts for each iteration and a final overlay chart.

   ```rust
   fn generate_chart(
       responses: &[Vec<f64>],
       iteration: usize,
       pid_params: &[PIDParams],
       file_name: &str,
   ) -> Result<(), Box<dyn Error>> {
       // ... (chart generation code)
   }
   ```

6. **Main Loop**:
   In the main function, we run multiple iterations of:
   - Simulating the system
   - Calculating performance metrics
   - Generating a chart of the system response
   - Using the AI to suggest new PID parameters
   - Updating the controller with the new parameters

   After all iterations, we generate a final overlay chart showing all system responses.

### Interpreting the Results

The generated charts provide a visual representation of how the system's response changes as the PID parameters are tuned. Look for:

- Faster settling times (the system reaches the setpoint more quickly)
- Reduced overshoot (the system doesn't go as far past the setpoint)
- Smaller steady-state error (the final position is closer to the setpoint)

The overlay chart allows you to compare all iterations side-by-side, clearly showing the improvement in system performance over time.

### Customization

Feel free to modify the `System` struct to simulate different types of systems, adjust the performance metric calculations, or change the number of iterations. You can also experiment with different chart styles or additional visualizations.

### Troubleshooting

If you encounter any issues:
- Ensure your OpenAI API key is correctly set.
- Check that all dependencies are properly installed.
- Verify that you're using a compatible Rust version.
- If charts aren't generating, ensure you have write permissions in the project directory.

For more detailed information, refer to the [Rig documentation](https://docs.rs/rig).