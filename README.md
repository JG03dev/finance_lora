# Finance_LoRA

Finance_LoRA is a full-stack web application designed to serve as a chatbot for financial queries. 

## Tech Stack

- **Frontend**: [Yew.rs](https://yew.rs/) - Rust framework for creating multi-threaded front-end web apps.
- **Frontend Bundler**: [Trunk](https://trunkrs.dev/) - Tool to build and ship Rust Wasm applications.
- **Backend**: [Actix Web](https://actix.rs/) - Powerful, pragmatic, and extremely fast web framework for Rust.
- **Fine-tuned Models**: [evcxr](https://github.com/google/evcxr) - Rust Jupyter kernel.

## Prerequisites

- **Rust**: Ensure Rust is installed. If not, you can install it from [here](https://www.rust-lang.org/learn/get-started).
- **Trunk**: Install Trunk using `cargo install trunk`.
- **Node.js**: Required for Trunk. Install from [here](https://nodejs.org/).
- **Python**: Required for evcxr. Install from [here](https://www.python.org/).
- **evcxr**: Install evcxr using `cargo install evcxr`.

In order for the backend to work, create a `/backend/.env` file in the root directory and add your Hugging Face API key:

```
HUGGING_FACE_API_KEY=hf_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/JG03dev/finance_lora.git
   cd finance_lora
   ```

## Running the Application

### Backend

1. Navigate to the backend directory:
   ```bash
   cd backend
   ```

2. Run the backend server:
   ```bash
   cargo run
   ```
### Frontend

1. Navigate and build the project:
   ```bash
   cd frontend
   trunk build
   ```

2. Serve the frontend application:
   ```bash
   trunk serve
   ```

### Fine-tuned Models

1. Run the evcxr Jupyter kernel:
   ```bash
   evcxr_jupyter --install
   ```

2. Start Jupyter Notebook and open the relevant notebooks for the models:
   ```bash
   jupyter notebook
   ```