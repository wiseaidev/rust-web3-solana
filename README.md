# 📚 Rust Web3 Solana

[![Maintenance](https://img.shields.io/badge/Maintained%3F-yes-green.svg)](https://github.com/wiseaidev)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Jupyter Notebook](https://img.shields.io/badge/Jupyter-Notebook-blue.svg?logo=Jupyter&logoColor=orange)](https://jupyter.org/)

Welcome to the Rust Web3 Solana repository! This collection of Jupyter notebooks provides an extensive tutorial on using Rust for Solana development. I have compiled these resources while preparing for a senior blockchain & solana developer role interviews, aiming to create a hands-on tutorial for utilizing Rust in Solana development through Jupyter notebooks. This repository houses a wide range of topics covering web3 in general and Solana in particular.

## 📝 Table of Contents

- [Installation](#-installation)
- [Tutorials](#-tutorials)
- [Contributing](#-contributing)
- [Licence](#-licence)
- [Star History](#-star-history)

## 🚀 Installation

To use the notebooks in this repository, you need to set up your environment. Follow these steps to get started:

1. Clone the repository to your local machine:

	```sh
	git clone https://github.com/wiseaidev/rust-web3-solana.git
	```

1. Install the required dependencies and libraries. Make sure you have [`Rust`](https://rustup.rs/), [`Jupyter Notebook`](https://jupyter.org/install), and [`evcxr_jupyter`](https://github.com/evcxr/evcxr/blob/main/evcxr_jupyter/README.md) installed on your system.

	```sh
	# Install a Rust toolchain (e.g., nightly):
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly

	# Install Jupyter Notebook
	pip install notebook

	# Install evcxr_jupyter
	cargo install evcxr_jupyter
	evcxr_jupyter --install	
	```

1. Navigate to the cloned repository:

	```sh
	cd rust-web3-solana
	```

1. Start Jupyter Notebook:

	```sh
	jupyter notebook
	```

1. Access the notebooks in your web browser by clicking on the notebook file you want to explore.

## 📌 Chapters

| ID | Title | NB Pages | Topics | Open on GitHub | Launch on Binder | Launch on Colab | Read PDF |
|----|---------------|-----------|:-------------|-------------|----------------|-------|-------|
| 1  | **Web3 Solana Basics** | 21+ | - Cryptographic Key Pairs <br>- Digital Wallets <br>- Solana Accounts <br>- Smart Contracts <br>- Instructions and Transactions <br>- The Architecture of the Solana Blockchain <br>- Communicating with Solana through JSON RPC <br>- Proof of History (POH) | [![Github](https://img.shields.io/badge/launch-Github-181717.svg?logo=github&logoColor=white)](./chapter-1/chapter-1.ipynb) | [![Binder](https://mybinder.org/badge_logo.svg)](https://mybinder.org/v2/gh/wiseaidev/rust-web3-solana/main?filepath=chapter-1/chapter-1.ipynb) | [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/wiseaidev/rust-web3-solana/blob/main/chapter-1/chapter-1.ipynb) | [![nbviewer](https://img.shields.io/badge/Read%20PDF-nbviewer-blue)](https://nbviewer.org/github/wiseaidev/rust-web3-solana/tree/main/chapter-1/chapter-1.pdf) |

## 🤝 Contributing

We welcome contributions to enhance the Rust Web3 Solana repository! To contribute, please follow the [`CONTRIBUTING.md`](CONTRIBUTING.md) file guidelines. Thank you for helping make this project better!

## 📜 License

This project is licensed under the [MIT](https://opensource.org/licenses/MIT). For more details, You can refer to the [LICENSE](LICENSE) file.

## 📈 Star History

[![Star History Chart](https://api.star-history.com/svg?repos=wiseaidev/rust-web3-solana&type=Date)](https://star-history.com/#wiseaidev/rust-web3-solana&Date)