use std::{collections::HashMap, sync::mpsc::TryRecvError};

use rand::random_range;

use crate::{engine::{blocks::{block::Block, block_model::{self, BlockModel}}, core::worker::Worker, rendering::{chunk_mesher, chunk_renderer::{self, upload_chunk_mesh}}, util::utility::inside_circle, world::{chunk::{CHUNK_WIDTH, Chunk}, chunk_manager::ChunkState::{Available, InWork}, chunk_region::ChunkRegion, chunk_state::{ChunkState, GenerationStep}, generation::{self, terrain_generator::{TerrainGenerator, Worldtype}}, lighting_system, voxel_coordinates::*, world::RENDER_DISTANCE}}, game::terrain_features::{basic_room::BasicRoom, oak_tree::OakTree, test_structure::TestStruct}};


/// Instanced once per loaded dimension
pub struct ChunkManager {
    pub chunks: HashMap<ChunkCoord, ChunkState>,
    generating_chunks: Vec<GeneratingChunk>, // receivers of generating chunks that are being edited by a thread
    generating_chunk_regions: Vec<GeneratingChunkRegion>,
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

pub struct GeneratingChunkRegion {
    pub coord: ChunkCoord,
    pub receiver: std::sync::mpsc::Receiver<(Chunk, ChunkRegion)>,
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
            generating_chunk_regions: Vec::new(),
            meshed_chunks: Vec::new(),
            dirty_chunks: Vec::new(),
            freed_chunks: Vec::new(),
            chunks_generating: 0,
            chunks_loaded: 0,
        }
    }

    pub fn load_chunks_in_radius(
        &mut self,
        bx: i32,
        bz: i32,
        radius: u32,
        worker: &Worker,
        block_models: &Vec<BlockModel>,
        /*loaders: Vec<&ChunkLoader>*/
    ) {
        let sx: i32 = bx.div_euclid(CHUNK_WIDTH);
        let sz: i32 = bz.div_euclid(CHUNK_WIDTH);
        let mut x = sx;
        let mut z = sz;
        let mut dir = 0;
        let mut step = 0;
        let mut step_len = 1;
        let mut turns_at = 0;
        let diam = radius * 2;

        for i in 0..diam*diam {
            
            if inside_circle(x-sx, z-sz, radius as i32) {
                let coord = ChunkCoord::new(x, z);
                
                if self.chunks.get(&coord).is_none() {
                    self.generate_chunk(x, z, block_models, worker);
                }
            }

            match dir {
                0 => x += 1, // right
                1 => z += 1, // up
                2 => x -= 1, // left
                3 => z -= 1, // down
                _ => {}
            }

            step += 1;

            if step >= step_len {
                step = 0;
                dir = (dir + 1) % 4;
                turns_at += 1;
                if turns_at >= 2 {
                    turns_at = 0;
                    step_len += 1;
                }
            }
        }

        let unloading_dist = RENDER_DISTANCE as i32 + 1;

        let mut c = Vec::new();
        for (key, val) in &self.chunks {
            match val {
                ChunkState::Available(chunk) => {
                    // unload chunk
                    if sx - chunk.coord.x > unloading_dist
                    || sz - chunk.coord.z > unloading_dist
                    || sx - chunk.coord.x < -unloading_dist
                    || sz - chunk.coord.z < -unloading_dist {
                        c.push(chunk.coord);
                    }
                }
                _ => {}
            }
        }
        for coord in c {
            if let Available(chunk) = self.chunks.remove(&coord).unwrap() {
                self.free_chunk(chunk);
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

            chunk_generator.generate_terrain(&mut chunk, 15); // POSSIBLE BUG FIX LATER

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

    pub fn free_chunk(&mut self, chunk: Chunk) {
        self.freed_chunks.push(chunk);
    }

    pub fn save_chunk(&mut self, chunk: &Chunk) {
        todo!();
    }

    pub fn destroy_chunk(&mut self, chunk: Chunk) {
        chunk_renderer::destroy_chunk_mesh(&chunk);
        if self.chunks_loaded >= 1 {self.chunks_loaded -= 1;}
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
        while let Some(chunk) = self.freed_chunks.pop() {
            // save chunk
            // destroy chunk
            self.destroy_chunk(chunk);
        }
    }

    pub fn update(&mut self, block_models: &Vec<BlockModel>, worker: &Worker) {

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

                    self.generating_chunks.remove(i);
                }
                Err(TryRecvError::Empty) => { /* chunk still generating */ }
                Err(TryRecvError::Disconnected) => { self.generating_chunks.remove(i); /* inactive / panicked */ }
            }
        }

        for i in 0..self.generating_chunk_regions.len() {
            
            if i >= self.generating_chunk_regions.len() {
                break;
            };

            match self.generating_chunk_regions[i].receiver.try_recv() {
                Ok(mut value) => {

                    let mut chunk = value.0;
                    let mut region = value.1;

                    let coord = chunk.coord;
                    let gen_step = chunk.generation_step();
                    
                    if gen_step == GenerationStep::Complete {
                        self.chunks_loaded += 1;
                        self.chunks_generating -= 1;
                        chunk.set_dirty(true);
                        self.dirty_chunks.push(coord);
                    }

                    self.chunks.insert(coord, Available(chunk));
                    ChunkRegion::insert_chunk_region(self, &coord, region);

                    self.generating_chunk_regions.remove(i);
                }
                Err(TryRecvError::Empty) => { /* chunk still generating */ }
                Err(TryRecvError::Disconnected) => { self.generating_chunk_regions.remove(i); /* inactive / panicked */ }
            }
        }

        for i in 0..self.meshed_chunks.len() {
            
            if i >= self.meshed_chunks.len() {
                break;
            };

            match self.meshed_chunks[i].receiver.try_recv() {
                Ok(mut mesh) => {

                    let coord = self.meshed_chunks[i].coord;

                    if let Some(chunk) = self.get_avl_chunk_mut(coord.x, coord.z) {
                        upload_chunk_mesh(chunk, mesh);
                    }

                    self.meshed_chunks.remove(i);
                }
                Err(TryRecvError::Empty) => { /* chunk still generating */ }
                Err(TryRecvError::Disconnected) => {self.meshed_chunks.remove(i); /* inactive / panicked */ }
            }
        }


        let mut feature_ready_chunks: Vec<ChunkCoord> = Vec::new();
        let mut direct_light_ready_chunks: Vec<ChunkCoord> = Vec::new();
        let mut indirect_light_ready_chunks: Vec<ChunkCoord> = Vec::new();

        for (key, val) in &self.chunks {
            match val {

                ChunkState::Available(chunk) => {

                    match chunk.generation_step() {

                        GenerationStep::Terrain => {feature_ready_chunks.push(chunk.coord);}
                        
                        GenerationStep::DirectLight => {direct_light_ready_chunks.push(chunk.coord);}

                        GenerationStep::IndirectLight => {indirect_light_ready_chunks.push(chunk.coord);}

                        _ => {}
                    }
                }

                ChunkState::InWork(step) => {}
            }
        }

        for coord in feature_ready_chunks {
                        
            let region = ChunkRegion::take_chunk_region(self, &coord);
            if region.is_none() {
                continue;
            }
            
            // region already checks if center chunk is available
            let mut region = region.unwrap();
            let mut chunk = self.take_avl_chunk(coord.x, coord.z).unwrap();

            let receiver = worker.execute(move || {

                for i in 1..=random_range(0..10) { // 1..20, 10..40
                    TerrainGenerator::generate_structure(&OakTree {}, &mut chunk, &mut region);
                }

                /*for i in 1..=random_range(3..5) {
                    TerrainGenerator::generate_structure(&BasicRoom {}, &mut chunk, &mut region);
                }
                
                for i in 1..=random_range(3..5) {
                    TerrainGenerator::generate_structure(&TestStruct {}, &mut chunk, &mut region);
                }*/

                chunk.set_generation_step(GenerationStep::DirectLight);

                (chunk, region)
            });

            self.generating_chunk_regions.push(
                GeneratingChunkRegion {
                    coord: coord.clone(),
                    receiver,
                }
            );
        }

        for coord in direct_light_ready_chunks {
                        
            /*let region = ChunkRegion::take_chunk_region(self, &coord);
            if region.is_none() {
                continue;
            }*/
            
            // region already checks if center chunk is available
            //let mut region = region.unwrap();
            if !self.chunk_avail(coord.x, coord.z) {return;}
            let mut chunk = self.take_avl_chunk(coord.x, coord.z).unwrap();

            let receiver = worker.execute(move || {

                lighting_system::bake_direct_light(&mut chunk);

                chunk.set_generation_step(GenerationStep::IndirectLight);

                chunk
            });

            self.generating_chunks.push(
                GeneratingChunk {
                    coord: coord.clone(),
                    receiver,
                }
            );
        }

        for coord in indirect_light_ready_chunks {
                        
            let region = ChunkRegion::take_chunk_region(self, &coord);
            if region.is_none() {
                continue;
            }
            
            // region already checks if center chunk is available
            let mut region = region.unwrap();
            let mut chunk = self.take_avl_chunk(coord.x, coord.z).unwrap();

            let receiver = worker.execute(move || {

                lighting_system::bake_indirect_light(&mut chunk, &mut region);

                chunk.set_generation_step(GenerationStep::Complete);

                (chunk, region)
            });

            self.generating_chunk_regions.push(
                GeneratingChunkRegion {
                    coord: coord.clone(),
                    receiver,
                }
            );
        }

        self.update_freeing_queue();
    }

    pub fn update_dirty_chunks<'a>(&mut self, block_models: &Vec<BlockModel>, worker: &Worker) {

        let mut dirt: Vec<ChunkCoord> = Vec::new();

        let mut i = 0;
        while let Some(coord) = self.dirty_chunks.pop() {
            i += 1;
            if coord.x >= 0 && coord.z >= 0 {
            }
            if let Some(region) = ChunkRegion::copy_chunk_region(self, &coord) {
                if let Some(chunk) = self.get_avl_chunk_mut(coord.x, coord.z) {
                    chunk.set_dirty(false);

                    
                    /*let mesh = chunk_mesher::build_mesh(
                        chunk,
                        &region,
                        block_models,
                    );*/
                    //chunk_renderer::upload_chunk_mesh(chunk, mesh);
                        

                        // chunk mesh should store creation time to skip mesh
                        // if newer version is already used
                        //self.meshing_chunks += 1;
                    let coord = ChunkCoord::new(chunk.coord.x, chunk.coord.z);
                    let chunk_clone = chunk.clone();
                    let block_models_clone = block_models.clone();
                    
                    let receiver = worker.execute(move || {

                        let mesh = chunk_mesher::build_mesh(
                            &chunk_clone,
                            &region,
                            &block_models_clone, //TODO: replace clone with arc later
                        );

                        mesh
                    });

                    self.meshed_chunks.push(
                        MeshedChunk {
                            coord,
                            receiver,
                        }
                    );

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

    pub fn take_avl_chunk(&mut self, cx: i32, cz: i32) -> Option<Chunk> {
        if !self.chunk_avail(cx, cz) {return None;}
        if let Some(state) = self.chunks.insert(ChunkCoord::new(cx, cz), ChunkState::InWork(GenerationStep::Empty)) {
            match state {
                Available(chunk) => {return Some(chunk);}
                InWork(_) => {return None;}
            }
        }
        None
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

    /// Checks if the chunk is complete and available
    pub fn chunk_avail(&self, cx: i32, cz: i32) -> bool {
        if let Some(state) = self.chunks.get(&ChunkCoord::new(cx, cz)) {
            match state {
                Available(chunk) => {
                    /*if matches!(chunk.generation_step(), GenerationStep::Complete) {
                        return true;
                    }*/
                    return true;
                }
                InWork(_) => {return false;}
            }
        }
        false
    }
    // If Generation Step complete
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

    pub fn complete_chunk_avail(&self, cx: i32, cz: i32) -> bool {
        if let Some(state) = self.chunks.get(&ChunkCoord::new(cx, cz)) {
            match state {
                Available(chunk) => {
                    if matches!(chunk.generation_step(), GenerationStep::Complete) {
                        return true;
                    }
                    return false;
                }
                InWork(_) => {return false;}
            }
        }
        false
    }
}