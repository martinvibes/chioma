# Benchmarking Guide

**Priority:** MEDIUM
**Category:** Documentation
**Type:** Documentation
**Status:** Completed
**Related Issues:** #753, #17, #12, #16

## 1. Introduction

This guide provides procedures for measuring and analyzing the performance of Soroban-based smart contracts within the Chioma ecosystem. Benchmarking is essential to minimize gas costs and ensure system reliability during period of high network traffic.

## 2. Benchmarking Setup

### 2.1 Dependencies
Ensure you have the following tools installed:
- `rustup` with the `wasm32-unknown-unknown` target.
- `soroban-cli` (latest version).
- `cargo-bench` (optional, for low-level performance testing).

### 2.2 Compilation Profile
Always perform benchmarking on optimized release builds to get accurate measurements.

```bash
cargo build --release --target wasm32-unknown-unknown
```

## 3. Performance Measurement

### 3.1 Gas Cost Measurement
Soroban uses "Gas" to measure the cost of contract execution. To measure gas costs for a specific function call:

1. **Invoke the function in a local environment:**
```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --network testnet \
  -- \
  <FUNCTION_NAME> \
  --arg1 <VAL> \
  <EXTRA_ARGS>
```

2. **Check the output for gas usage details:**
The CLI provides cost breakdowns including CPU instructions and RAM usage.

### 3.2 Automated Testing for Costs
You can use the `soroban-sdk` test utilities to assert on gas costs within your unit tests:

```rust
#[test]
fn test_gas_usage() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);
    
    // Perform operation
    client.heavy_function();
    
    // The test summary will report resource consumption if run with --nocapture
}
```

## 4. Performance Analysis

### 4.1 Bottleneck Identification
Common performance bottlenecks in Soroban include:
- **Excessive Storage Access:** Reading and writing to persistent storage is the most expensive operation.
- **Complex Loops:** $O(N^2)$ operations should be avoided.
- **Large Event Payloads:** Emitting large events increases ledger footprint.

### 4.2 Resource Metrics
- **CPU Instructions:** Number of Instructions executed by the VM.
- **RAM Usage:** Memory used during execution.
- **Read/Write Bytes:** Data moved to and from the ledger.

## 5. Baseline Establishment

Before optimizing, establish a baseline for major functions:
1. Run the benchmark 10 times.
2. Calculate the average CPU and RAM usage.
3. Record these values in the `performance_baselines.json` file.

## 6. Performance Comparison

When a new optimization is implemented:
1. Re-run the benchmarks under identical conditions.
2. Compare the results against the established baseline.
3. Document the percentage improvement in the PR description.

## 7. Benchmarking Tools

- **Soroban CLI:** Main tool for interaction and basic cost reporting.
- **Stellar Horizon/RPC:** For monitoring gas usage on the live network.
- **Criterion.rs:** For local micro-benchmarking of non-contract Rust logic.
- **Valgrind/Cachegrind:** For memory usage profiling (not Soroban-specific).

## 8. Best Practices

- **Test with realistic data:** Don't benchmark with empty fields. Use payloads that reflect typical production usage.
- **Isolate environment:** Run benchmarks when your machine is not under heavy load to ensure consistency.
- **Optimize Storage:** Group multiple reads/writes into single structures where possible.
- **Use Temporary Storage:** For data that only needs to live for a short period (blocks/days).

## 9. Reporting Benchmarks

Benchmark reports should include:
- **Environment:** (e.g., Localhost, Testnet).
- **Function:** The name of the function tested.
- **CPU:** Instructions used.
- **Memory:** Bytes used.
- **Comparison:** % change from baseline.
- **Recommendation:** Any further potential optimizations.

## 10. Examples

### Benchmarking `create_agreement`
```bash
# Record baseline
soroban contract invoke --id ... -- create_agreement --landlord G... --tenant G...
# Output: [CPU: 1,200,500, RAM: 450,200]
```
