pub trait SkipNth: Iterator + Sized {
    fn skip_nth(self, n: usize) -> NthSkipper<Self>;
}

impl<Iter> SkipNth for Iter
where
    Iter: Iterator,
{
    fn skip_nth(self, n: usize) -> NthSkipper<Self> {
        NthSkipper {
            iterator: self,
            index_to_skip: Some(n),
        }
    }
}

#[derive(Clone)]
pub struct NthSkipper<Iter>
where
    Iter: Iterator,
{
    iterator: Iter,
    index_to_skip: Option<usize>,
}

impl<Iter> Iterator for NthSkipper<Iter>
where
    Iter: Iterator,
{
    type Item = Iter::Item;
    fn next(&mut self) -> Option<Self::Item> {
        match self.index_to_skip {
            None => self.iterator.next(), // we already skipped :)
            Some(n) => {
                self.index_to_skip = n.checked_sub(1);
                if self.index_to_skip.is_none() {
                    // we are skipping!
                    self.iterator.next();
                }
                self.iterator.next()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn some_skippage() {
        let orig_list = [1, 2, 3, 5];
        let skipped_list: Vec<_> =
            orig_list.iter().copied().skip_nth(2).collect();
        assert_eq!(&skipped_list, &[1, 2, 5]);
    }
}
