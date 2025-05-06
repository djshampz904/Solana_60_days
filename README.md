# 🚀 Solana 60 Days Challenge

<div align="center">
  <img src="https://solana.com/_next/image?url=%2F_next%2Fstatic%2Fmedia%2Flogotype.e4df684f.svg&w=256&q=75" alt="Solana Logo" width="400"/>
  <p><em>My journey to becoming proficient in Solana blockchain development</em></p>
  
  ![Solana](https://img.shields.io/badge/Solana-black?style=for-the-badge&logo=solana&logoColor=white)
  ![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
  ![TypeScript](https://img.shields.io/badge/TypeScript-007ACC?style=for-the-badge&logo=typescript&logoColor=white)
  ![Anchor](https://img.shields.io/badge/Anchor-3D3D3D?style=for-the-badge&logo=anchor&logoColor=white)
</div>

## 📝 About This Project

This repository documents my 60-day journey learning Solana blockchain development. Each day focuses on specific concepts, gradually building up my skills with practical programs using the Anchor framework.

## 🗂️ Project Structure

Each day is contained in its own folder with complete Anchor project setup:

```
dayN/
  ├── Anchor.toml       # Anchor configuration
  ├── programs/         # Solana programs (smart contracts) in Rust
  ├── tests/            # TypeScript tests for the programs
  ├── app/              # Frontend applications (when applicable)
  └── migrations/       # Deployment scripts
```

## 📚 Progress Log

### Day 1: Hello World
- Created a basic Anchor program
- Implemented a simple "Hello World" message instruction
- Learned about program structure and deployment

### Day 2: Program Inputs & Error Handling
- Working with function parameters (u64, String)
- Processing array inputs
- Implementing error handling with checked operations
- Basic calculator functionality

## 🛠️ Technologies Used

- **Solana**: Blockchain platform
- **Anchor**: Framework for Solana program development
- **Rust**: Programming language for smart contracts
- **TypeScript**: For testing and frontend integration

## 🚀 Getting Started

1. **Prerequisites**:
   ```bash
   # Install Rust and Cargo
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Install Solana CLI
   sh -c "$(curl -sSfL https://release.solana.com/v1.17.0/install)"
   
   # Install Anchor
   cargo install --git https://github.com/coral-xyz/anchor avm --locked
   avm install latest
   avm use latest
   ```

2. **Running a project**:
   ```bash
   # Navigate to the day's directory
   cd dayN
   
   # Build the program
   anchor build
   
   # Run tests
   anchor test
   
   # Deploy to localnet
   anchor deploy
   ```

## 📖 Key Learnings

- Solana's programming model with accounts and instructions
- Memory safety and error handling in Rust
- Testing Solana programs with TypeScript
- Anchor framework structure and best practices

## 🔮 Future Goals

As this challenge progresses, I aim to:
- Build more complex applications
- Integrate with frontend frameworks
- Explore token creation and management
- Implement cross-program invocation
- Optimize for performance and security

## 📜 License

This project is open source and available under the [MIT License](LICENSE).

---

<div align="center">
  <p>Created with ❤️ by a Solana enthusiast</p>
  <p>Last updated: May 6, 2025</p>
</div>