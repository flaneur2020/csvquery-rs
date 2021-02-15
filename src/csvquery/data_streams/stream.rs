use crate::csvquery::data_types::DataBlock;
use crate::csvquery::error::CQResult;
use futures::stream::Stream;

pub type SendableDataBlockStream =
    std::pin::Pin<Box<dyn Stream<Item = CQResult<DataBlock>> + Sync + Send>>;
