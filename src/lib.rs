pub struct Permutator<const N: usize, T> {
    ar: [T;N],
    c: [usize;N], // should be N-1, but this requires unstable feature gate?
    i: usize,
    f: bool,
}
impl<const N: usize, T: Copy + Eq> Permutator<N, T> {
    pub fn new(ar: [T;N]) -> Self {
        Self {
            ar,
            c: [0;N],
            i: 0,
            f: false,
        }
    }
}
impl<const N: usize, T: Copy + Eq> Iterator for Permutator<N, T> {
    type Item = [T;N];

    fn next(&mut self) -> Option<Self::Item> {
        if self.f {
            while self.i < self.ar.len() - 1 {
                if self.c[self.i] == self.i + 1 {
                    self.c[self.i] = 0;
                    self.i += 1;
                } else if self.ar[0] == self.ar[self.i + 1] {
                    self.ar[..=(self.i+1)].rotate_left(1);
                    self.c[self.i] += 1;
                } else {
                    self.ar[..=(self.i+1)].reverse();
                    self.c[self.i] += 1;
                    self.i = 0;
                    return Some(self.ar);
                }
            }
        } else {
            self.f = true;
            return Some(self.ar);
        }
        return None
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn no_duplicates() {
        let permutations = Permutator::new([1,2,2,3,3,3,4,4,4,4]).collect::<Vec<_>>();
        let unique = permutations.iter().collect::<HashSet<_>>();
        assert_eq!(permutations.len(), unique.len());
        assert_eq!(permutations.len(), 12600);
        println!("{} permutations", permutations.len());
    }

    #[test]
    fn doesnt_explode_pc() {
        let seq = [1,2,2,3,3,3,4,4,4,4,5,5,5,5,5];
        let count = Permutator::new(seq).count();
        println!("{count}");
    }
}
