use std::{collections::HashMap, sync::mpsc::TryRecvError};

use crate::engine::{blocks::{block::Block, block_model::BlockModel}, core::worker::Worker, rendering::{chunk_mesher, chunk_renderer}, world::{chunk::Chunk, chunk_manager::ChunkState::{Available, InWork}, chunk_region::ChunkRegion, chunk_state::{ChunkState, GenerationStep}, generation::{self, terrain_generator::{TerrainGenerator, Worldtype}}, voxel_coordinates::*, world::RENDER_DISTANCE}};


/// Instanced once per loaded dimension
pub struct ChunkManager {
    chunks: HashMap<ChunkCoord, ChunkState>,
    generating_chunks: Vec<GeneratingChunk>, // receivers of generating chunks that are being edited by a thread
    meshed_chunks: Vec<MeshedChunk>,
    pub dirty_chunks: Vec<ChunkCoord>,
    freed_chunks: Vec<Chunk>, // the chunk will move into this queue on deletion and save to disk before being destroyed
    pub chunks_generating: u32,
    pub chunks_loaded: u32,
}

pub struct GeneratingChunk {
    pub coord: ChunkCoord,
    pub receiver: std::sync::mpsc::Receiver<Chunk>,
}

pub struct MeshedChunk {
    pub coord: ChunkCoord,
    pub receiver: std::sync::mpsc::Receiver<Vec<f32>>,
}

/// A point that loads- and keeps chunks around it loaded
pub struct ChunkLoader {
    center: BlockCoord,
    radius: u32,
}

// TODO: all methods should be callable ingame with a command
impl ChunkManager {

    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
            generating_chunks: Vec::new(),
            meshed_chunks: Vec::new(),
            dirty_chunks: Vec::new(),
            freed_chunks: Vec::new(),
            chunks_generating: 0,
            chunks_loaded: 0,
        }
    }

    pub fn load_chunks_in_radius(
        &mut self,
        x: i32,
        z: i32,
        radius: u32,
        worker: &Worker,
        block_models: &Vec<BlockModel>,
        /*loaders: Vec<&ChunkLoader>*/
    ) {

        let c = ChunkCoord::from_block_coord(&BlockCoord::new(x, z));
        let cx = c.x;
        let cz = c.z;
        let r = radius as i32;

        for x in cx-r..cx+r {
            for z in cz-r..cz+r {
                let coord = ChunkCoord::new(x, z);

                if self.chunks.get(&coord).is_some() {continue;}
                self.generate_chunk(coord.x, coord.z, block_models, worker);
            }
        }
    }

    pub fn generate_chunk(&mut self, cx: i32, cz: i32, block_models: &Vec<BlockModel>, worker: &Worker) {

        // clears old chunk before generating new one.
        // it would be nice to have an option that keeps
        // the old chunk while generating until replacing it
        // with the new version
        self.chunks.insert(ChunkCoord::new(cx, cz), ChunkState::InWork(GenerationStep::Empty));

        self.chunks_generating += 1;
        let receiver = worker.execute(move || {
            let chunk_generator = TerrainGenerator::new(12345, Worldtype::DEFAULT);
            let mut chunk = Chunk::new(cx, cz);

            chunk_generator.generate_chunk_terrain(&mut chunk, 15); // POSSIBLE BUG FIX LATER

            chunk
        });

        self.generating_chunks.push(
            GeneratingChunk {
                coord: ChunkCoord::new(cx, cz),
                receiver,
            }
        );
    }

    pub fn load_chunk(&mut self, cx: i32, cz: i32) {
        // from disc
        todo!();
        self.chunks_loaded += 1;
    }

    pub fn free_chunk(&mut self, chunk: &Chunk) {
        todo!();
    }

    pub fn save_chunk(&mut self, chunk: &Chunk) {
        todo!();
    }

    pub fn destroy_chunk(&mut self, chunk: Chunk) {
        todo!();
        self.chunks_loaded -= 1;
    }

    /// Clears **all** `chunks` without saving them
    pub fn clear_chunks(&mut self) {
        todo!();
        self.chunks_loaded = 0;
        self.chunks_generating = 0;
    }
    /// Saves **all** `chunks` to disk and clears them from memory
    pub fn free_chunks(&mut self) {
        todo!();
        self.chunks_loaded = 0;
    }

    pub fn update_freeing_queue(&mut self) {
        for chunk in self.freed_chunks.pop() {
            // save chunk
            // destroy chunk
            todo!();
        }
    }

    pub fn update(&mut self, block_models: &Vec<BlockModel>, worker: &Worker) {
        // TODO: THE GENERATING CHUNKS VECTOR NEVER GETS CLEARED
        for i in 0..self.generating_chunks.len() {
            
            if i >= self.generating_chunks.len() {
                break;
            };

            match self.generating_chunks[i].receiver.try_recv() {
                Ok(mut chunk) => {
                    let coord = chunk.coord;
                    let gen_step = chunk.generation_step();
                    if gen_step == GenerationStep::Complete {
                        self.chunks_loaded += 1;
                        self.chunks_generating -= 1;
                        chunk.set_dirty(true);
                        self.dirty_chunks.push(coord);
                    }
                    self.chunks.insert(coord, Available(chunk));
                    
                }
                Err(TryRecvError::Empty) => { /* chunk still generating */ }
                Err(TryRecvError::Disconnected) => { /* inactive / panicked */ }
            }
        }
    }

    pub fn update_dirty_chunks(&mut self, block_models: &Vec<BlockModel>) {

        let mut dirt: Vec<ChunkCoord> = Vec::new();

        let mut i = 0;
        while let Some(coord) = self.dirty_chunks.pop() {
            i += 1;
            if coord.x >= 0 && coord.z >= 0 {
            }
            if let Some(region) = ChunkRegion::copy_chunk_region(self, &coord) {
                if let Some(chunk) = self.get_avl_chunk_mut(coord.x, coord.z) {
                    chunk.set_dirty(false);
                    let mesh = chunk_mesher::build_mesh(
                        chunk,
                        &region,
                        block_models,
                    );

                    chunk_renderer::upload_chunk_mesh(chunk, mesh);

                } else {dirt.push(coord);}
            } else {dirt.push(coord);}
        }

        for coord in dirt {
            self.dirty_chunks.push(coord);
        }
    }

    pub fn mark_chunk_dirty(&mut self, chunk: &Chunk) {
        let cx = chunk.coord.x;
        let cz = chunk.coord.z;
        self.dirty_chunks.push(ChunkCoord::new(cx, cz));
    }

    /// Takes chunk and replaces it with placeholder `ChunkState::InWork`
    pub fn take_chunk(&mut self, cx: i32, cz: i32) -> Option<ChunkState> {
        self.chunks.insert(ChunkCoord::new(cx, cz), ChunkState::InWork(GenerationStep::Empty))
    }

    pub fn get_chunk(&mut self, cx: i32, cz: i32) -> Option<ChunkState> {
        self.chunks.remove(&ChunkCoord::new(cx, cz))
    }

    pub fn get_chunk_ref(&mut self, cx: i32, cz: i32) -> Option<&ChunkState> {
        self.chunks.get(&ChunkCoord::new(cx, cz))
    }

    pub fn get_chunk_mut(&mut self, cx: i32, cz: i32) -> Option<&mut ChunkState> {
        self.chunks.get_mut(&ChunkCoord::new(cx, cz))
    }

    pub fn get_avl_chunk_ref(&self, cx: i32, cz: i32) -> Option<&Chunk> {
        if let Some(state) = self.chunks.get(&ChunkCoord::new(cx, cz)) {
            match state {
                Available(chunk) => {return Some(chunk);}
                InWork(_) => {return None;}
            }
        }
        None
    }

    pub fn get_avl_chunk_mut(&mut self, cx: i32, cz: i32) -> Option<&mut Chunk> {
        if let Some(state) = self.chunks.get_mut(&ChunkCoord::new(cx, cz)) {
            match state {
                Available(chunk) => {return Some(chunk);}
                InWork(_) => {return None;}
            }
        }
        None
    }

    pub fn insert_chunk(&mut self, chunk: Chunk, cx: i32, cz: i32) -> bool {
        self.chunks.insert(ChunkCoord::new(cx, cz), ChunkState::Available(chunk)).is_some()
    }

    pub fn clone_chunk(&self, cx: i32, cz: i32) -> Option<Chunk> {
        if let Some(state) = self.chunks.get(&ChunkCoord::new(cx, cz)) {
            match state {
                Available(chunk) => {
                    if matches!(chunk.generation_step(), GenerationStep::Complete) {
                        return Some(chunk.clone());
                    }
                    return None;
                }
                InWork(_) => {return None;}
            }
        }
        None
    }
}