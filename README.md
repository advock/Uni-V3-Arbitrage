Uni-V3-Arbitrage
Uni-V3-Arbitrage is a Rust-based project that facilitates arbitrage trading on Uniswap V3. The project includes tools for fetching pool data, managing configurations, and simulating blockchain states.

Table of Contents
Installation
Usage
Configuration
Modules
Contributing
License
Installation
Prerequisites
Rust: Make sure you have Rust installed. If not, you can download it from rust-lang.org.
Steps
Clone the repository:

bash
Copy code
git clone https://github.com/advock/Uni-V3-Arbitrage.git
cd Uni-V3-Arbitrage
Build the project:

bash
Copy code
cargo build --release
Set up environment variables:

Create a .env file in the project root with the following content:

env
Copy code
NETWORK_HTTP=<your-http-provider>
NETWORK_WSS=<your-websocket-provider>
NETWORK_WSS_LOGS=<your-wss-logs-provider>
PRIVATE_KEY=<your-private-key>
GRAPH_API=<your-graph-api-url>
Usage
To run the project, use the following command:

bash
Copy code
cargo run --release
This will execute the main function, which fetches the list of Uniswap V3 pools and saves the data to a file named pools_output.

Configuration
The project configuration is handled in the src/config.rs file. It sets up HTTP and WebSocket providers, and initializes the wallet and graph URL from environment variables.

Modules
Main Modules
main.rs: Entry point of the application. Sets up the environment and calls the get_pools_list function.
lib.rs: Declares the modules used in the project.
Configuration and Constants
config.rs: Defines the configuration structure and initializes providers and wallets.
constants.rs: Contains various constants like addresses and Uniswap V3 configurations.
Contract Modules and Data Collection
contract_modules/UniV3/data_collector/collector.rs: Data collector for UniV3 pools.
contract_modules/UniV3/data_collector/mod.rs: Module declaration for data collector.
contract_modules/UniV3/mod.rs: Module declaration for UniV3.
contract_modules/UniV3/types.rs: Defines the structure for UniV3 pools and related data.
Fork Simulator
fork/simulator/data_base_errors.rs: Defines errors related to the database.
fork/simulator/fork_db.rs: Implements the ForkDB structure.
fork/simulator/fork_factory.rs: Implements the ForkFactory structure.
fork/simulator/global_backend.rs: Handles the global backend for fetching data.
fork/simulator/mod.rs: Module declaration for the simulator.
Helper and State Management
helper.rs: Helper functions for address conversion and binding.
state.rs: Manages the state, including indexed pairs and cycles.
Pool Getter and Updater
uniV3PoolGetter.rs: Fetches the list of pools from the GraphQL API and saves them to a file.
updater.rs: Contains functions for starting the updater and updating blocks.
Contributing
Contributions are welcome! Please fork the repository and submit pull requests for any enhancements or bug fixes.

License
This project is licensed under the MIT License. See the LICENSE file for details.
