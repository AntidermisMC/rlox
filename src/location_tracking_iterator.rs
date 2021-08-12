use crate::location::Location;
use std::iter::Peekable;

pub struct LocationTrackingIterator<T: Iterator<Item=char>> {
    location: Location,
    it: T,
}

impl<T: Iterator<Item=char>> Iterator for LocationTrackingIterator<T> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
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
            it
        }
    }
}

impl<T: Iterator<Item=char>> LocationTrackingIterator<Peekable<T>> {
    pub fn peek(&mut self) -> Option<&char> {
        self.it.peek()
    }

    pub fn peek_location(&mut self) -> Option<Location> {
        let mut loc = self.location;
        let c = self.peek();
        if let Some(c) = c {
        loc.advance(*c);
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
        it.next();
        assert_eq!(it.get_location(), Location::new(1, 1));
        it.next();
        assert_eq!(it.get_location(), Location::new(2, 0));
        it.next();
        assert_eq!(it.get_location(), Location::new(2, 1));
        it.next();
        assert_eq!(it.get_location(), Location::new(3, 0));
    }
}