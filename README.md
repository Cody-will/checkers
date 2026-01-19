<div id="top">

<!-- HEADER STYLE: CLASSIC -->
<div align="center">


# CHECKERS

<em>Master the game, conquer with confidence</em>

<!-- BADGES -->
<img src="https://img.shields.io/github/last-commit/Cody-will/checkers?style=flat&logo=git&logoColor=white&color=0080ff" alt="last-commit">
<img src="https://img.shields.io/github/languages/top/Cody-will/checkers?style=flat&color=0080ff" alt="repo-top-language">
<img src="https://img.shields.io/github/languages/count/Cody-will/checkers?style=flat&color=0080ff" alt="repo-language-count">

<em>Built with the tools and technologies:</em>

<img src="https://img.shields.io/badge/Rust-000000.svg?style=flat&logo=Rust&logoColor=white" alt="Rust">
<img src="https://img.shields.io/badge/TOML-9C4121.svg?style=flat&logo=TOML&logoColor=white" alt="TOML">
<img src="https://img.shields.io/badge/Slint-2379F4.svg?style=flat&logo=Slint&logoColor=white" alt="Slint">

</div>
<br>

---

## Table of Contents

- [Overview](#overview)
- [Getting Started](#getting-started)
    - [Prerequisites](#prerequisites)
    - [Installation](#installation)
    - [Usage](#usage)
    - [Testing](#testing)
- [Features](#features)
- [Project Structure](#project-structure)
    - [Project Index](#project-index)

---

## Overview



---

## Features

|      | Component       | Details                                                                                     |
| :--- | :-------------- | :------------------------------------------------------------------------------------------ |
| ⚙️  | **Architecture**  | <ul><li>Client-Server Model with Rust backend and Slint UI components</li><li>Modular separation between UI (Slint files) and core logic (Rust)</li><li>Event-driven communication via message passing</li></ul> |
| 🔩 | **Code Quality**  | <ul><li>Consistent Rust idioms, leveraging ownership and concurrency features</li><li>Clear separation of concerns</li><li>Use of Cargo for dependency management and build</li></ul> |
| 📄 | **Documentation** | <ul><li>Basic README with project overview</li><li>Configuration details in `Cargo.toml` and `Cargo.lock`</li><li>UI layouts defined in `.slint` files</li></ul> |
| 🔌 | **Integrations**  | <ul><li>UI built with Slint, integrated into Rust via `slint` crate</li><li>Input handling with `rdev` for event capture</li><li>Build and package managed via Cargo</li></ul> |
| 🧩 | **Modularity**    | <ul><li>UI components separated into individual `.slint` files (`app.slint`, `player.slint`, `board.slint`, etc.)</li><li>Core game logic encapsulated in Rust modules</li></ul> |
| 🧪 | **Testing**       | <ul><li>Limited info, likely unit tests in Rust (`cargo test`)</li><li>UI tests not explicitly documented</li></ul> |
| ⚡️  | **Performance**   | <ul><li>Rust's ownership model ensures minimal runtime overhead</li><li>UI rendering optimized via Slint's efficient rendering engine</li></ul> |
| 🛡️ | **Security**      | <ul><li>Rust's safety guarantees prevent common bugs</li><li>No explicit security features documented</li></ul> |
| 📦 | **Dependencies**  | <ul><li>Core dependencies include `slint`, `rust`, `rdev`</li><li>UI components in `.slint` files, managed via Cargo</li></ul> |

---

## Project Structure

```sh
└── checkers/
    ├── Cargo.lock
    ├── Cargo.toml
    ├── build.rs
    ├── src
    │   ├── game
    │   └── main.rs
    └── ui
        ├── app.slint
        ├── assets
        ├── board.slint
        ├── config.slint
        ├── game.slint
        ├── lobby.slint
        ├── player.slint
        └── structs.slint
```

---

### Project Index

<details open>
	<summary><b><code>CHECKERS/</code></b></summary>
	<!-- __root__ Submodule -->
	<details>
		<summary><b>__root__</b></summary>
		<blockquote>
			<div class='directory-path' style='padding: 8px 0; color: #666;'>
				<code><b>⦿ __root__</b></code>
			<table style='width: 100%; border-collapse: collapse;'>
			<thead>
				<tr style='background-color: #f8f9fa;'>
					<th style='width: 30%; text-align: left; padding: 8px;'>File Name</th>
					<th style='text-align: left; padding: 8px;'>Summary</th>
				</tr>
			</thead>
				<tr style='border-bottom: 1px solid #eee;'>
					<td style='padding: 8px;'><b><a href='https://github.com/Cody-will/checkers/blob/master/Cargo.toml'>Cargo.toml</a></b></td>
					<td style='padding: 8px;'>- Defines project metadata and dependencies for the checkers application, establishing core configurations and external libraries needed for cross-platform GUI development and input handling<br>- Serves as the foundation for building and managing the applications structure, ensuring proper setup of tools and resources essential for the overall architecture.</td>
				</tr>
				<tr style='border-bottom: 1px solid #eee;'>
					<td style='padding: 8px;'><b><a href='https://github.com/Cody-will/checkers/blob/master/build.rs'>build.rs</a></b></td>
					<td style='padding: 8px;'>- Facilitates the build process by compiling the Slint UI definition into executable code, ensuring the user interface is integrated into the application<br>- It manages dependencies between source files and triggers recompilation when UI files change, supporting seamless updates and efficient development within the overall project architecture.</td>
				</tr>
			</table>
		</blockquote>
	</details>
	<!-- src Submodule -->
	<details>
		<summary><b>src</b></summary>
		<blockquote>
			<div class='directory-path' style='padding: 8px 0; color: #666;'>
				<code><b>⦿ src</b></code>
			<table style='width: 100%; border-collapse: collapse;'>
			<thead>
				<tr style='background-color: #f8f9fa;'>
					<th style='width: 30%; text-align: left; padding: 8px;'>File Name</th>
					<th style='text-align: left; padding: 8px;'>Summary</th>
				</tr>
			</thead>
				<tr style='border-bottom: 1px solid #eee;'>
					<td style='padding: 8px;'><b><a href='https://github.com/Cody-will/checkers/blob/master/src/main.rs'>main.rs</a></b></td>
					<td style='padding: 8px;'>- Orchestrates the applications user interface and state transitions, managing the flow between lobby and game views<br>- Facilitates user interactions such as starting a game, making moves, and quitting, while synchronizing game state with the UI<br>- Serves as the central controller that integrates UI components with core game logic within the overall architecture.</td>
				</tr>
			</table>
			<!-- game Submodule -->
			<details>
				<summary><b>game</b></summary>
				<blockquote>
					<div class='directory-path' style='padding: 8px 0; color: #666;'>
						<code><b>⦿ src.game</b></code>
					<table style='width: 100%; border-collapse: collapse;'>
					<thead>
						<tr style='background-color: #f8f9fa;'>
							<th style='width: 30%; text-align: left; padding: 8px;'>File Name</th>
							<th style='text-align: left; padding: 8px;'>Summary</th>
						</tr>
					</thead>
						<tr style='border-bottom: 1px solid #eee;'>
							<td style='padding: 8px;'><b><a href='https://github.com/Cody-will/checkers/blob/master/src/game/types.rs'>types.rs</a></b></td>
							<td style='padding: 8px;'>- Defines core data structures and enums for managing game state, player interactions, and board representation in a checkers game<br>- Facilitates game flow control, move application, and UI state synchronization, ensuring seamless integration between game logic and user interface within the overall architecture<br>- Supports game progression, player turns, and move validation across the codebase.</td>
						</tr>
						<tr style='border-bottom: 1px solid #eee;'>
							<td style='padding: 8px;'><b><a href='https://github.com/Cody-will/checkers/blob/master/src/game/mod.rs'>mod.rs</a></b></td>
							<td style='padding: 8px;'>- Defines the core modular structure of the game, organizing key components such as the game board, player interactions, game rules, and user interface types<br>- It serves as the central hub that integrates various aspects of the game, facilitating seamless interaction and coordination among different modules to support the overall game architecture.</td>
						</tr>
						<tr style='border-bottom: 1px solid #eee;'>
							<td style='padding: 8px;'><b><a href='https://github.com/Cody-will/checkers/blob/master/src/game/navigation.rs'>navigation.rs</a></b></td>
							<td style='padding: 8px;'>- Defines the navigation logic for managing different views within the game, primarily focusing on initializing and transitioning between the lobby and other views<br>- Facilitates seamless view state management, ensuring the game can accurately track and switch user interfaces during gameplay<br>- Serves as a core component for maintaining consistent user experience across the applications navigation flow.</td>
						</tr>
						<tr style='border-bottom: 1px solid #eee;'>
							<td style='padding: 8px;'><b><a href='https://github.com/Cody-will/checkers/blob/master/src/game/player.rs'>player.rs</a></b></td>
							<td style='padding: 8px;'>- Defines the Player entity within the game architecture, encapsulating player identity, assigned side, and captured pieces<br>- Facilitates tracking each players progress and status, including capturing game pieces and determining the winner<br>- Serves as a core component for managing player state and interactions, supporting game flow and outcome evaluation within the overall system.</td>
						</tr>
						<tr style='border-bottom: 1px solid #eee;'>
							<td style='padding: 8px;'><b><a href='https://github.com/Cody-will/checkers/blob/master/src/game/rules.rs'>rules.rs</a></b></td>
							<td style='padding: 8px;'>- Defines core move logic for a checkers game, including move generation, capturing rules, and forced move states<br>- Facilitates gameplay by determining valid slides and jumps based on piece type and position, ensuring adherence to game rules and capturing mechanics within the overall game architecture.</td>
						</tr>
						<tr style='border-bottom: 1px solid #eee;'>
							<td style='padding: 8px;'><b><a href='https://github.com/Cody-will/checkers/blob/master/src/game/ui_types.rs'>ui_types.rs</a></b></td>
							<td style='padding: 8px;'>- Defines data transformation and UI model mappings for the game interface, translating core game state and player data into UI-compatible formats<br>- Facilitates seamless synchronization between game logic and visual presentation, enabling dynamic updates of game elements such as the board, players, and game status within the user interface layer.</td>
						</tr>
						<tr style='border-bottom: 1px solid #eee;'>
							<td style='padding: 8px;'><b><a href='https://github.com/Cody-will/checkers/blob/master/src/game/board.rs'>board.rs</a></b></td>
							<td style='padding: 8px;'>- Defines the game board structure for a checkers game, initializing an 8x8 grid with appropriately placed black and red pieces<br>- Provides functionality to access individual square states and to flatten the board into a linear representation, supporting game state management and move validation within the overall architecture.</td>
						</tr>
						<tr style='border-bottom: 1px solid #eee;'>
							<td style='padding: 8px;'><b><a href='https://github.com/Cody-will/checkers/blob/master/src/game/play.rs'>play.rs</a></b></td>
							<td style='padding: 8px;'>- Defines core game logic for managing state, moves, and win conditions in a checkers game<br>- Handles player interactions, move validation, capturing, king promotion, turn switching, and game outcome determination, integrating with the UI for real-time updates<br>- Serves as the central engine orchestrating gameplay flow within the overall architecture.</td>
						</tr>
					</table>
				</blockquote>
			</details>
		</blockquote>
	</details>
	<!-- ui Submodule -->
	<details>
		<summary><b>ui</b></summary>
		<blockquote>
			<div class='directory-path' style='padding: 8px 0; color: #666;'>
				<code><b>⦿ ui</b></code>
			<table style='width: 100%; border-collapse: collapse;'>
			<thead>
				<tr style='background-color: #f8f9fa;'>
					<th style='width: 30%; text-align: left; padding: 8px;'>File Name</th>
					<th style='text-align: left; padding: 8px;'>Summary</th>
				</tr>
			</thead>
				<tr style='border-bottom: 1px solid #eee;'>
					<td style='padding: 8px;'><b><a href='https://github.com/Cody-will/checkers/blob/master/ui/structs.slint'>structs.slint</a></b></td>
					<td style='padding: 8px;'>- Defines core data structures and enumerations for the user interface, representing game states, player details, board positions, and view management<br>- Facilitates seamless communication between UI components and game logic, enabling dynamic rendering of game status, player actions, and visual updates within the overall architecture<br>- Ensures consistent data handling across the applications visual and interactive layers.</td>
				</tr>
				<tr style='border-bottom: 1px solid #eee;'>
					<td style='padding: 8px;'><b><a href='https://github.com/Cody-will/checkers/blob/master/ui/app.slint'>app.slint</a></b></td>
					<td style='padding: 8px;'>- Defines the main user interface window for the Checkers game, orchestrating view transitions between lobby and gameplay<br>- Manages layout, sizing, and user interactions such as starting a game, making moves, quitting, or returning to the lobby<br>- Serves as the central component connecting visual elements with game logic within the overall application architecture.</td>
				</tr>
				<tr style='border-bottom: 1px solid #eee;'>
					<td style='padding: 8px;'><b><a href='https://github.com/Cody-will/checkers/blob/master/ui/config.slint'>config.slint</a></b></td>
					<td style='padding: 8px;'>- Defines visual assets, themes, and color schemes to establish a consistent and customizable user interface for the application<br>- Facilitates centralized management of images and styling properties, ensuring cohesive appearance and easier updates across the entire UI architecture<br>- Supports the overall design system by providing reusable visual elements and theme configurations.</td>
				</tr>
				<tr style='border-bottom: 1px solid #eee;'>
					<td style='padding: 8px;'><b><a href='https://github.com/Cody-will/checkers/blob/master/ui/player.slint'>player.slint</a></b></td>
					<td style='padding: 8px;'>- Defines a visual component for displaying a players status and captured pieces within the game interface<br>- It visually presents the players name and a grid of captured game pieces, dynamically updating based on game state and turn<br>- Integrates seamlessly into the overall UI architecture, supporting real-time updates and consistent styling across the application.</td>
				</tr>
				<tr style='border-bottom: 1px solid #eee;'>
					<td style='padding: 8px;'><b><a href='https://github.com/Cody-will/checkers/blob/master/ui/game.slint'>game.slint</a></b></td>
					<td style='padding: 8px;'>- Defines the core user interface for the game, orchestrating layout, player interactions, and pause menu functionality<br>- Facilitates seamless gameplay experience by managing player input, rendering game components, and handling state transitions such as pausing, returning to lobby, or quitting<br>- Serves as the visual and interactive foundation within the overall architecture, connecting user actions to game logic.</td>
				</tr>
				<tr style='border-bottom: 1px solid #eee;'>
					<td style='padding: 8px;'><b><a href='https://github.com/Cody-will/checkers/blob/master/ui/lobby.slint'>lobby.slint</a></b></td>
					<td style='padding: 8px;'>- Defines the lobby interface for the application, providing users with options to start a new game or quit<br>- It displays the logo prominently and includes interactive buttons with hover effects, facilitating navigation within the apps user experience<br>- This component integrates visual branding with core user actions, forming a key entry point in the overall architecture.</td>
				</tr>
				<tr style='border-bottom: 1px solid #eee;'>
					<td style='padding: 8px;'><b><a href='https://github.com/Cody-will/checkers/blob/master/ui/board.slint'>board.slint</a></b></td>
					<td style='padding: 8px;'>- Defines the visual and interactive layout of an 8x8 game board, managing the rendering of tiles, highlighting selectable and selected positions, and displaying game pieces based on the current game state<br>- Facilitates user interactions by detecting clicks on tiles to trigger player moves, integrating seamlessly with the overall game logic and state management.</td>
				</tr>
			</table>
		</blockquote>
	</details>
</details>

---

## Getting Started

### Prerequisites

This project requires the following dependencies:

- **Programming Language:** Rust
- **Package Manager:** Cargo

### Installation

Build checkers from the source and install dependencies:

1. **Clone the repository:**

    ```sh
    ❯ git clone https://github.com/Cody-will/checkers
    ```

2. **Navigate to the project directory:**

    ```sh
    ❯ cd checkers
    ```

3. **Install the dependencies:**

**Using [cargo](https://www.rust-lang.org/):**

```sh
❯ cargo build
```

### Usage

Run the project with:

**Using [cargo](https://www.rust-lang.org/):**

```sh
cargo run
```

### Testing

Checkers uses the {__test_framework__} test framework. Run the test suite with:

**Using [cargo](https://www.rust-lang.org/):**

```sh
cargo test
```

---

<div align="left"><a href="#top">⬆ Return</a></div>

---
