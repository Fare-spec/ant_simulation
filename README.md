# Ant Colony Simulation 🐜

This project is a **Rust-based simulation** of an ant colony moving on a grid. The goal is to simulate the behavior of ants, including finding food, returning it to the nest, and exploring the grid. It is designed as a learning challenge to explore Rust's core concepts such as enums, vectors, and basic simulation logic.

---

## Project Goals 🎯

The main objectives of this project are:
1. Create a 2D grid where each cell can represent different states:
   - Empty
   - Nest (the ants' starting point)
   - Food (resources to collect)
   - Ant (representing the ants on the grid)
2. Simulate the movement of ants across the grid.
3. Implement logic for ants to:
   - Search for food.
   - Return food to the nest.
   - Move randomly if no food is found.
4. Display the grid in the console to visualize the simulation.

---

## Current Features ✅

1. **Grid Creation:**
   - A square grid of customizable size is initialized, with all cells set to `Empty` by default.

2. **Grid Display:**
   - The grid is displayed in the console, using symbols to represent different cell types:
     - `.` : Empty cell
     - `N` : Nest
     - `F` : Food
     - `A` : Ant

---

## Planned Features 🔄

The following features are planned to complete the simulation:
1. Allow ants to move on the grid based on simple rules (e.g., random movement, towards food).
2. Add functionality to place food and nests at specific positions.
3. Implement pheromone trails that guide ants towards food and back to the nest.
4. Add a step-by-step simulation loop to observe the ants' behavior over time.
5. Track metrics like the total amount of food collected.

---

## Code Structure 🛠️

- **`main.rs`**: The main entry point of the program, responsible for:
  - Creating the grid.
  - Displaying the grid.
  - Handling the logic for modifying cells and running the simulation.
- **`Cell` Enum**: Defines the possible states for each cell in the grid:
  - `Empty`: A cell with no content.
  - `Nest`: The ants' starting point.
  - `Food`: A resource that ants can collect.
  - `Ant`: Represents an ant on the grid.

---

## How to Run 🚀

### Prerequisites

You need to have **Rust** installed on your machine. If you don't have it yet, install it with [rustup](https://www.rust-lang.org/tools/install).

### Steps

1. Clone this repository:
   ```bash
   git clone https://github.com/your-username/ant-colony-simulation.git
   cd ant-colony-simulation
   ```

2. Compile and run the project:
   ```bash
   cargo run
   ```

3. Observe the grid displayed in the console. Future updates will include step-by-step simulation.

---

## Contribution 🤝

This project is designed as a challenge, and contributions are welcome! Here are a few ways you can contribute:
1. Add features to improve the simulation (e.g., ant movement logic, pheromone trails).
2. Optimize the existing code for performance and readability.
3. Suggest new ideas to make the simulation more realistic or engaging.

To contribute, feel free to open an **issue** or submit a **pull request**.

---

## License 📜

This project is licensed under the [MIT License](https://opensource.org/licenses/MIT). You are free to use, modify, and distribute it as you like.

---

## Notes 📝

This is a work in progress and is intentionally incomplete to encourage creativity and exploration of Rust's features.
