use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    errors::ChromaError,
    execution::{
        operator::{wrap, TaskResult},
        operators::pull_log::{PullLogsInput, PullLogsOperator, PullLogsOutput},
    },
    log::log::{Log, PullLogsError},
    sysdb::{self, sysdb::SysDb},
    system::{Component, ComponentContext, Receiver, System},
    types::{GetVectorsResult, Segment},
};
use async_trait::async_trait;
use tracing::Span;
use uuid::Uuid;

use super::common::get_hnsw_segment_from_id;

#[derive(Debug)]
enum ExecutionState {
    Pending,
    PullLogs,
    // IMPL NOTE: read vectors should filter out the vectors that are not present in the index.
    ReadVectors,
    MergeResults,
}

#[derive(Debug)]
pub struct GetVectorsOrchestrator {
    state: ExecutionState,
    // Component Execution
    system: System,
    // Query state
    get_ids: Vec<String>,
    hnsw_segment_id: Uuid,
    // State fetched or created for query execution
    record_segment: Option<Segment>,
    // // query_vectors index to the result
    // hnsw_result_offset_ids: HashMap<usize, Vec<usize>>,
    // hnsw_result_distances: HashMap<usize, Vec<f32>>,
    // brute_force_result_user_ids: HashMap<usize, Vec<String>>,
    // brute_force_result_distances: HashMap<usize, Vec<f32>>,
    // brute_force_result_embeddings: HashMap<usize, Vec<Vec<f32>>>,
    // // Task id to query_vectors index
    // hnsw_task_id_to_query_index: HashMap<Uuid, usize>,
    // brute_force_task_id_to_query_index: HashMap<Uuid, usize>,
    // merge_task_id_to_query_index: HashMap<Uuid, usize>,
    // // Result state
    // results: Option<Vec<Vec<VectorQueryResult>>>,
    // // State machine management
    // merge_dependency_count: u32,
    // finish_dependency_count: u32,
    // // Services
    log: Box<dyn Log>,
    sysdb: Box<dyn SysDb>,
    // dispatcher: Box<dyn Receiver<TaskMessage>>,
    // hnsw_index_provider: HnswIndexProvider,
    // blockfile_provider: BlockfileProvider,
    // Result channel
    result_channel:
        Option<tokio::sync::oneshot::Sender<Result<GetVectorsResult, Box<dyn ChromaError>>>>,
}

impl GetVectorsOrchestrator {
    pub fn new(
        system: System,
        get_ids: Vec<String>,
        hnsw_segment_id: Uuid,
        log: Box<dyn Log>,
        sysdb: Box<dyn SysDb>,
    ) -> Self {
        Self {
            state: ExecutionState::Pending,
            system,
            get_ids,
            hnsw_segment_id,
            log: log,
            sysdb: sysdb,
            record_segment: None,
            result_channel: None,
        }
    }

    async fn pull_logs(
        &mut self,
        self_address: Box<dyn Receiver<TaskResult<PullLogsOutput, PullLogsError>>>,
    ) {
        self.state = ExecutionState::PullLogs;
        let operator = PullLogsOperator::new(self.log.clone());
        let end_timestamp = SystemTime::now().duration_since(UNIX_EPOCH);
        let end_timestamp = match end_timestamp {
            // TODO: change protobuf definition to use u64 instead of i64
            Ok(end_timestamp) => end_timestamp.as_nanos() as i64,
            Err(e) => {
                // Log an error and reply + return
                return;
            }
        };

        // let collection = self
        //     .collection
        //     .as_ref()
        //     .expect("State machine invariant violation. The collection is not set when pulling logs. This should never happen.");

        // let input = PullLogsInput::new(
        //     collection.id,
        //     // The collection log position is inclusive, and we want to start from the next log
        //     collection.log_position + 1,
        //     100,
        //     None,
        //     Some(end_timestamp),
        // );
        // let task = wrap(operator, input, self_address);
        // // Wrap the task with current span as the parent. The worker then executes it
        // // inside a child span with this parent.
        // match self.dispatcher.send(task, Some(Span::current())).await {
        //     Ok(_) => (),
        //     Err(e) => {
        //         // TODO: log an error and reply to caller
        //     }
        // }
    }

    fn terminate_with_error(&mut self, error: Box<dyn ChromaError>, ctx: &ComponentContext<Self>) {
        let result_channel = self
            .result_channel
            .take()
            .expect("Invariant violation. Result channel is not set.");
        match result_channel.send(Err(error)) {
            Ok(_) => (),
            Err(e) => {
                // Log an error - this implied the listener was dropped
                println!("[HnswQueryOrchestrator] Result channel dropped before sending error");
            }
        }
        // Cancel the orchestrator so it stops processing
        ctx.cancellation_token.cancel();
    }

    ///  Run the orchestrator and return the result.
    ///  # Note
    ///  Use this over spawning the component directly. This method will start the component and
    ///  wait for it to finish before returning the result.
    pub(crate) async fn run(mut self) -> Result<GetVectorsResult, Box<dyn ChromaError>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.result_channel = Some(tx);
        let mut handle = self.system.clone().start_component(self);
        let result = rx.await;
        handle.stop();
        result.unwrap()
    }
}

// ============== Component Implementation ==============

#[async_trait]
impl Component for GetVectorsOrchestrator {
    fn get_name() -> &'static str {
        "GetVectorsOrchestrator"
    }

    fn queue_size(&self) -> usize {
        1000
    }

    async fn on_start(&mut self, ctx: &ComponentContext<Self>) {
        // Populate the orchestrator with the initial state - The HNSW Segment, The Record Segment and the Collection
        let hnsw_segment =
            match get_hnsw_segment_from_id(self.sysdb.clone(), &self.hnsw_segment_id).await {
                Ok(segment) => segment,
                Err(e) => {
                    self.terminate_with_error(e, ctx);
                    return;
                }
            };

        // let collection_id = match &hnsw_segment.collection {
        //     Some(collection_id) => collection_id,
        //     None => {
        //         self.terminate_with_error(
        //             Box::new(HnswSegmentQueryError::HnswSegmentHasNoCollection),
        //             ctx,
        //         );
        //         return;
        //     }
        // };

        // let collection = match self.get_collection(self.sysdb.clone(), collection_id).await {
        //     Ok(collection) => collection,
        //     Err(e) => {
        //         self.terminate_with_error(e, ctx);
        //         return;
        //     }
        // };

        // // Validate that the collection has a dimension set. Downstream steps will rely on this
        // // so that they can unwrap the dimension without checking for None
        // if collection.dimension.is_none() {
        //     self.terminate_with_error(
        //         Box::new(HnswSegmentQueryError::CollectionHasNoDimension),
        //         ctx,
        //     );
        //     return;
        // };

        // let record_segment = match self
        //     .get_record_segment_for_collection(self.sysdb.clone(), collection_id)
        //     .await
        // {
        //     Ok(segment) => segment,
        //     Err(e) => {
        //         self.terminate_with_error(e, ctx);
        //         return;
        //     }
        // };

        // match IndexConfig::from_segment(&hnsw_segment, collection.dimension.unwrap()) {
        //     Ok(index_config) => {
        //         self.index_config = Some(index_config);

        //         // Normalize the query vectors if we are using the cosine similarity
        //         if self.index_config.as_ref().unwrap().distance_function == DistanceFunction::Cosine
        //         {
        //             for query_vector in self.query_vectors.iter_mut() {
        //                 *query_vector = normalize(query_vector);
        //             }
        //         }
        //     }
        //     Err(e) => {
        //         self.terminate_with_error(e, ctx);
        //         return;
        //     }
        // }

        // self.record_segment = Some(record_segment);
        // self.hnsw_segment = Some(hnsw_segment);
        // self.collection = Some(collection);

        // self.pull_logs(ctx.sender.as_receiver()).await;
    }
}

// ============== Handlers ==============
