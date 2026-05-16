# Simple Whitelist: Trivial Task Pipeline

An entry-level access-control matrix designed to manage allowed participants for launchpads or guarded dApps. This project implements a foundational whitelist smart contract with administrative controls, authentication checks, and event-driven architecture.

## 📋 Project Overview

**Simple Whitelist** is a modular smart contract framework that enables secure, authenticated access management for decentralized applications. The contract provides core functionality to add/remove addresses from a whitelist and query membership status, all protected by administrative authentication.

### Key Features

- **Administrative Access Control**: Owner-based authentication ensuring only authorized users can modify the whitelist
- **Efficient Data Storage**: Optimized on-chain storage using typed enums for storage keys
- **Event Tracking**: Native contract events for all state changes, enabling external indexing and monitoring
- **Query Interface**: Public read function to check whitelist membership
- **Gas Optimized**: Minimal state changes and efficient lookup mechanisms

---

## 🎯 Project Roadmap

The implementation is divided into three interconnected issues, structured by complexity and dependency:

### Issue #1.1: Initialize State & Modifiers
**Complexity**: Trivial (100 Points)

**Objective**: Establish the contract's foundational structure and initialization logic.

**Scope**:
- Define the primary contract structure
- Implement storage keys using a typed enum pattern
- Create an `initialize()` administrative setup function
- Persist the contract owner (Address) to storage during initialization
- Establish the pattern for future state management

**Acceptance Criteria**:
- [ ] Contract compiles without errors
- [ ] Storage enum properly defines all storage keys
- [ ] `initialize()` function correctly saves owner address
- [ ] Owner can be retrieved from contract storage
- [ ] Function properly guards against re-initialization

---

### Issue #1.2: Whitelist Management Logic
**Complexity**: Medium (150 Points)

**Objective**: Implement core whitelist manipulation functions with security protections.

**Scope**:
- Write `add_address(env: Env, user: Address)` function to add addresses to whitelist
- Write `remove_address(env: Env, user: Address)` function to remove addresses from whitelist
- Implement authentication checks requiring the contract owner's signature
- Use `admin.require_auth()` pattern to enforce admin-only operations
- Manage underlying storage Map for efficient lookups
- Handle edge cases (duplicate additions, non-existent removals)

**Acceptance Criteria**:
- [ ] `add_address()` successfully adds valid addresses to whitelist
- [ ] `remove_address()` successfully removes addresses from whitelist
- [ ] Functions reject calls without valid owner authentication
- [ ] Unauthorized users cannot modify whitelist
- [ ] Duplicate additions are handled gracefully
- [ ] Storage state is correctly updated in both operations

---

### Issue #1.3: Read Queries & Event Dispatches
**Complexity**: Trivial (100 Points)

**Objective**: Complete the contract's public interface with query functionality and event emissions.

**Scope**:
- Create `is_whitelisted(env: Env, user: Address) -> bool` query function
- Implement efficient search against the underlying Map storage
- Emit native Soroban contract events for all state modifications
- Dispatch events on successful additions to whitelist
- Dispatch events on successful removals from whitelist
- Enable external indexing engines and monitoring services

**Acceptance Criteria**:
- [ ] `is_whitelisted()` returns true for addresses in whitelist
- [ ] `is_whitelisted()` returns false for addresses not in whitelist
- [ ] Events are emitted on successful additions
- [ ] Events are emitted on successful removals
- [ ] Event data includes relevant context (user address, action type)
- [ ] External services can index contract events

---

## 🏗️ Architecture

### Contract Structure

```
SimpleWhitelist
├── Storage
│   ├── Owner (Address)
│   └── Whitelist (Map<Address, bool>)
├── Admin Functions
│   ├── initialize() → void
│   ├── add_address() → void
│   └── remove_address() → void
└── Public Functions
    └── is_whitelisted() → bool
```

### Storage Keys

Storage keys are managed through a typed enum to ensure consistency and prevent collisions:

- `Owner`: Stores the contract administrator's address
- `Whitelist`: Map data structure maintaining whitelist membership

### Event Types

The contract emits events for audit trails and external indexing:

- `Whitelisted`: Emitted when an address is added
- `Removed`: Emitted when an address is removed

---

## 🔐 Security Considerations

### Authentication Pattern

All state-modifying operations use the `require_auth()` pattern:

```
admin.require_auth() → Only the contract owner can execute
```

### Access Control

- **Initialization**: Can only be called once (idempotent)
- **Modifications**: Restricted to contract owner
- **Queries**: Public, read-only operations with no restrictions

### Storage Safety

- Typed storage keys prevent accidental overwrites
- Map structure ensures efficient lookups without iteration
- Events provide immutable audit trails

---

## 🚀 Getting Started

### Prerequisites

- Rust toolchain with `wasm32-unknown-unknown` target
- Soroban SDK dependencies
- Basic understanding of smart contract patterns

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/Kings9595/Simple-Whitelist.git
   cd Simple-Whitelist
   ```

2. Build the project:
   ```bash
   cargo build --target wasm32-unknown-unknown --release
   ```

3. Deploy to Soroban network:
   ```bash
   soroban contract deploy --wasm path/to/contract.wasm
   ```

### Usage Example

```rust
// Initialize the contract
let contract = SimpleWhitelist::new(env);
contract.initialize(&owner_address);

// Add an address to whitelist
contract.add_address(&user_address);

// Check if address is whitelisted
let is_allowed = contract.is_whitelisted(&user_address);

// Remove address from whitelist
contract.remove_address(&user_address);
```

---

## 📊 Development Phases

### Phase 1: Foundation (Issue #1.1)
- Set up contract structure
- Define storage keys
- Implement initialization

### Phase 2: Logic (Issue #1.2)
- Implement add/remove functions
- Integrate authentication
- Handle state management

### Phase 3: Interface (Issue #1.3)
- Create query functions
- Emit events
- Enable external integration

---

## 🧪 Testing Strategy

### Unit Tests

- Storage initialization tests
- Add/remove operation tests
- Authentication failure scenarios
- Event emission verification

### Integration Tests

- Multi-user scenarios
- Edge cases (duplicates, re-initialization)
- Cross-contract calls
- Event indexing

### Security Audits

- Access control verification
- State mutation safety
- Event integrity
- Gas optimization

---

## 📈 Use Cases

### Launchpad Access Control
Manage which addresses can participate in token sales or launches:
```
Before: Anyone can participate
After: Only whitelisted addresses can claim tokens
```

### dApp Access Gates
Restrict access to premium features or beta programs:
```
gated_feature() {
    require(is_whitelisted(caller))
    ...
}
```

### DAO Membership
Control voting rights based on whitelist membership:
```
vote(proposal_id) {
    require(is_whitelisted(voter))
    ...
}
```

---

## 📝 API Reference

### Functions

#### `initialize(owner: Address) → void`
Sets up the contract with the initial owner address.
- **Parameters**: `owner` - The address of the contract administrator
- **Access**: Once per contract deployment
- **Events**: None

#### `add_address(user: Address) → void`
Adds an address to the whitelist.
- **Parameters**: `user` - The address to whitelist
- **Access**: Owner only (requires authentication)
- **Events**: Emits `Whitelisted` event
- **Errors**: Fails if caller is not owner

#### `remove_address(user: Address) → void`
Removes an address from the whitelist.
- **Parameters**: `user` - The address to remove
- **Access**: Owner only (requires authentication)
- **Events**: Emits `Removed` event
- **Errors**: Fails if caller is not owner

#### `is_whitelisted(user: Address) → bool`
Queries whether an address is whitelisted.
- **Parameters**: `user` - The address to check
- **Returns**: `true` if whitelisted, `false` otherwise
- **Access**: Public
- **Events**: None

---

## 🤝 Contributing

Contributions are welcome! Follow these steps:

1. Fork the repository
2. Create a feature branch for each issue
3. Write tests for new functionality
4. Submit a pull request with a clear description
5. Ensure all tests pass and code follows conventions

---

## 📄 License

This project is provided as an educational resource for learning smart contract development on the Soroban platform.

---

## 🔗 Resources

- [Soroban Documentation](https://soroban.stellar.org/)
- [Stellar Network](https://stellar.org/)
- [Rust Smart Contracts](https://docs.rs/soroban-sdk/)
- [Access Control Patterns](https://docs.soliditylang.org/en/latest/access-control.html)

---

## 📞 Support

For questions or issues:
- Open a GitHub issue in this repository
- Check existing documentation
- Review example implementations

---

## ✨ Acknowledgments

Built as a learning project to master entry-level smart contract development patterns on the Soroban platform.

---

**Last Updated**: 2026-05-16
