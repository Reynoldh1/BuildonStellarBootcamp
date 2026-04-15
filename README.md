Stellar Scheduler DApp

Decentralized Task Scheduling on Stellar Soroban

Submission Information

This project was built as a workshop submission and satisfies all requirements:

Built using Soroban Smart Contract (Rust)
Application is different from workshop example
Includes clear README in English
Includes Testnet Contract ID
Includes Testnet interaction screenshot
Repository name matches the project context
Project Overview

Stellar Scheduler DApp is a decentralized task scheduling system deployed on the Stellar Soroban blockchain.

The smart contract allows users to create tasks with deadlines and automatically determine whether a task is completed on time or expired.

Unlike traditional productivity apps that rely on centralized databases, this scheduler stores tasks on-chain, ensuring transparency, persistence, and trustless verification.

This project demonstrates how Soroban smart contracts can be used for time-based automation and productivity tools.

Project Vision

The goal of this project is to explore how blockchain can be used for trustless task management and time validation.

We aim to:

Eliminate reliance on centralized productivity platforms
Provide tamper-proof task tracking
Enable deadline verification using blockchain timestamps
Demonstrate real-world Soroban use cases
Smart Contract Features
Add Schedule (Create Task)

Create a task with a specific deadline.

Stored data:

Task ID
Description
Deadline (Unix Timestamp)
Status (PENDING by default)

Validation:

Deadline must be in the future

Function:
add_schedule(id, description, deadline)

Complete Task

Mark a task as completed and automatically verify deadline status.

Smart contract logic:

If completed before deadline → DONE
If completed after deadline → EXPIRED

Function:
complete_task(id)

Get Schedule

Retrieve task details by ID.

Returns:

ID
Description
Deadline
Status

Function:
get_schedule(id)

Why Blockchain for Scheduling?

This DApp guarantees:

Immutable task records
Trustless deadline verification using ledger timestamp
Transparent task history
No centralized database
Stellar Network Benefits
Fast execution
Extremely low fees
Secure smart contract environment
Global decentralized infrastructure
Smart Contract Details

Network: Stellar Soroban Testnet

Contract ID:
PASTE_YOUR_DEPLOYED_CONTRACT_ID_HERE

Testnet Screenshot

Add screenshot file to repository:

/screenshot.png

Caption example:
Screenshot showing successful interaction with SchedulerContract on Stellar Soroban Testnet.

Tech Stack
Rust
Soroban SDK
Stellar Blockchain
Smart Contracts
How To Interact

After deployment, call the contract functions:

add_schedule() → Create new task
complete_task() → Complete and verify task
get_schedule() → Retrieve task details
Future Improvements

Planned upgrades:

Task ownership using wallet address
Multiple tasks per user
Task reminders via off-chain service
Recurring schedules
Frontend dashboard
Integration with Stellar wallets
Suggested Repository Name

stellar-scheduler-dapp

Submission Checklist
Unique Soroban DApp
English README
Smart contract deployed
Contract ID included
Screenshot included
Ready for submission