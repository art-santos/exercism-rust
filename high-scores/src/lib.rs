#[derive(Debug)]
pub struct HighScores{
    scores: Vec<u32>,
}


impl HighScores {

    pub fn new(scores: &[u32]) -> Self {
        let scores = HighScores {
            scores: scores.to_vec(),
        };
        scores
    }

    pub fn scores(&self) -> &[u32] {
        let len = &self.scores.len();
        &self.scores[0..*len]
    }

    pub fn latest(&self) -> Option<u32> {
        if self.scores == [] {
            None
        } else {
            let len = &self.scores.len() - 1;
            Some(self.scores[len])
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        if self.scores == [] {
            None
        }else{
            let max = self.scores.iter().max().unwrap();
            Some(*max)
        }
        
    }

    pub fn personal_top_three(&self) -> Vec<u32> { 
        if self.scores == [] {
            vec![]
        }else{
            let len = &self.scores.len();
            let mut vec = self.scores.clone();
            vec.sort_by(|a, b| b.cmp(a));

            if len < &3 {
                vec[0..*len].to_vec()
            }else{
                vec[0..3].to_vec()
            }
        }       

    }
}
