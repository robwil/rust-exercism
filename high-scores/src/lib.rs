use std::collections::BinaryHeap;

#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
    top_scores: Vec<u32>,
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        // since there is no mutating the HighScores once created, we can calculate top 3 scores in constructor
        // using max heap here has time complexity O(3 log N) == O(log N)
        let mut heap: BinaryHeap<u32> = BinaryHeap::with_capacity(scores.len());
        for score in scores {
            heap.push(*score);
        }
        let mut top_scores = vec![];
        let mut i = 0;
        while let Some(score) = heap.pop() {
            top_scores.push(score);
            i += 1;
            if i >= 3 {
                break;
            }
        }
        HighScores { scores, top_scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        if self.scores.is_empty() {
            return None;
        }
        Some(self.scores[self.scores.len() - 1])
    }

    pub fn personal_best(&self) -> Option<u32> {
        if self.top_scores.is_empty() {
            return None;
        }
        Some(self.top_scores[0])
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        self.top_scores.clone()
    }
}
