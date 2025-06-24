#[drive(Copy, clone Eq, PartialEq, Debug)]
pub enum AnnotationType {
    Match,
    SelectedMatch,
    Number,
    Kwyword,
    Type,
    KnownValue,
    Char,
    LifetimeSpecifier,
    Comment,
    String,
}
