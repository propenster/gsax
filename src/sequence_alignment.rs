use crate::linkedlist::LinkedList;
use std::cmp::max as max;

///align by Needleman-Wunsch global sequence alignment algorithm
///let seq1 and seq2 be two sequences to be aligned
/// let n be length of seq1 and m be length of seq2
///
///return alignment output...
pub fn align_sequences(seq1: &str, seq2: &str) -> Result<(String, String), Box<dyn std::error::Error>> {
    let m = seq1.len();
    let n = seq2.len();
    //this is the score matrix...a 2d array to hold the match or mismatch score of alignment
    let mut score_matrix = vec![vec![0isize; n + 1]; m + 1];
    //improvement of a direction-matrix that's based on a Singly-LinkedList
    let mut direction_matrix = vec![vec![LinkedList::new(); n + 1]; m + 1];

    //construct the score matrix...
    for i in 0..=m {
        for j in 0..=n {
            // direction_matrix[i][j] = LinkedList::new();
            if i == 0 {
                score_matrix[i][j] = (j as isize) * -1; //gap penalty
                direction_matrix[i][j].push('-');
            } else if j == 0 {
                score_matrix[i][j] = (i as isize) * -1; //gap penalty
                direction_matrix[i][j].push('-');
            } else {
                let match_score = get_match(score_matrix[i - 1][j - 1], seq1.chars().nth(i - 1).unwrap(), seq2.chars().nth(j - 1).unwrap());
                let delete = score_matrix[i - 1][j] - 1;
                let insert = score_matrix[i][j - 1] - 1;
                score_matrix[i][j] = max(max(match_score, delete), insert);
                if score_matrix[i][j] == match_score {
                    direction_matrix[i][j].push('M'); //this means Match/mismatch
                } else if score_matrix[i][j] == delete {
                    direction_matrix[i][j].push('D'); //a deletion
                } else {
                    direction_matrix[i][j].push('I'); //an insertion
                }
            }
        }
    }

    // println!("matrix: {:?}", direction_matrix);
    // trackback to build the optimal aligned sequences... bottom right to top left
    let mut aligned_sequence1 = String::new();
    let mut aligned_sequence2 = String::new();
    let mut x = m;
    let mut y = n;

    while x > 0 || y > 0 {
        let current_head = direction_matrix[x][y].head.take();
        if x > 0 && y > 0 && current_head.as_ref().is_some_and(|v| v.data == 'M') {
            aligned_sequence1.push(seq1.chars().nth(x - 1).unwrap());
            aligned_sequence2.push(seq2.chars().nth(y - 1).unwrap());
            x -= 1;
            y -= 1;
        } else if x > 0 && current_head.as_ref().is_some_and(|v| v.data == 'D') {
            aligned_sequence1.push(seq1.chars().nth(x - 1).unwrap());
            aligned_sequence2.push('-');
            x -= 1;
        } else {
            aligned_sequence1.push('-');
            aligned_sequence2.push(seq2.chars().nth(y - 1).unwrap());
            y -= 1;
        }
    }

    // println!("Aligned Seq1 before rev: {}", aligned_sequence1);
    // println!("Aligned Seq2 before rev: {}", aligned_sequence2);
    aligned_sequence1 = aligned_sequence1.chars().rev().collect();
    aligned_sequence2 = aligned_sequence2.chars().rev().collect();

    Ok((aligned_sequence1, aligned_sequence2))
}

pub fn align_sequences_v2(seq1: &str, seq2: &str) -> Result<(String, String), Box<dyn std::error::Error>> {
    let m = seq1.len();
    let n = seq2.len();
    // this is the score matrix...a 2d array to hold the match or mismatch score of alignment
    let mut score_matrix = vec![vec![0isize; n + 1]; m + 1];
    // improvement of a direction-matrix that's based on a Singly-LinkedList
    let mut direction_matrix = vec![vec![LinkedList::new(); n + 1]; m + 1];

    // construct the score matrix...
    for i in 0..=m {
        for j in 0..=n {
            // direction_matrix[i][j] = LinkedList::new();
            if i == 0 {
                //println!("i = {} and j = {} ", i, j);
                score_matrix[i][j] = (j as isize) * -1; // gap penalty
                direction_matrix[i][j].push('-');
            } else if j == 0 {
                score_matrix[i][j] = (i as isize) * -1; // gap penalty
                direction_matrix[i][j].push('-');
            } else {
                let match_score = get_match(score_matrix[i - 1][j - 1], seq1.chars().nth(i - 1).unwrap(), seq2.chars().nth(j - 1).unwrap());
                let delete = score_matrix[i - 1][j] - 1;
                let insert = score_matrix[i][j - 1] - 1;
                score_matrix[i][j] = max(max(match_score, delete), insert);

                if score_matrix[i][j] == match_score {
                    direction_matrix[i][j].push('M'); // this means Match/mismatch
                } else if score_matrix[i][j] == delete {
                    direction_matrix[i][j].push('D'); // a deletion
                } else {
                    direction_matrix[i][j].push('I'); // an insertion
                }
            }
        }
    }
    // println!("matrix: {:?}", direction_matrix);
    // trackback to build the optimal aligned sequences... bottom right to top left
    let mut aligned_sequence1 = String::new();
    let mut aligned_sequence2 = String::new();
    let mut x = m;
    let mut y = n;

    while x > 0 || y > 0 {
        let current_head = direction_matrix[x][y].head.take();
        if x > 0 && y > 0 && current_head.as_ref().is_some_and(|v| v.data == 'M') {
            aligned_sequence1.push(seq1.chars().nth(x - 1).unwrap());
            aligned_sequence2.push(seq2.chars().nth(y - 1).unwrap());
            x -= 1;
            y -= 1;
        } else if x > 0 && current_head.as_ref().is_some_and(|v| v.data == 'D') {
            aligned_sequence1.push(seq1.chars().nth(x - 1).unwrap());
            aligned_sequence2.push('-');
            x -= 1;
        } else {
            aligned_sequence1.push('-');
            aligned_sequence2.push(seq2.chars().nth(y - 1).unwrap());
            y -= 1;
        }
    }
    // Reverse the aligned sequences because we built them backwards
    let aligned_sequence1: String = aligned_sequence1.chars().rev().collect();
    let aligned_sequence2: String = aligned_sequence2.chars().rev().collect();

    Ok((aligned_sequence1, aligned_sequence2))
}

fn get_match(score: isize, char1: char, char2: char) -> isize {
    //let mut match_score = 0;
    if char1.eq_ignore_ascii_case(&char2) {
        return score + 1;
    } else { score - 1 }
}