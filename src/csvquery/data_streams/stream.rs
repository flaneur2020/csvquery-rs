use crate::csvquery::data_types::DataBlock;
use crate::csvquery::error::CSVQueryResult;
use futures::stream::Stream;

pub type DataBlockStream =
    std::pin::Pin<Box<dyn Stream<Item = CSVQueryResult<DataBlock>> + Sync + Send>>;
