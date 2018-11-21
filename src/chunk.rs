pub struct Chunk {
    CHUNK_X: f64,
    CHUNK_Y: f64,
    CHUNK_SIZE_X: i32,
    CHUNK_SIZE_Y: i32,
    CHUNK_AMOUNT_X: i32,
    CHUNK_AMOUNT_Y: i32,
}

#[allow(dead_code)]
impl Chunk {
    pub fn new() -> Chunk {
        Chunk {CHUNK_X, CHUNK_Y, CHUNK_SIZE_X : 16, CHUNK_SIZE_Y : 16, CHUNK_AMOUNT_X : 5, CHUNK_AMOUNT_Y : 5}
    }
    pub fn checkChunks(&mut self) {
        let player_chunk_x = self.player.x/self.CHUNK_SIZE_X;
        let player_chunk_y = self.player.y/self.CHUNK_SIZE_Y;

        // Unload chunks


        // Load chunks
    }
    pub fn unloadChunks(&mut self) {
        for lawn in &self.terrain {
            if self.CHUNK_X > player_chunk_x + (self.CHUNK_AMOUNT_X-1)/2 || self.CHUNK_X < player.chunk_x - (self.CHUNK_AMOUNT_X-1)/2 || self.CHUNK_Y > player_chunk_y + (self.CHUNK_AMOUNT_Y-1)/2 || self.CHUNK_Y < player_chunk_y - (self.CHUNK_AMOUNT_Y-1)/2 {
                println!("UNLOAD CHUNK", )
            }
        }
    }
    pub fn loadChunks(&mut self) {
        for lawn in &self.terrain {
            
        }
    }
}