# TODO

- #### Basic storage engine (Week 1-2)
  - Simple file-based key-value store
  - Basic serialization/deserialization
  - No indexing yet


- #### Schema system (Week 2-3)
  - Macro-based schema definition
  - Type validation
  - Document structure


- #### Indexing (Week 3-4)
  - B-tree implementation or use existing crate
  - Primary key indexing
  - Simple field indexing


- #### Query system (Week 4-5)
  - Basic filtering
  - Range queries
  - Simple aggregations


- #### Persistence & reliability (Week 5-6)
  - Crash recovery
  - Atomic operations
  - File corruption handling

# Updated TODO (Simplified!)

## Week 1-2: CoW B-tree Foundation
- ✅ Paged storage (already done)
- Implement CoW B-tree on existing page infrastructure
- Root pointer management for atomic commits

## Week 3-4: Document Integration
- ✅ Schema system (already done)
- Integrate CoW B-trees with document serialization
- Primary and secondary index creation

## Week 5-6: Concurrency & Polish
- Single-writer, multi-reader transaction manager
- Connection pooling and API finalization
- Performance optimization and testing

## REMOVED: WAL complexity, crash recovery, file corruption handling
# (All handled naturally by CoW B-trees!)