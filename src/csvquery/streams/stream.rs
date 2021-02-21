use crate::csvquery::data_types::RecordBatch;
use crate::csvquery::error::CQResult;
use futures::stream::Stream;

pub type SendableRecordBatchStream =
    std::pin::Pin<Box<dyn Stream<Item = CQResult<RecordBatch>> + Sync + Send>>;
