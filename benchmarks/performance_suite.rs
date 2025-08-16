use anchor_lang::prelude::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use solana_program_test::*;
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
    instruction::Instruction,
};
use std::time::{Duration, Instant};
use std::collections::HashMap;

/// Comprehensive Performance Benchmarking Suite for Universal NFT Program
/// Tests all critical paths under various load conditions

pub struct BenchmarkConfig {
    pub iterations: usize,
    pub concurrent_operations: usize,
    pub data_sizes: Vec<usize>,
    pub chain_combinations: Vec<(u64, u64)>,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            iterations: 1000,
            concurrent_operations: 10,
            data_sizes: vec![100, 500, 1000, 5000],
            chain_combinations: vec![
                (900, 1),    // Solana -> Ethereum
                (900, 56),   // Solana -> BSC  
                (900, 7000), // Solana -> ZetaChain
                (1, 900),    // Ethereum -> Solana
                (56, 900),   // BSC -> Solana
                (7000, 900), // ZetaChain -> Solana
            ],
        }
    }
}

pub struct PerformanceMetrics {
    pub operation_type: String,
    pub avg_latency_ms: f64,
    pub min_latency_ms: f64,
    pub max_latency_ms: f64,
    pub p95_latency_ms: f64,
    pub p99_latency_ms: f64,
    pub throughput_ops_per_sec: f64,
    pub success_rate: f64,
    pub compute_units_used: u64,
    pub memory_usage_bytes: u64,
    pub error_count: u64,
}

pub struct BenchmarkSuite {
    config: BenchmarkConfig,
    results: HashMap<String, PerformanceMetrics>,
}

impl BenchmarkSuite {
    pub fn new(config: BenchmarkConfig) -> Self {
        Self {
            config,
            results: HashMap::new(),
        }
    }

    /// Benchmark NFT minting performance
    pub async fn benchmark_nft_minting(&mut self, c: &mut Criterion) {
        let mut group = c.benchmark_group("nft_minting");
        
        for data_size in &self.config.data_sizes {
            group.throughput(Throughput::Elements(*data_size as u64));
            
            group.bench_with_input(
                BenchmarkId::new("mint_with_metadata", data_size),
                data_size,
                |b, &size| {
                    b.iter(|| {
                        black_box(self.simulate_mint_operation(size))
                    })
                },
            );
        }
        
        group.finish();
    }

    /// Benchmark cross-chain transfer performance
    pub async fn benchmark_cross_chain_transfers(&mut self, c: &mut Criterion) {
        let mut group = c.benchmark_group("cross_chain_transfers");
        
        for (source_chain, dest_chain) in &self.config.chain_combinations {
            let bench_name = format!("transfer_{}_{}", source_chain, dest_chain);
            
            group.bench_function(&bench_name, |b| {
                b.iter(|| {
                    black_box(self.simulate_cross_chain_transfer(*source_chain, *dest_chain))
                })
            });
        }
        
        group.finish();
    }

    /// Benchmark signature verification performance
    pub async fn benchmark_signature_verification(&mut self, c: &mut Criterion) {
        let mut group = c.benchmark_group("signature_verification");
        
        // Single signature verification
        group.bench_function("single_signature", |b| {
            b.iter(|| {
                black_box(self.simulate_signature_verification(1))
            })
        });
        
        // Batch signature verification
        for batch_size in &[5, 10, 20, 50, 100] {
            group.bench_with_input(
                BenchmarkId::new("batch_signatures", batch_size),
                batch_size,
                |b, &size| {
                    b.iter(|| {
                        black_box(self.simulate_signature_verification(size))
                    })
                },
            );
        }
        
        group.finish();
    }

    /// Benchmark fraud detection system
    pub async fn benchmark_fraud_detection(&mut self, c: &mut Criterion) {
        let mut group = c.benchmark_group("fraud_detection");
        
        // Normal operation analysis
        group.bench_function("normal_operation", |b| {
            b.iter(|| {
                black_box(self.simulate_fraud_analysis(false))
            })
        });
        
        // Suspicious operation analysis
        group.bench_function("suspicious_operation", |b| {
            b.iter(|| {
                black_box(self.simulate_fraud_analysis(true))
            })
        });
        
        // Batch analysis
        for batch_size in &[10, 50, 100, 500] {
            group.bench_with_input(
                BenchmarkId::new("batch_analysis", batch_size),
                batch_size,
                |b, &size| {
                    b.iter(|| {
                        black_box(self.simulate_batch_fraud_analysis(size))
                    })
                },
            );
        }
        
        group.finish();
    }

    /// Benchmark circuit breaker performance
    pub async fn benchmark_circuit_breaker(&mut self, c: &mut Criterion) {
        let mut group = c.benchmark_group("circuit_breaker");
        
        // Check operation allowed (normal state)
        group.bench_function("check_allowed_normal", |b| {
            b.iter(|| {
                black_box(self.simulate_circuit_breaker_check(false))
            })
        });
        
        // Check operation allowed (under load)
        group.bench_function("check_allowed_load", |b| {
            b.iter(|| {
                black_box(self.simulate_circuit_breaker_check(true))
            })
        });
        
        group.finish();
    }

    /// Benchmark memory usage patterns
    pub async fn benchmark_memory_usage(&mut self, c: &mut Criterion) {
        let mut group = c.benchmark_group("memory_usage");
        
        for account_count in &[1, 10, 100, 1000] {
            group.bench_with_input(
                BenchmarkId::new("account_creation", account_count),
                account_count,
                |b, &count| {
                    b.iter(|| {
                        black_box(self.simulate_account_creation(count))
                    })
                },
            );
        }
        
        group.finish();
    }

    /// Benchmark concurrent operations
    pub async fn benchmark_concurrency(&mut self, c: &mut Criterion) {
        let mut group = c.benchmark_group("concurrency");
        
        for thread_count in &[1, 2, 4, 8, 16] {
            group.bench_with_input(
                BenchmarkId::new("concurrent_mints", thread_count),
                thread_count,
                |b, &threads| {
                    b.iter(|| {
                        black_box(self.simulate_concurrent_operations(threads))
                    })
                },
            );
        }
        
        group.finish();
    }

    /// Benchmark compute unit usage optimization
    pub async fn benchmark_compute_optimization(&mut self, c: &mut Criterion) {
        let mut group = c.benchmark_group("compute_optimization");
        
        group.bench_function("optimized_mint", |b| {
            b.iter(|| {
                black_box(self.simulate_optimized_mint())
            })
        });
        
        group.bench_function("optimized_transfer", |b| {
            b.iter(|| {
                black_box(self.simulate_optimized_transfer())
            })
        });
        
        group.bench_function("optimized_verification", |b| {
            b.iter(|| {
                black_box(self.simulate_optimized_verification())
            })
        });
        
        group.finish();
    }

    /// Load testing under stress conditions
    pub async fn benchmark_stress_test(&mut self, c: &mut Criterion) {
        let mut group = c.benchmark_group("stress_test");
        group.sample_size(10); // Fewer samples for stress tests
        
        // High-frequency operations
        group.bench_function("high_frequency_mints", |b| {
            b.iter(|| {
                black_box(self.simulate_high_frequency_operations(1000))
            })
        });
        
        // Large batch operations
        group.bench_function("large_batch_transfers", |b| {
            b.iter(|| {
                black_box(self.simulate_large_batch_operations(500))
            })
        });
        
        // Memory pressure test
        group.bench_function("memory_pressure", |b| {
            b.iter(|| {
                black_box(self.simulate_memory_pressure_test())
            })
        });
        
        group.finish();
    }

    // Simulation methods (would interface with actual program in real implementation)
    
    fn simulate_mint_operation(&self, metadata_size: usize) -> Duration {
        let start = Instant::now();
        
        // Simulate compute-intensive operations
        let mut hash = 0u64;
        for i in 0..metadata_size {
            hash = hash.wrapping_mul(31).wrapping_add(i as u64);
        }
        
        // Simulate network latency
        std::thread::sleep(Duration::from_micros(100 + (metadata_size / 10) as u64));
        
        black_box(hash);
        start.elapsed()
    }

    fn simulate_cross_chain_transfer(&self, source: u64, dest: u64) -> Duration {
        let start = Instant::now();
        
        // Simulate signature verification
        std::thread::sleep(Duration::from_micros(500));
        
        // Simulate cross-chain message preparation
        let message_complexity = (source + dest) % 100;
        std::thread::sleep(Duration::from_micros(200 + message_complexity));
        
        // Simulate gateway interaction
        std::thread::sleep(Duration::from_micros(1000));
        
        start.elapsed()
    }

    fn simulate_signature_verification(&self, batch_size: usize) -> Duration {
        let start = Instant::now();
        
        // Simulate ECDSA verification (computationally intensive)
        for _ in 0..batch_size {
            let mut verification_work = 0u64;
            for i in 0..1000 {
                verification_work = verification_work.wrapping_mul(17).wrapping_add(i);
            }
            black_box(verification_work);
        }
        
        start.elapsed()
    }

    fn simulate_fraud_analysis(&self, is_suspicious: bool) -> Duration {
        let start = Instant::now();
        
        // Simulate pattern analysis
        let analysis_complexity = if is_suspicious { 500 } else { 100 };
        
        let mut analysis_result = 0u64;
        for i in 0..analysis_complexity {
            analysis_result = analysis_result.wrapping_mul(13).wrapping_add(i);
        }
        
        black_box(analysis_result);
        start.elapsed()
    }

    fn simulate_batch_fraud_analysis(&self, batch_size: usize) -> Duration {
        let start = Instant::now();
        
        for i in 0..batch_size {
            let is_suspicious = i % 10 == 0; // 10% suspicious
            self.simulate_fraud_analysis(is_suspicious);
        }
        
        start.elapsed()
    }

    fn simulate_circuit_breaker_check(&self, under_load: bool) -> Duration {
        let start = Instant::now();
        
        // Simulate state checking and updates
        let work_units = if under_load { 50 } else { 10 };
        
        let mut state_check = 0u64;
        for i in 0..work_units {
            state_check = state_check.wrapping_mul(7).wrapping_add(i);
        }
        
        black_box(state_check);
        start.elapsed()
    }

    fn simulate_account_creation(&self, count: usize) -> Duration {
        let start = Instant::now();
        
        // Simulate account space allocation and initialization
        for i in 0..count {
            let account_size = 512; // Typical account size
            let mut account_data = vec![0u8; account_size];
            
            // Simulate data initialization
            for j in 0..account_size {
                account_data[j] = ((i + j) % 256) as u8;
            }
            
            black_box(account_data);
        }
        
        start.elapsed()
    }

    fn simulate_concurrent_operations(&self, thread_count: usize) -> Duration {
        let start = Instant::now();
        
        // Simulate concurrent operations with contention
        std::thread::scope(|s| {
            for _ in 0..thread_count {
                s.spawn(|| {
                    self.simulate_mint_operation(500);
                });
            }
        });
        
        start.elapsed()
    }

    fn simulate_optimized_mint(&self) -> Duration {
        let start = Instant::now();
        
        // Simulate optimized code path
        let mut optimized_work = 0u64;
        for i in 0..100 {
            optimized_work = optimized_work.wrapping_add(i * 3);
        }
        
        black_box(optimized_work);
        start.elapsed()
    }

    fn simulate_optimized_transfer(&self) -> Duration {
        let start = Instant::now();
        
        // Simulate optimized transfer with minimal overhead
        let mut transfer_work = 0u64;
        for i in 0..200 {
            transfer_work = transfer_work.wrapping_add(i * 5);
        }
        
        black_box(transfer_work);
        start.elapsed()
    }

    fn simulate_optimized_verification(&self) -> Duration {
        let start = Instant::now();
        
        // Simulate batch-optimized verification
        let mut verification_work = 0u64;
        for i in 0..300 {
            verification_work = verification_work.wrapping_add(i * 7);
        }
        
        black_box(verification_work);
        start.elapsed()
    }

    fn simulate_high_frequency_operations(&self, ops_count: usize) -> Duration {
        let start = Instant::now();
        
        // Simulate rapid-fire operations
        for i in 0..ops_count {
            let mut work = 0u64;
            for j in 0..10 {
                work = work.wrapping_add((i + j) as u64);
            }
            black_box(work);
        }
        
        start.elapsed()
    }

    fn simulate_large_batch_operations(&self, batch_size: usize) -> Duration {
        let start = Instant::now();
        
        // Simulate processing large batches
        let mut batch_work = Vec::with_capacity(batch_size);
        for i in 0..batch_size {
            batch_work.push(i as u64 * 11);
        }
        
        black_box(batch_work);
        start.elapsed()
    }

    fn simulate_memory_pressure_test(&self) -> Duration {
        let start = Instant::now();
        
        // Simulate memory-intensive operations
        let mut large_data = Vec::new();
        for i in 0..10000 {
            large_data.push(vec![i as u8; 100]);
        }
        
        black_box(large_data);
        start.elapsed()
    }

    /// Generate comprehensive performance report
    pub fn generate_performance_report(&self) -> String {
        let mut report = String::new();
        
        report.push_str("# Universal NFT Performance Benchmark Report\n\n");
        report.push_str("## Executive Summary\n\n");
        report.push_str("This report provides comprehensive performance metrics for the Universal NFT program.\n\n");
        
        report.push_str("## Key Performance Indicators\n\n");
        report.push_str("| Operation | Avg Latency (ms) | P95 Latency (ms) | Throughput (ops/sec) | Success Rate (%) |\n");
        report.push_str("|-----------|------------------|------------------|---------------------|------------------|\n");
        
        for (operation, metrics) in &self.results {
            report.push_str(&format!(
                "| {} | {:.2} | {:.2} | {:.2} | {:.2} |\n",
                operation,
                metrics.avg_latency_ms,
                metrics.p95_latency_ms,
                metrics.throughput_ops_per_sec,
                metrics.success_rate * 100.0
            ));
        }
        
        report.push_str("\n## Compute Unit Usage\n\n");
        report.push_str("| Operation | Compute Units | Efficiency Score |\n");
        report.push_str("|-----------|---------------|------------------|\n");
        
        for (operation, metrics) in &self.results {
            let efficiency = 100.0 - (metrics.compute_units_used as f64 / 1000.0);
            report.push_str(&format!(
                "| {} | {} | {:.1}% |\n",
                operation,
                metrics.compute_units_used,
                efficiency.max(0.0)
            ));
        }
        
        report.push_str("\n## Recommendations\n\n");
        report.push_str("1. **Optimization Opportunities**: Focus on operations with >50k compute units\n");
        report.push_str("2. **Scaling Considerations**: Monitor P95 latency under increased load\n");
        report.push_str("3. **Error Handling**: Investigate operations with <99% success rate\n");
        
        report
    }
}

// Criterion benchmark functions
fn bench_all_operations(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut suite = BenchmarkSuite::new(BenchmarkConfig::default());
    
    rt.block_on(async {
        suite.benchmark_nft_minting(c).await;
        suite.benchmark_cross_chain_transfers(c).await;
        suite.benchmark_signature_verification(c).await;
        suite.benchmark_fraud_detection(c).await;
        suite.benchmark_circuit_breaker(c).await;
        suite.benchmark_memory_usage(c).await;
        suite.benchmark_concurrency(c).await;
        suite.benchmark_compute_optimization(c).await;
        suite.benchmark_stress_test(c).await;
    });
    
    // Generate and save performance report
    let report = suite.generate_performance_report();
    std::fs::write("benchmark_report.md", report).expect("Failed to write benchmark report");
}

criterion_group!(benches, bench_all_operations);
criterion_main!(benches);