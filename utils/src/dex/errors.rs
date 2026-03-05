#[derive(Debug)]
pub enum MintTransactionError {
    NotEnoughKeys,
    CantFindTokenAddress,
    NoTransactionFound,
    CouldNotParseTransaction,
}