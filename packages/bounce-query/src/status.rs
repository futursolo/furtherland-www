/// Query Status
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum QueryStatus {
    /// The query is idling.
    ///
    /// For a Query, it means that it's paused.
    /// For a Mutation, this menas that it's not started.
    Idle, // paused for queries, not started for mutations
    /// The query is loading.
    Loading,
    /// The query is successful.
    Ok,
    /// The query has failed with an Error.
    Err,
}
