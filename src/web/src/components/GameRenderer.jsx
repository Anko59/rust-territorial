import { useEffect, useRef } from 'react'
import * as THREE from 'three'
import useGameStore from '../store'

const GameRenderer = () => {
  const containerRef = useRef()
  const rendererRef = useRef()
  const sceneRef = useRef()
  const cameraRef = useRef()
  const terrainMeshRef = useRef()
  const gridMeshRef = useRef()
  const textLabelsRef = useRef([])
  
  const { grid, players, gridWidth, gridHeight, worldMap } = useGameStore()

  useEffect(() => {
    // Initialize Three.js scene
    const container = containerRef.current
    const width = container.clientWidth
    const height = container.clientHeight

    // Create scene
    const scene = new THREE.Scene()
    scene.width = width
    scene.height = height
    sceneRef.current = scene

    // Create camera
    const camera = new THREE.OrthographicCamera(
      width / -2, width / 2,
      height / 2, height / -2,
      0.1, 1000
    )
    camera.position.z = 100
    cameraRef.current = camera

    // Create renderer
    const renderer = new THREE.WebGLRenderer({ antialias: true })
    renderer.setSize(width, height)
    renderer.setClearColor(0x000000)
    container.appendChild(renderer.domElement)
    rendererRef.current = renderer

    // Create terrain geometry (background)
    const terrainGeometry = new THREE.PlaneGeometry(width, height, gridWidth - 1, gridHeight - 1)
    const terrainMaterial = new THREE.MeshBasicMaterial({
      vertexColors: true,
      side: THREE.DoubleSide
    })
    const terrainMesh = new THREE.Mesh(terrainGeometry, terrainMaterial)
    terrainMesh.position.z = -1 // Place behind grid
    scene.add(terrainMesh)
    terrainMeshRef.current = terrainMesh

    // Create grid geometry (player territories)
    const gridGeometry = new THREE.PlaneGeometry(width, height, gridWidth - 1, gridHeight - 1)
    const gridMaterial = new THREE.MeshBasicMaterial({
      vertexColors: true,
      side: THREE.DoubleSide,
      transparent: true,
      opacity: 0.7
    })
    const gridMesh = new THREE.Mesh(gridGeometry, gridMaterial)
    scene.add(gridMesh)
    gridMeshRef.current = gridMesh

    // Handle resize
    const handleResize = () => {
      const newWidth = container.clientWidth
      const newHeight = container.clientHeight
      
      camera.left = newWidth / -2
      camera.right = newWidth / 2
      camera.top = newHeight / 2
      camera.bottom = newHeight / -2
      camera.updateProjectionMatrix()
      
      renderer.setSize(newWidth, newHeight)
      scene.width = newWidth
      scene.height = newHeight
    }
    window.addEventListener('resize', handleResize)

    // Animation loop
    const animate = () => {
      requestAnimationFrame(animate)
      renderer.render(scene, camera)
    }
    animate()

    return () => {
      window.removeEventListener('resize', handleResize)
      container.removeChild(renderer.domElement)
      renderer.dispose()
    }
  }, [])

  // Update terrain colors when world map changes
  useEffect(() => {
    if (!worldMap?.color_map || !terrainMeshRef.current) return

    const colors = []
    for (let y = 0; y < gridHeight; y++) {
      for (let x = 0; x < gridWidth; x++) {
        const color = worldMap.color_map[y][x]
        colors.push(color[0] / 255, color[1] / 255, color[2] / 255)
      }
    }

    terrainMeshRef.current.geometry.setAttribute(
      'color',
      new THREE.Float32BufferAttribute(colors, 3)
    )
  }, [worldMap, gridWidth, gridHeight])

  // Update grid colors when grid data changes
  useEffect(() => {
    if (!grid || !gridMeshRef.current || !sceneRef.current) return

    const colors = []
    const playerColors = new Map()
    
    // Generate colors for players
    if (players) {
      players.forEach((player, id) => {
        playerColors.set(id, new THREE.Color(
          Math.sin(id * 0.5) * 0.5 + 0.5,
          Math.sin(id * 0.7) * 0.5 + 0.5,
          Math.sin(id * 0.9) * 0.5 + 0.5
        ))
      })
    }

    // Set colors for each vertex
    for (let y = 0; y < gridHeight; y++) {
      for (let x = 0; x < gridWidth; x++) {
        const playerId = grid[y] && grid[y][x]
        const color = (playerId !== null && playerId !== undefined && playerColors.has(playerId)) ? 
          playerColors.get(playerId) : 
          new THREE.Color(0, 0, 0)
        colors.push(color.r, color.g, color.b)
      }
    }

    gridMeshRef.current.geometry.setAttribute(
      'color',
      new THREE.Float32BufferAttribute(colors, 3)
    )
    
    // Update player labels
    const scene = sceneRef.current
    if (!scene || !scene.width || !scene.height) return
    
    // Remove old labels
    textLabelsRef.current.forEach(label => scene.remove(label))
    textLabelsRef.current = []
    
    // Add new labels
    if (players) {
      players.forEach(player => {
        if (!player) return
        
        const canvas = document.createElement('canvas')
        const ctx = canvas.getContext('2d')
        const fontSize = Math.min(30 * Math.sqrt((player.resources || 0) / 1000), 48)
        
        ctx.font = `${fontSize}px Arial`
        const text = `${player.name || 'Player'} (${player.resources || 0})`
        const metrics = ctx.measureText(text)
        
        canvas.width = metrics.width
        canvas.height = fontSize * 1.5
        
        ctx.font = `${fontSize}px Arial`
        ctx.fillStyle = 'white'
        ctx.fillText(text, 0, fontSize)
        
        const texture = new THREE.CanvasTexture(canvas)
        const material = new THREE.SpriteMaterial({ map: texture })
        const sprite = new THREE.Sprite(material)
        
        const x = ((player.center_x || 0) / gridWidth) * scene.width - scene.width / 2
        const y = -((player.center_y || 0) / gridHeight) * scene.height + scene.height / 2
        
        sprite.position.set(x, y, 1)
        sprite.scale.set(canvas.width, canvas.height, 1)
        
        scene.add(sprite)
        textLabelsRef.current.push(sprite)
      })
    }
    
  }, [grid, players, gridWidth, gridHeight])

  return <div ref={containerRef} style={{ width: '100%', height: '100%' }} />
}

export default GameRenderer
