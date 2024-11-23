import { create } from 'zustand'
import pako from 'pako'

const useGameStore = create((set) => ({
  grid: null,
  players: null,
  worldMap: null,
  gridWidth: 800,
  gridHeight: 600,
  ws: null,

  setGrid: (grid) => set({ grid }),
  setPlayers: (players) => set({ players }),
  setWorldMap: (worldMap) => set({ worldMap }),

  connectWebSocket: () => {
    const ws = new WebSocket(`ws://${window.location.host}/ws`)
    
    ws.onmessage = (event) => {
      const data = JSON.parse(event.data)
      
      if (data.type === 'game_state') {
        const state = data.data;
        
        if (state.grid) {
          // Convert hex string to bytes
          const hexString = state.grid.grid;
          const bytes = new Uint8Array(hexString.match(/.{1,2}/g).map(byte => parseInt(byte, 16)));
          
          // Decompress using zlib
          const decompressed = pako.inflate(bytes);
          
          // Convert to 2D grid
          const grid = Array(state.grid.height).fill().map(() => 
            Array(state.grid.width).fill(null)
          );
          
          let idx = 0;
          for (let y = 0; y < state.grid.height; y++) {
            for (let x = 0; x < state.grid.width; x++) {
              const value = decompressed[idx++];
              grid[y][x] = value === 255 ? null : value;
            }
          }
          
          set({ grid });
        }
        
        if (state.players) {
          set({ players: state.players });
        }

        if (state.world_map) {
          set({ worldMap: state.world_map });
        }
      } else if (data.type === 'player_info') {
        set({ players: data.data });
      }
    }
    
    ws.onclose = () => {
      console.log('WebSocket disconnected')
      // Try to reconnect after a delay
      setTimeout(() => {
        useGameStore.getState().connectWebSocket()
      }, 1000)
    }
    
    set({ ws })
  },

  disconnectWebSocket: () => {
    const { ws } = useGameStore.getState()
    if (ws) {
      ws.close()
      set({ ws: null })
    }
  }
}))

export default useGameStore
