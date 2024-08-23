# Adaptive PID Controller Tuner using [Rig](https://github.com/0xPlaygrounds/rig)

This project demonstrates how to leverage [Rig](https://github.com/0xPlaygrounds/rig), a powerful Rust library for building LLM-powered applications, to create an AI agent that tunes a PID controller. Whether you're new to control systems or looking to explore AI-enhanced engineering applications, this example provides an excellent starting point.

### What is a PID Controller?

Before we dive in, let's briefly explain what a PID controller is:

A PID (Proportional-Integral-Derivative) controller is a control loop mechanism widely used in industrial systems. It continuously calculates an error value as the difference between a desired setpoint and a measured process variable and applies a correction based on proportional, integral, and derivative terms.

Imagine you're driving a car and trying to maintain a constant speed:
- The Proportional term is like your immediate response to speed changes.
- The Integral term is like your memory of past errors, helping eliminate persistent offsets.
- The Derivative term is like your anticipation of future changes based on the rate of change.

Tuning these three parameters (Kp, Ki, Kd) is crucial for optimal system performance.

### Prerequisites

Before you begin, make sure you have the following installed:

- Rust (latest stable version)
- Cargo (Rust's package manager)

You'll also need an OpenAI API key. If you don't have one, you can sign up at [OpenAI's website](https://openai.com).

### Setup

1. Create a new Rust project:
   ```
   cargo new rig-pid-tuner
   cd rig-pid-tuner
   ```

2. Add the following dependencies to your `Cargo.toml`:
   ```toml
   [dependencies]
   rig-core = "0.1.0"
   serde = { version = "1.0", features = ["derive"] }
   serde_json = "1.0"
   tokio = { version = "1.0", features = ["full"] }
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
5. A main loop simulating the system and allowing the AI to tune the controller.

### Running the Example

1. Copy the provided code into your `src/main.rs` file.
2. Run the example using:
   ```
   cargo run
   ```

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

5. **Main Loop**:
   In the main function, we run multiple iterations of:
   - Simulating the system
   - Calculating performance metrics
   - Using the AI to suggest new PID parameters
   - Updating the controller with the new parameters

### Customization

Feel free to modify the `System` struct to simulate different types of systems, or adjust the performance metric calculations to focus on different aspects of system performance.

### Troubleshooting

If you encounter any issues:
- Ensure your OpenAI API key is correctly set.
- Check that all dependencies are properly installed.
- Verify that you're using a compatible Rust version.

For more detailed information, refer to the [Rig documentation](https://docs.rs/rig).

