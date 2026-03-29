fn main() {
    let want_a_string = true;
    let want_b_string = !want_a_string;
    let a = if want_a_string {
        Either::Left("A")
    } else {
        Either::Right(42)
    };

    let b = if want_b_string {
        Either::Left("B")
    } else {
        Either::Right(54)
    };

    if a.is_left() {
        println!("It is a string: {}", a.left().unwrap());
    } else {
        println!("It is a number: {}", a.right().unwrap());
    }
    println!("{}", b.as_ref().right().unwrap());
    println!("{}", b.left().unwrap_or("this is no string :/"));
}

enum Either<L, R> {
    Left(L),
    Right(R)
}

impl<L, R> Either<L, R> {

    /// Consume self and return option that contains the left variant if any
    fn left(self) -> Option<L> { 
        match self {
            Either::Left(l) => Some(l),
            _ => None
        }
    }

    /// Consume self and return option that contains the right variant if any
    fn right(self) -> Option<R> { 
        match self {
            Either::Right(r) => Some(r),
            _ => None
        }
    }

    /// Returns true if self is the left variant
    fn is_left(&self) -> bool { 
        matches!(self, Either::Left(_)) 
    }

    /// Returns true if self is the right variant
    fn is_right(&self) -> bool { 
        matches!(self, Either::Right(_)) 
    }

    /// Returns a new Either that that holds reference to this either inner value
    fn as_ref(&self) -> Either<&L, &R> { 
        match self {
            Either::Left(l) => Either::Left(l),
            Either::Right(r) => Either::Right(r),
        }
    }
}