# 📧 MultiversX Email Box dApp

A decentralized email application built on the MultiversX blockchain as part of the NEARX DOJO challenge. This project enables users to send, receive, and store messages on-chain with content references stored on IPFS.

![MultiversX](https://img.shields.io/badge/Platform-MultiversX-blue)
![License](https://img.shields.io/badge/License-MIT-green)
![Status](https://img.shields.io/badge/Status-In%20Development-yellow)

## 🌟 Project Overview

This decentralized email application allows users to send and receive messages on the MultiversX blockchain. The project implements a familiar email interface while leveraging blockchain technology for security, ownership, and decentralization.

## 🏆 NearX DOJO Challenge

This project is being developed as part of the [NearX DOJO MultiversX Challenge](https://github.com/nrxschool/dojo-multiversx/) (Week #4: Email Box with Smart Contract). The challenge requirements include:

### Must Have:
- Frontend with Web3 authentication (MultiversX Wallet) for sending/receiving emails
- Rust smart contract to store email messages
- Integration using @multiversx/sdk-js

### Should Have:
- Permission verification to prevent unauthorized access
- Message size limits to avoid excessive gas costs
- Intuitive and reactive frontend design
- Transaction and sending state feedback
- Contract events for automatic frontend updates

### Could Have:
- Content encryption for enhanced privacy
- Message search functionality
- Web3 notifications for new messages
- Off-chain storage to reduce costs (storing only hashes on-chain)

## ✅ Completed Features

- ✉️ Rust smart contract to send/receive emails on-chain
- 📋 Metadata storage: sender, recipient, subject, preview, timestamp, IPFS hash
- 📂 Separate inbox/sent folders per user
- 📄 Pagination support for scalable message viewing
- 🔔 Emitted events for sent emails
- ⚙️ Configurable size limits for previews & content
- 🔐 Access control: users can only query their own inboxes

## 💡 Design Decisions

- **Storage Optimization**: Only message previews stored on-chain; full content lives on IPFS
- **Scalability**: Pagination implemented for retrieving inbox/sent messages
- **Efficiency**: Avoiding storage/gas bloat by managing payload sizes
- **Security**: Access control mechanisms to protect user privacy

## 🛣️ Roadmap

- 🖥️ Frontend with MultiversX wallet authentication
- 📝 Email-like UI and sending form
- ⚡ Live updates via blockchain events
- 🔒 Exploring encryption and storage strategies

## 🤝 Contributions & Feedback

I'm new to Rust and smart contracts, learning by doing, failing, and iterating. Any feedback or contributions are welcome!

- 💬 Suggestions for improvements
- 🐞 Bug reports
- 🧠 Design pattern recommendations
- 📚 Learning resources

Feel free to open issues or submit pull requests on the project repository.

## 📜 License

This project is licensed under the MIT License.
