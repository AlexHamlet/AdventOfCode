pub trait ConsecutiveOverlappingPairs: Iterator + Sized {
    fn consecutive_overlapping_pairs(self) -> OverlappingPairs<Self>;
}

impl<Iter> ConsecutiveOverlappingPairs for Iter
where
    Iter: Iterator,
    Iter::Item: Clone,
{
    fn consecutive_overlapping_pairs(mut self) -> OverlappingPairs<Self> {
        let previous = self.next();
        OverlappingPairs {
            iterator: self,
            previous,
        }
    }
}

#[derive(Clone)]
pub struct OverlappingPairs<Iter>
where
    Iter: Iterator,
{
    iterator: Iter,
    previous: Option<Iter::Item>,
}

/* output of #[derive(Clone)] in this case: */
/*
impl<Iter> Clone for OverlappingPairs<Iter>
where
    Iter: Iterator + Clone,
    Iter::Item: Clone,
{
    fn clone(&self) -> Self {
        Self {
            iterator: self.iterator.clone(),
            previous: self.previous.clone(),
        }
    }
}
*/

impl<Iter> Iterator for OverlappingPairs<Iter>
where
    Iter: Iterator,
    Iter::Item: Clone,
{
    type Item = (Iter::Item, Iter::Item);
    fn next(&mut self) -> Option<Self::Item> {
        let previous = self.previous.take()?;
        let current = self.iterator.next()?;
        self.previous = Some(current.clone());
        Some((previous, current))
    }
}

// "apple", "banana", "orange", "mango", "macbook air"
// should yield:
// previous = "apple"
// ("apple", "banana")
// previous = "banana"
// ("banana", "orange")
// previous = "orange"
// ("orange", "mango")
// previous = "mango"
// ("mango", "macbook air")
// previous = "macbook air"
// (and then nones for the rest of time)
// previous = ???

// ["emate"]
// should yield:
// (just nones)

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dissimilar_fruit_exchange() {
        let orig_list = ["apple", "banana", "orange", "mango", "macbook air"];
        let pair_list: Vec<(&str, &str)> = orig_list
            .iter()
            .copied()
            .consecutive_overlapping_pairs()
            .collect();
        assert_eq!(
            pair_list,
            [
                ("apple", "banana"),
                ("banana", "orange"),
                ("orange", "mango"),
                ("mango", "macbook air"),
            ]
        );
    }
}
