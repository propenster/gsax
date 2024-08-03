use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use crate::sequence_alignment::{align_sequences, align_sequences_v2};




fn init() -> (String, String){
    let seq1 = BufReader::new(File::open("./src/testdata/MT-human.fa", ).unwrap()).lines().skip(1)
        .map(|l| l.unwrap())
        .collect::<Vec<String>>().join("\n");

    let seq2 = BufReader::new(File::open("./src/testdata/MT-orang.fa", ).unwrap()).lines().skip(1)
        .map(|l| l.unwrap())
        .collect::<Vec<String>>().join("\n");

    (seq1, seq2)
}
pub fn run_all_benchmarks() -> Result<(), Box<dyn std::error::Error>>{
    let (seq1, seq2) = init();

    println!("Running benchmarks");

    test_benchmark_align_sequences_10000(&seq1, &seq2, 1, "test_benchmark_align_sequences_10000", 1);
    test_benchmark_align_sequences_v2_10000(&seq1, &seq2, 2,"test_benchmark_align_sequences_v2_10000", 1);

    Ok(())
}

fn test_benchmark_align_sequences_10000(seq1: &str, seq2: &str, job_id: u8, name: &str, iterations: u32){
    println!("Running benchmark job id: {} {}", job_id, name);
    // let seq1 = "AGCTTAGGCTAGCTTGGCATCGATC";
    // let seq2 = "AGCTTGGCATCGATTGGCAT";
    let now = Instant::now();
    {
        for _ in 0..iterations{
            match align_sequences(seq1, seq2){
                Ok((_,_)) => {
                    // println!("Aligned Sequence1: {}", aligned_seq1);
                    // println!("Aligned Sequence2: {}", aligned_seq2);
                },
                Err(err) => panic!("Error while aligning sequences >>> {}", err)
            };
        }

    }
    let elapsed = now.elapsed();
    println!("Job id: {} Elapsed: {:.2?}", job_id, elapsed);
    assert!(true);
}
fn test_benchmark_align_sequences_v2_10000(seq1: &str, seq2: &str, job_id: u8, name: &str, iterations: u32){
    println!("Running benchmark job id: {} {}", job_id, name);
    // let seq1 = "AGCTTAGGCTAGCTTGGCATCGATC";
    // let seq2 = "AGCTTGGCATCGATTGGCAT";
    let now = Instant::now();
    {
        for _ in 0..iterations{
            match align_sequences_v2(seq1, seq2){
                Ok((_,_)) => {
                    // println!("Aligned Sequence1: {}", aligned_seq1);
                    // println!("Aligned Sequence2: {}", aligned_seq2);
                },
                Err(err) => panic!("Error while aligning sequences >>> {}", err)
            };
        }
    }
    let elapsed = now.elapsed();
    println!("Job id: {} Elapsed: {:.2?}", job_id, elapsed);
    assert!(true);
}


fn get_func_name<F>(_: F) -> &'static str
    where F : Fn()
{
    std::any::type_name::<F>()
}
