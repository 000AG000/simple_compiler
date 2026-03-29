use crate::sem_parser::Statement;


#[derive(Debug,Clone,PartialEq)]
/// All Types of Execution Frames that can occur in the program
/// Used as identifier in Framekind
pub enum FrameKind {
    Block,
    Loop { remaining: usize },
}

#[derive(Debug,Clone,PartialEq)]
/// Execution Frame for storing the relevant Frame data
pub struct Frame<'a> {
    /// saving the current frame context e.g. for loops 
    pub kind: FrameKind,
    /// statements defined in the frame
    pub statements: & 'a[Statement],
    /// instruction pointer pointing to next statement
    pub ip: usize,
}


impl<'a> Frame<'a>{
    /// crate a new frame with instruction pointer to 0
    pub fn new(kind:FrameKind,statements:&'a [Statement]) -> Self{
        Self { kind, statements, ip: 0 }
    }


}