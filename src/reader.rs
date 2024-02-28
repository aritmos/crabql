use super::{
    super::{checker::Checker, expr::ExprResult},
    column::Column,
    table::Table,
};

// The query that is being built
type Q = ();

// The context/state of the reader.
// Used for validating expressions
type T = ();

// TODO: Remove Q and T generics once their types are settled
pub struct Reader<'c, C> {
    pub(super) checker: &'c mut C,
    pub(super) state: T,
    pub(super) query: Q,
}

pub struct SealedReader<T> {
    state: T,
}

impl<'c, C> Reader<'c, C> {
    pub fn new(checker: &'c mut C) -> Self
    where
        C: Checker,
    {
        Reader {
            checker,
            state: T::default(),
            query: Q::default(),
        }
    }

    /// Adds a table into its state
    pub fn table(self, _id: &str) -> Self {
        todo!()
    }

    /// Selects the given rows for reading, returns a `SealedReader` that cannot be internally
    /// modified further.
    pub fn select<'t, 'col, Func, ColIter>(self, _f: Func) -> Result<SealedReader<T>, String>
    where
        Func: FnOnce(Table<'t>) -> ColIter,
        ColIter: IntoIterator<Item = Column<'col>>,
        't: 'col,
    {
        todo!()
    }

    /// Selects all rows for reading, returns a `SealedReader` that cannot be internally modified
    /// further.
    pub fn select_all(self) -> Result<SealedReader<T>, String> {
        todo!()
    }

    /// Filters the rows in the current table
    pub fn filter<F>(self, _f: F) -> Self
    where
        F: FnOnce(Table) -> ExprResult,
    {
        todo!()
    }
}