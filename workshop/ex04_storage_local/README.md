# Exercise 04: Local File Storage for Face Embeddings

## Overview

This exercise teaches you how to build a persistent storage system for face embeddings using local JSON files. You'll implement a complete storage solution that can save, retrieve, and manage embedding records - essential for any face authentication system that needs to remember users between sessions.

## Why Storage Matters

Face authentication systems need persistent storage to:
- **Remember Users**: Store embeddings from registration for future login attempts
- **Enable Comparison**: Retrieve stored embeddings to compare against live captures
- **Manage Identities**: Track multiple embeddings per user for better accuracy
- **Persist Data**: Maintain user data across application restarts

## Architecture Overview

The storage system uses a trait-based design for flexibility:

```rust
// Data structure for each stored embedding
pub struct EmbeddingRecord {
    pub id: String,                                    // Unique identifier
    pub name: String,                                  // User name
    pub embedding: Vec<f32>,                          // Face embedding vector
    pub created_at: chrono::DateTime<chrono::Utc>,    // Timestamp
    pub metadata: HashMap<String, String>,            // Additional data
}

// Storage interface (trait)
pub trait EmbeddingStorage {
    fn store_embedding(&mut self, record: EmbeddingRecord) -> Result<()>;
    fn get_embedding(&self, id: &str) -> Result<Option<EmbeddingRecord>>;
    fn get_all_embeddings(&self) -> Result<Vec<EmbeddingRecord>>;
    fn delete_embedding(&mut self, id: &str) -> Result<bool>;
}

// Local file implementation
pub struct LocalFileStorage {
    file_path: String,
    data: Mutex<HashMap<String, EmbeddingRecord>>,
}
```

## Your Tasks

### Task 1: Implement `LocalFileStorage::new()`

```rust
pub fn new(file_path: String) -> Result<Self>
```

This constructor should:
1. **Create Storage Instance**: Initialize the struct with the file path
2. **Set Up In-Memory Cache**: Create a Mutex-protected HashMap for fast access
3. **Load Existing Data**: Call `load_data()` to read any existing records

#### Implementation Steps:
```rust
let storage = LocalFileStorage {
    file_path,
    data: Mutex::new(HashMap::new()),
};

// Load existing data if file exists
storage.load_data()?;
Ok(storage)
```

### Task 2: Implement `load_data()`

```rust
fn load_data(&self) -> Result<()>
```

This method should:
1. **Check File Existence**: Return early if file doesn't exist
2. **Handle Empty Files**: Deal gracefully with zero-byte files
3. **Parse JSON**: Deserialize the file content into a HashMap
4. **Update Cache**: Store the loaded data in the in-memory HashMap

#### Implementation Steps:
```rust
// Check if file exists
if !Path::new(&self.file_path).exists() {
    return Ok(());
}

// Check if file is empty
let metadata = fs::metadata(&self.file_path)?;
if metadata.len() == 0 {
    return Ok(());
}

// Read and parse JSON
let file = File::open(&self.file_path)?;
let reader = BufReader::new(file);
let data: HashMap<String, EmbeddingRecord> = serde_json::from_reader(reader)?;

// Update in-memory cache
if let Ok(mut guard) = self.data.lock() {
    *guard = data;
}
```

### Task 3: Implement `save_data()`

```rust
fn save_data(&self) -> Result<()>
```

This method should:
1. **Create Directory**: Ensure the parent directory exists
2. **Open File**: Create or truncate the target file
3. **Serialize Data**: Convert the HashMap to pretty-printed JSON
4. **Write File**: Save the JSON to disk

#### Implementation Steps:
```rust
// Create directory if needed
if let Some(parent) = Path::new(&self.file_path).parent() {
    fs::create_dir_all(parent)?;
}

// Open file for writing
let file = OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .open(&self.file_path)?;

// Write JSON data
if let Ok(guard) = self.data.lock() {
    serde_json::to_writer_pretty(file, &*guard)?;
}
```

### Task 4: Implement `EmbeddingStorage` Trait

#### `store_embedding()`
```rust
fn store_embedding(&mut self, record: EmbeddingRecord) -> Result<()>
```

1. **Add to Cache**: Insert the record into the in-memory HashMap
2. **Persist to Disk**: Call `save_data()` to write changes

#### `get_embedding()`
```rust
fn get_embedding(&self, id: &str) -> Result<Option<EmbeddingRecord>>
```

1. **Search Cache**: Look up the record by ID in the HashMap
2. **Return Result**: Clone and return the record if found

#### `get_all_embeddings()`
```rust
fn get_all_embeddings(&self) -> Result<Vec<EmbeddingRecord>>
```

1. **Collect All**: Get all values from the HashMap
2. **Return Vector**: Convert to a Vec of cloned records

#### `delete_embedding()`
```rust
fn delete_embedding(&mut self, id: &str) -> Result<bool>
```

1. **Remove from Cache**: Delete the record from HashMap
2. **Check Success**: Track whether anything was actually removed
3. **Persist Changes**: Save to disk if deletion occurred
4. **Return Status**: Return true if something was deleted

### Task 5: Implement `open_temp_storage()`

```rust
pub fn open_temp_storage() -> Result<(Box<dyn EmbeddingStorage>, String)>
```

This helper function should:
1. **Generate Unique Path**: Create a temporary filename using UUID
2. **Create Storage**: Initialize a LocalFileStorage instance
3. **Return Boxed Trait**: Return as a trait object for flexibility

#### Implementation:
```rust
let path = format!("workshop_local_{}.json", Uuid::new_v4());
let storage = LocalFileStorage::new(path.clone())?;
Ok((Box::new(storage), path))
```

## Technical Details

### JSON File Format:
The storage saves data as a JSON object where keys are record IDs:
```json
{
  "uuid-1": {
    "id": "uuid-1",
    "name": "Alice",
    "embedding": [0.1, 0.2, ...],
    "created_at": "2024-01-01T12:00:00Z",
    "metadata": {}
  },
  "uuid-2": { ... }
}
```

### Concurrency Handling:
- Uses `Mutex<HashMap>` for thread-safe access to in-memory data
- Locks are held briefly during read/write operations
- File I/O is synchronized through the mutex

### Error Handling:
- Gracefully handles missing files (starts with empty storage)
- Deals with corrupted JSON (logs warning, starts fresh)
- Proper error propagation using `Result<T>`

## Testing

The provided tests verify:
- **Basic Storage**: Can store and retrieve records
- **Sorting**: Results are returned in correct order
- **Limits**: Respects k-parameter for top-k queries
- **Empty Storage**: Handles empty storage gracefully

Run tests with:
```bash
cargo test
```

## File Management

The storage system:
- **Auto-creates** directories as needed
- **Handles** missing files gracefully
- **Overwrites** files completely on each save (simple but safe)
- **Uses** pretty-printed JSON for human readability

## Production Considerations

This simple file-based approach works well for:
- **Development and Testing**: Easy to inspect and debug
- **Small Datasets**: Hundreds to thousands of embeddings
- **Single-User Applications**: No concurrent access needed

For production systems, consider:
- **Database Storage**: PostgreSQL with pgvector extension
- **Vector Databases**: Qdrant, Pinecone, Weaviate
- **Concurrent Access**: Proper locking mechanisms
- **Backup Strategies**: Regular data backups

## Next Steps

After completing this exercise, you'll be ready to:
- Implement similarity search and retrieval (Exercise 05)
- Understand how storage enables face authentication
- Build more sophisticated storage solutions
- Integrate with production databases

This storage foundation is crucial for the face authentication system's ability to persist and retrieve user embeddings efficiently.
