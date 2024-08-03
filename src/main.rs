use std::fs::File;
use std::io::{BufRead, BufReader};
use Gsax::benchmark::run_all_benchmarks;
use Gsax::sequence_alignment::{align_sequences};

/// Global Sequence Alignment X
/// gsax seq1, seq2, seqType,
/// hasAffineGapPenalty [then matchScore, mismatchScore, gapOpeningPenalty, gapExtensionPenalty],
/// gapPenalty
///
///
fn main() {
    println!("Hello, world!");
    if std::env::args().skip(1).len() > 0 { //we are expecting cargo run -- -b or --bench or --benchmark
        println!("added more args to run");
        let prospective_bench_command = std::env::args().nth(1).unwrap();
        println!("run command: {}", prospective_bench_command);
        let allowed = vec!["b", "bench", "benchmark"];
        if !allowed.contains(&prospective_bench_command.as_str()) { return; }

        println!("This is probably a request to benchmark: bench command: {}", &prospective_bench_command);
        match run_all_benchmarks(){
            Ok(_) => println!("Run all benchmarks successfully!"),
            _ => todo!()
        }
        return;
    }

    // let seq1 = "AGCTTAGGCTAGCTTGGCATCGATC";
    // let seq2 = "AGCTTGGCATCGATTGGCAT";

    let seq1 = BufReader::new(File::open("./src/testdata/MT-human.fa", ).unwrap()).lines().skip(1)
        .map(|l| l.unwrap())
        .collect::<Vec<String>>().join("\n");

    let seq2 = BufReader::new(File::open("./src/testdata/MT-orang.fa", ).unwrap()).lines().skip(1)
        .map(|l| l.unwrap())
        .collect::<Vec<String>>().join("\n");



    match align_sequences(&seq1, &seq2){
        Ok((aligned_seq1, aligned_seq2)) => {
            println!("Aligned Sequence1: {}", aligned_seq1);
            println!("Aligned Sequence2: {}", aligned_seq2);
        },
        Err(err) => panic!("Error while aligning sequences >>> {}", err)
    }
}








#[cfg(test)]
mod tests{
    use std::time::Instant;
    use Gsax::linkedlist::LinkedList;
    use Gsax::sequence_alignment::{align_sequences, align_sequences_v2};

    #[test]
    fn test_linked_list_works(){
        let mut linkedlist = LinkedList::new();
        assert_eq!(linkedlist.pop(), None);
        linkedlist.push('A');
        linkedlist.push('G');
        linkedlist.push('C');
        linkedlist.push('T');

        println!("LinkedList -> {:?}", linkedlist);

        assert_eq!(linkedlist.pop(), Some('T'));
        assert_eq!(linkedlist.pop(), Some('C'));
        assert_eq!(linkedlist.pop(), Some('G'));
        assert_eq!(linkedlist.pop(), Some('A'));
        assert_eq!(linkedlist.pop(), None);

    }



}