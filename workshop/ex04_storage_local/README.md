# Exercise 4: Local Storage

In this exercise, you'll implement a local file-based storage system for embedding records. This builds on the concepts from the main face authentication application.

## Learning Objectives

- Define data structures with proper serialization
- Implement traits for storage abstraction
- Work with JSON file storage
- Handle file I/O operations
- Apply concepts from the main application

## Task Overview

The skeleton code is provided with struct and trait definitions. You need to implement the functionality for these components:

1. **LocalFileStorage Methods**: Implement the constructor and file I/O operations
2. **EmbeddingStorage Trait Implementation**: Implement storage operations for LocalFileStorage  
3. **Storage Factory Function**: Implement `open_temp_storage()` to create storage instances

Note: `store_dummy()` is already implemented as an example and to enable testing.

## Implementation Guide

The code already provides:
- `EmbeddingRecord` struct with all required fields and derives
- `EmbeddingStorage` trait with method signatures
- `LocalFileStorage` struct skeleton with method stubs

### Your Tasks

#### 1. LocalFileStorage Constructor and Helper Methods

Implement these methods in `LocalFileStorage`:

**`new(file_path: String) -> Result<Self>`**
- Simply store the file_path in the struct

**`load_records(&self) -> Result<Vec<EmbeddingRecord>>`**
- Check if file exists, return empty Vec if not
- Read file content as string
- Handle empty files gracefully
- Deserialize JSON to `Vec<EmbeddingRecord>`

**`save_records(&self, records: &[EmbeddingRecord]) -> Result<()>`**
- Serialize records to JSON string (use `serde_json::to_string_pretty`)
- Write string to file

#### 2. EmbeddingStorage Trait Implementation

Implement these methods for `LocalFileStorage`:

**`store_embedding(&mut self, record: EmbeddingRecord) -> Result<()>`**
- Load existing records
- Add new record to the list
- Save the updated list

**`get_embedding(&self, id: &str) -> Result<Option<EmbeddingRecord>>`**
- Load all records
- Find the record with matching id
- Return `Some(record)` if found, `None` if not

**`get_all_embeddings(&self) -> Result<Vec<EmbeddingRecord>>`**
- Simply call `load_records`

**`delete_embedding(&mut self, id: &str) -> Result<bool>`**
- Load existing records
- Keep track of initial length
- Remove records with matching id
- Save updated records if anything was removed
- Return true if something was deleted, false otherwise

#### 3. Helper Functions

**`open_temp_storage() -> Result<Box<dyn EmbeddingStorage>>`**
- Generate a unique filename using Uuid: `format!("workshop_local_{}.json", Uuid::new_v4())`
- Create a LocalFileStorage with that path
- Return it as a `Box<dyn EmbeddingStorage>`

**`store_dummy(storage: &mut Box<dyn EmbeddingStorage>, name: &str, embedding_len: usize) -> Result<String>`** ✅ *Already implemented*
This function is provided as a complete implementation to:
- Show how to create an `EmbeddingRecord` following the pattern from `app/src/register.rs`
- Demonstrate proper usage of the storage trait
- Enable the test to run once you implement the storage methods

## Reference Implementation

Look at `app/src/register.rs` to see how EmbeddingRecord is used in the main application. The pattern there shows:
- How to create a record with UUID and timestamp
- How to populate metadata
- How to store the record using the storage trait

## Testing

Run the test with:
```bash
cargo test -- --ignored
```

The test will verify that you can store and retrieve records correctly.

## Tips

1. Handle the case where the JSON file doesn't exist yet
2. Use `serde_json` for serialization/deserialization
3. Use `uuid::Uuid::new_v4()` for generating unique IDs
4. Use `chrono::Utc::now()` for timestamps
5. Consider edge cases like empty files

## Success Criteria

- All TODO items are implemented
- The test passes
- You can store and retrieve embedding records
- The JSON file format is readable and valid

This exercise demonstrates the storage layer that's crucial for the face authentication system's ability to persist user embeddings between sessions.
