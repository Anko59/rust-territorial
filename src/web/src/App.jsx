import { useEffect } from 'react'
import GameRenderer from './components/GameRenderer'
import useGameStore from './store'

function App() {
  const connectWebSocket = useGameStore(state => state.connectWebSocket)

  useEffect(() => {
    connectWebSocket()
  }, [connectWebSocket])

  return (
    <div style={{ width: '100%', height: '100%' }}>
      <GameRenderer />
    </div>
  )
}

export default App
