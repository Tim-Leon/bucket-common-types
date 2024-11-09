///// Every storage engine type is categorised based on:
///// Latency, storage-medium, and redundancy.
///// The cost of storage is highly dependent on those factors.
///// Each storage engine that runs has a name that uniquely identifies it, and a storage engine type.
/////
///// Usually high throughput and low latency can be associate with the use of NVME, and low throughput and high latency is likely cause of the underling storage being HDD.
/////
///// Naming convention:
///// Redundancy-level, Throughput, Latency-level.
///// Storage engine types
//#[derive(
//    Debug, Clone, Default, Eq, PartialEq, strum::EnumString, strum::Display, Serialize, Deserialize,
//)]
//pub enum StorageEngineType {
//    #[default]
//    ReducedRedundancyHddAverag = {
//        pub struct LowAverageHigh {
//
//        }
//    },
//}
//pub struct StorageEngineType {
//    pub latency: u32,
//    pub redundancy: u32,
//    pub throughput: u32,
//}
//pub enum Latency {
//    High,
//    Medium,
//    Low,
//}
//
//pub enum Redundancy {
//    High,
//    Medium,
//    Low,
//}
//
//pub enum Throughput {
//    High,
//    Medium,
//    Low,
//}
//
///// Storage engine type with the specific name of that storage engine.
//pub struct SpecificStorageEngine {
//    pub storage_engine_name: String,
//    pub storage_engine_type: StorageEngineType,
//}
