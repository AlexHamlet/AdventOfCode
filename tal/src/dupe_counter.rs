pub trait CountDupes {
    type Item;
    fn count_dupes(self) -> impl Iterator<Item = (Self::Item, usize)>;
}

impl<T, Iter> CountDupes for Iter
where
    T: PartialEq,
    Iter: Iterator<Item = T>,
{
    type Item = T;
    fn count_dupes(mut self) -> impl Iterator<Item = (Self::Item, usize)> {
        let first_item = self.next();
        DupeCounter {
            iterator: self,
            current_item: first_item,
            current_count: 1,
        }
    }
}

struct DupeCounter<T, Iter>
where
    T: PartialEq,
    Iter: Iterator<Item = T>,
{
    iterator: Iter,
    current_item: Option<T>,
    current_count: usize,
}

impl<T, Iter> Iterator for DupeCounter<T, Iter>
where
    T: PartialEq,
    Iter: Iterator<Item = T>,
{
    type Item = (T, usize);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let new_item = self.iterator.next();
            if new_item.is_some() && new_item == self.current_item {
                self.current_count += 1;
            } else {
                let old_item = self.current_item.take()?;
                let old_count = self.current_count;
                self.current_item = new_item;
                self.current_count = 1;
                return Some((old_item, old_count));
            }
        }
    }
}
