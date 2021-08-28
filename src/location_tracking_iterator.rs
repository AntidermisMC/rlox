use crate::location::Location;

pub struct LocationTrackingIterator<T: Iterator<Item=char>> {
    location: Location,
    it: T,
    peek_1: Option<char>,
    peek_2: Option<char>,
}

impl<T: Iterator<Item=char>> Iterator for LocationTrackingIterator<T> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {

        // Something has been peeked already
        if let Some(c1) = self.peek_1 {
            self.peek_1 = self.peek_2.take();
            self.location.advance(c1);
            return Some(c1);
        }

        // Nothing was peeked
        let c = self.it.next();
        match c {
            None => None,
            Some(c) => {
                self.location.advance(c);
                Some(c)
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.it.size_hint()
    }
}

impl<T: Iterator<Item=char>> LocationTrackingIterator<T> {
    pub fn get_location(&self) -> Location {
        self.location
    }

    pub fn new(it: T) -> Self {
        LocationTrackingIterator {
            location: Location::start(),
            it,
            peek_1: None,
            peek_2: None,
        }
    }

    pub fn peek(&mut self) -> Option<&<Self as Iterator>::Item> {
        if self.peek_1 == None {
            self.peek_1 = self.it.next();
        }
        self.peek_1.as_ref()
    }

    pub fn peek_location(&mut self) -> Option<Location> {
        if let Some(&c) = self.peek() {
            let mut loc = self.location;
            loc.advance(c);
            Some(loc)
        }
        else {
            None
        }
    }

    pub fn peek_2(&mut self) -> Option<&<Self as Iterator>::Item> {
        if self.peek_2 != None {
            self.peek_2.as_ref()
        }
        else if let Some(_) = self.peek() {
            self.peek_2 = self.it.next();
            self.peek_2.as_ref()
        }
        else {
            None
        }
    }

    pub fn peek_location_2(&mut self) -> Option<Location> {
        if let Some(&c) = self.peek_2() {
            let mut loc = self.peek_location().unwrap();
            loc.advance(c);
            Some(loc)
        }
        else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::location_tracking_iterator::LocationTrackingIterator;
    use crate::location::Location;

    fn assert_eq(expected: &str) {
        assert_eq!(LocationTrackingIterator::new(expected.chars()).collect::<String>(), expected)
    }

    #[test]
    fn next_returns_same_as_base_iterator() {
        assert_eq("");
        assert_eq("\n");
        assert_eq("one line");
        assert_eq("one\ntwo");
        assert_eq("one\ntwo\n");
    }

    #[test]
    fn locations_are_valid() {
        let mut it = LocationTrackingIterator::new("1\n2\n".chars());
        assert_eq!(it.get_location(), Location::start());
        assert_eq!(it.peek_location(), Some(Location::new(1, 1)));
        assert_eq!(it.peek_location_2(), Some(Location::new(2, 0)));
        assert_eq!(it.get_location(), Location::start());
        it.next();
        assert_eq!(it.get_location(), Location::new(1, 1));
        it.next();
        assert_eq!(it.get_location(), Location::new(2, 0));
        it.next();
        assert_eq!(it.get_location(), Location::new(2, 1));
        it.next();
        assert_eq!(it.get_location(), Location::new(3, 0));
    }

    #[test]
    fn peek_1_and_2() {
        let mut it = LocationTrackingIterator::new("abc".chars());
        assert_eq!(it.peek(), Some(&'a'));
        assert_eq!(it.peek_2(), Some(&'b'));

        assert_eq!(it.next(), Some('a'));
        assert_eq!(it.peek(), Some(&'b'));
        assert_eq!(it.peek_2(), Some(&'c'));

        assert_eq!(it.next(), Some('b'));
        assert_eq!(it.peek(), Some(&'c'));
        assert_eq!(it.peek_2(), None);

        assert_eq!(it.next(), Some('c'));
        assert_eq!(it.peek(), None);
        assert_eq!(it.peek_2(), None);
    }
}