# Third-Party Libraries Cheatsheet for Face Auth Workshop

This cheatsheet covers the essential parts of **candle_***, **image**, and **serde** libraries used across exercises 01-05.

---

## 🔥 Candle Framework


### Basic Tensor Operations

#### Creating Tensors
```rust
// From vector with shape
let data: Vec<u8> = image_data;
let tensor = Tensor::from_vec(data, (height, width, channels), &Device::Cpu)?;

// From array/slice
let mean = [0.485, 0.456, 0.406];
let mean_tensor = Tensor::new(&mean, &Device::Cpu)?;

// Reshape tensor  
let reshaped = tensor.reshape((3, 1, 1))?;
```

#### Tensor Shape Manipulation
```rust
// Permute dimensions (e.g., HWC to CHW)
let tensor = tensor.permute((2, 0, 1))?;

// Add batch dimension
let batched = tensor.unsqueeze(0)?;

// Remove singleton dimensions
let squeezed = tensor.squeeze(0)?.squeeze(0)?;
```

#### Data Type Conversions
```rust
// Convert to different data types
let float_tensor = tensor.to_dtype(DType::F32)?;

// Scale values (e.g., 0-255 to 0-1)
let normalized = tensor.to_dtype(DType::F32)? / 255.0;
```

#### Mathematical Operations

**Broadcasting Operations**: These automatically expand tensors to compatible shapes for element-wise operations.

```rust
// Broadcasting rules: smaller tensors are "stretched" to match larger ones
// Example: (3, 224, 224) + (3, 1, 1) = (3, 224, 224)
// The (3, 1, 1) tensor gets repeated across all 224x224 pixels

let result = tensor1.broadcast_add(&tensor2)?;     // Addition
let result = tensor1.broadcast_sub(&tensor2)?;     // Subtraction  
let result = tensor1.broadcast_mul(&tensor2)?;     // Multiplication
let result = tensor1.broadcast_div(&tensor2)?;     // Division

// Matrix multiplication (no broadcasting - strict dimension requirements)
let result = tensor_a.matmul(&tensor_b)?;

// Transpose swaps two dimensions
let transposed = tensor.transpose(0, 1)?;  // Swap dims 0 and 1
```

**How Broadcasting Works**:
- Dimensions are aligned from the right (trailing dimensions first)
- Missing dimensions are treated as size 1
- Dimensions of size 1 are stretched to match the other tensor
- Example: `(256,)` + `(3, 224, 224)` becomes `(1, 1, 256)` + `(3, 224, 224)` → `(3, 224, 224)`

#### Reduction Operations

**What `keepdim` means**: Maintains the original number of dimensions by keeping reduced dims as size 1.

```rust
// sum_keepdim example:
// Input:  (2, 3, 4) tensor
// .sum(1) → (2, 4)        # dimension 1 disappears
// .sum_keepdim(1) → (2, 1, 4)  # dimension 1 becomes size 1

let sum = tensor.sum_keepdim(1)?;  // Sum along dim 1, keep dim structure

// Element-wise operations
let sqrt_tensor = tensor.sqrt()?;   // √x for each element
let squared = tensor.sqr()?;        // x² for each element
```

**Why keepdim matters**: Preserves tensor shape for broadcasting operations. Without it, you can't broadcast the result back to the original tensor shape.

#### Extracting Values
```rust
// Single scalar value
let scalar: f32 = tensor.to_vec0()?;

// 1D vector
let values: Vec<f32> = tensor.to_vec1()?;

// Flatten all dimensions and get vector
let flattened: Vec<f32> = tensor.flatten_all()?.to_vec1()?;
```

### L2 Normalization (Essential for Embeddings)

**What it does**: Scales vectors to unit length while preserving direction. Essential for cosine similarity.

```rust
fn normalize_l2(v: &Tensor) -> Result<Tensor> {
    // Step by step breakdown:
    // 1. v.sqr()? - Square each element: [a, b, c] → [a², b², c²]
    // 2. .sum_keepdim(1)? - Sum along dimension 1, keep the dimension: [a²+b²+c²]
    // 3. .sqrt()? - Take square root: √(a²+b²+c²) = ||v||₂ (L2 norm)
    // 4. v.broadcast_div(...) - Divide original vector by its norm
    
    Ok(v.broadcast_div(&v.sqr()?.sum_keepdim(1)?.sqrt()?)?)
}
```

**Why use it**: After L2 normalization, `||v||₂ = 1`, which means:
- Cosine similarity becomes just a dot product
- Removes magnitude bias - focuses only on direction
- Essential for fair comparison of embeddings

### Cosine Similarity with Tensors
```rust
fn cosine_similarity(emb_a: &Tensor, emb_b: &Tensor) -> Result<f32> {
    let emb_a = normalize_l2(emb_a)?;
    let emb_b = normalize_l2(emb_b)?;
    let similarity = emb_a.matmul(&emb_b.transpose(0, 1)?)?;
    let similarity_value = similarity.squeeze(0)?.squeeze(0)?.to_vec0::<f32>()?;
    Ok(similarity_value)
}
```

### Model Loading & Usage

**Core Concepts**: Loading pre-trained models and running inference.

```rust
// Pattern for loading models from Hugging Face Hub
fn load_model_from_hf(model_name: &str) -> Result<SomeModelType> {
    let device = &Device::Cpu;
    
    // 1. Download model weights from Hugging Face
    let api = hf_hub::api::sync::Api::new()?;
    let api = api.model(model_name.to_string());
    let model_file = api.get("model.safetensors")?;
    
    // 2. Create VarBuilder from downloaded weights
    let vb = unsafe { 
        VarBuilder::from_mmaped_safetensors(&[model_file], DType::F32, &device)? 
    };
    
    // 3. Instantiate model architecture with loaded weights
    let model = SomeModel::load(&vb, &config)?;
    
    Ok(model)
}

// Pattern for running inference
fn run_inference(model: &impl Module, input: &Tensor) -> Result<Tensor> {
    // Ensure input has correct batch dimension
    let batched_input = if input.dim(0)? == 3 {  // (C,H,W)
        input.unsqueeze(0)?  // Add batch: (1,C,H,W)
    } else {
        input.clone()  // Already batched
    };
    
    // Forward pass through the model
    let output = model.forward(&batched_input)?;
    Ok(output)
}
```

**Key Points**:
- `VarBuilder`: Loads pre-trained weights from `.safetensors` files
- `Module::forward()`: Standard interface for neural network inference  
- **Batch Dimension**: Most models expect `(batch_size, channels, height, width)`
- **Device Management**: Ensure model and input tensors are on same device

---

## 🖼️ Image Processing

### Dependencies
```toml
[dependencies]
image = "0.25.6"
```

### Essential Imports
```rust
use image::{ImageReader, ImageFormat};
```

### Loading and Processing Images
```rust
// Load image from file path
let img = image::ImageReader::open(path)?
    .decode()?;

// Resize image (multiple resize methods)
let img = img.resize_to_fill(
    224,   // width
    224,   // height  
    image::imageops::FilterType::Triangle,  // filter type
);

// Convert to RGB8 format
let img = img.to_rgb8();

// Extract raw pixel data
let data: Vec<u8> = img.into_raw();  // Returns Vec<u8> with RGB values
```

### Filter Types
```rust
// Available filter types for resizing
image::imageops::FilterType::Triangle    // Good general purpose
image::imageops::FilterType::Lanczos3   // High quality
image::imageops::FilterType::Nearest    // Fastest, pixelated
image::imageops::FilterType::CatmullRom  // Sharp results
```


**Key Concept**: The `reshape((3, 1, 1))` creates tensors that broadcast across all pixels:
- Original image: `(3, 224, 224)` - 3 channels, 224×224 pixels  
- Mean/Std: `(3, 1, 1)` - 3 values, broadcasted to each pixel
- Result: Each of the 224×224 pixels gets normalized using its channel's specific mean/std

---

## 📦 Serde (Serialization/Deserialization)


### Defining Serializable Structs
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingRecord {
    ...
}
```

### JSON Serialization
```rust
// Serialize to JSON string (pretty printed)
let json_string = serde_json::to_string_pretty(&records)?;

// Serialize to JSON string (compact)
let json_string = serde_json::to_string(&records)?;

// Write to file
std::fs::write("data.json", json_string)?;
```

### JSON Deserialization
```rust
// Read from file
let content = std::fs::read_to_string("data.json")?;

// Handle empty files
if content.trim().is_empty() {
    return Ok(Vec::new());
}

// Deserialize from JSON string
let records: Vec<EmbeddingRecord> = serde_json::from_str(&content)?;
```

### Working with DateTime
```rust
// Create current timestamp
let timestamp = chrono::Utc::now();

// DateTime automatically serializes to ISO 8601 string in JSON
```

### Working with UUIDs
```rust
// Generate new UUID
let id = uuid::Uuid::new_v4().to_string();
```


## 🚀 Performance Tips

1. **Tensor Operations**: Use broadcast operations instead of loops when possible
2. **Memory Management**: Reuse tensors when possible, avoid unnecessary clones
3. **Model Loading**: Cache loaded models, don't reload for each inference
4. **Image Processing**: Consider batch processing multiple images at once
5. **Serialization**: Use `serde_json::to_string_pretty` for debugging, regular `to_string` for production

---

## ⚠️ Common Pitfalls

1. **Tensor Shapes**: Always check tensor dimensions before operations
2. **Device Compatibility**: Ensure all tensors are on the same device
3. **Data Types**: Be consistent with DType (F16 vs F32)
4. **Error Handling**: Use `?` operator and proper Result types
5. **Empty Files**: Always handle empty JSON files in deserialization
6. **Path Handling**: Use proper path validation for file operations
