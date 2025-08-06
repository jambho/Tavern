import React, { useState, useRef, useCallback, useEffect } from 'react';
import { Move, Grid3X3, Users } from 'lucide-react';
import { useTheme } from '../../contexts/ThemeContext';
import TokenDisplay from './TokenDisplay';

// Types and Interfaces
interface Token {
  id: number;
  x: number;
  y: number;
  color: string;
  name: string;
  size: number;
  snapToGrid: boolean;
}

interface Position {
  x: number;
  y: number;
}

interface BoardSize {
  width: number;
  height: number;
}

interface DragState {
  tokenId: number;
  offsetX: number;
  offsetY: number;
}

interface ContextMenuHandler {
  (e: React.MouseEvent<HTMLCanvasElement>, token: Token): void;
}

interface GameBoardProps {
  onContextMenu?: ContextMenuHandler;
}

const GameBoard: React.FC = () => {
  const { theme, themeMode, toggleTheme } = useTheme();
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const [gridSize, setGridSize] = useState<number>(40);
  const [showGrid, setShowGrid] = useState<boolean>(true);
  const [tokens, setTokens] = useState<Token[]>([
    { id: 1, x: 120, y: 120, color: '#ff6b6b', name: 'Player 1', size: 30, snapToGrid: true },
    { id: 2, x: 200, y: 160, color: '#4ecdc4', name: 'Player 2', size: 30, snapToGrid: true },
    { id: 3, x: 280, y: 200, color: '#45b7d1', name: 'NPC 1', size: 25, snapToGrid: false },
  ]);
  const [dragState, setDragState] = useState<DragState | null>(null);
  const [isDragging, setIsDragging] = useState<boolean>(false);
  const [hoveredToken, setHoveredToken] = useState<number | null>(null);
  const [boardSize, setBoardSize] = useState<BoardSize>({ width: 800, height: 600 });
  const [canvasScale, setCanvasScale] = useState<number>(1);
  const [canvasOffset, setCanvasOffset] = useState<Position>({ x: 0, y: 0 });

  // Use refs to avoid callback dependency issues
  const dragStateRef = useRef<DragState | null>(null);
  const tokensRef = useRef<Token[]>(tokens);
  const boardSizeRef = useRef<BoardSize>(boardSize);
  const gridSizeRef = useRef<number>(gridSize);
  const animationFrameRef = useRef<number | null>(null);
  const isDraggingRef = useRef<boolean>(false);
  const canvasScaleRef = useRef<number>(canvasScale);
  const canvasOffsetRef = useRef<Position>(canvasOffset);

  // Update refs when state changes
  useEffect(() => {
    dragStateRef.current = dragState;
    tokensRef.current = tokens;
    boardSizeRef.current = boardSize;
    gridSizeRef.current = gridSize;
    isDraggingRef.current = isDragging;
    canvasScaleRef.current = canvasScale;
    canvasOffsetRef.current = canvasOffset;
  }, [dragState, tokens, boardSize, gridSize, isDragging, canvasScale, canvasOffset]);

  // Calculate canvas scale and offset to center it
  const updateCanvasLayout = useCallback(() => {
    if (!canvasRef.current) return;
    
    const container = canvasRef.current.parentElement;
    if (!container) return;
    
    const containerRect = container.getBoundingClientRect();
    const containerWidth = containerRect.width - 32; // Account for padding
    const containerHeight = containerRect.height - 32;
    
    // Calculate scale to fit the canvas within the container
    const scaleX = containerWidth / boardSize.width;
    const scaleY = containerHeight / boardSize.height;
    const scale = Math.min(scaleX, scaleY, 1); // Don't scale up beyond 100%
    
    // Calculate offset to center the canvas
    const scaledWidth = boardSize.width * scale;
    const scaledHeight = boardSize.height * scale;
    const offsetX = (containerWidth - scaledWidth) / 2;
    const offsetY = (containerHeight - scaledHeight) / 2;
    
    setCanvasScale(scale);
    setCanvasOffset({ x: offsetX, y: offsetY });
  }, [boardSize]);

  // Convert screen coordinates to canvas coordinates
  const screenToCanvas = useCallback((screenX: number, screenY: number): Position => {
    return {
      x: (screenX - canvasOffsetRef.current.x) / canvasScaleRef.current,
      y: (screenY - canvasOffsetRef.current.y) / canvasScaleRef.current
    };
  }, []);

  // Convert canvas coordinates to screen coordinates
  const canvasToScreen = useCallback((canvasX: number, canvasY: number): Position => {
    return {
      x: canvasX * canvasScaleRef.current + canvasOffsetRef.current.x,
      y: canvasY * canvasScaleRef.current + canvasOffsetRef.current.y
    };
  }, []);

  // Snap position to grid
  const snapToGridPosition = useCallback((x: number, y: number, shouldSnap: boolean): Position => {
    if (!shouldSnap) return { x, y };
    return {
      x: Math.floor(x / gridSizeRef.current) * gridSizeRef.current + gridSizeRef.current / 2,
      y: Math.floor(y / gridSizeRef.current) * gridSizeRef.current + gridSizeRef.current / 2
    };
  }, []);

  // Reposition all tokens to grid
  const repositionTokensToGrid = useCallback(() => {
    setTokens(prev => prev.map(token => {
      if (token.snapToGrid) {
        const snapped = snapToGridPosition(token.x, token.y, true);
        return { ...token, x: snapped.x, y: snapped.y };
      }
      return token;
    }));
  }, [snapToGridPosition]);

  // Check if mouse is over a token
  const getTokenAtPosition = useCallback((x: number, y: number): Token | undefined => {
    // Check tokens in reverse order (top to bottom) for better UX
    for (let i = tokensRef.current.length - 1; i >= 0; i--) {
      const token = tokensRef.current[i];
      const distance = Math.sqrt(Math.pow(x - token.x, 2) + Math.pow(y - token.y, 2));
      if (distance <= token.size) {
        return token;
      }
    }
    return undefined;
  }, []);

  // Clean up drag state and event listeners
  const cleanupDrag = useCallback(() => {
    if (animationFrameRef.current) {
      cancelAnimationFrame(animationFrameRef.current);
      animationFrameRef.current = null;
    }
    setDragState(null);
    setIsDragging(false);
    isDraggingRef.current = false;
  }, []);

  // Handle mouse move for dragging using requestAnimationFrame
  const handleMouseMove = useCallback((e: MouseEvent): void => {
    if (!isDraggingRef.current || !dragStateRef.current || !canvasRef.current) {
      return;
    }

    // Cancel any pending animation frame
    if (animationFrameRef.current) {
      cancelAnimationFrame(animationFrameRef.current);
    }

    // Schedule the update for the next frame
    animationFrameRef.current = requestAnimationFrame(() => {
      try {
        if (!dragStateRef.current || !canvasRef.current) {
          cleanupDrag();
          return;
        }

        const rect = canvasRef.current.getBoundingClientRect();
        
        // Convert screen coordinates to canvas coordinates
        const canvasCoords = screenToCanvas(e.clientX - rect.left, e.clientY - rect.top);
        
        // Check if mouse is still within the canvas bounds
        if (canvasCoords.x < 0 || canvasCoords.x > boardSizeRef.current.width || 
            canvasCoords.y < 0 || canvasCoords.y > boardSizeRef.current.height) {
          // Mouse is outside canvas, but continue dragging with boundary constraints
          const constrainedX = Math.max(0, Math.min(boardSizeRef.current.width, canvasCoords.x));
          const constrainedY = Math.max(0, Math.min(boardSizeRef.current.height, canvasCoords.y));
          
          // Get the token being dragged
          const token = tokensRef.current.find(t => t.id === dragStateRef.current!.tokenId);
          if (!token) {
            cleanupDrag();
            return;
          }
          
          // Apply snapping if enabled for this token
          const snapped = snapToGridPosition(constrainedX, constrainedY, token.snapToGrid);
          
          // Ensure snapped coordinates are valid
          if (isNaN(snapped.x) || isNaN(snapped.y)) {
            return;
          }
          
          setTokens(prev => prev.map(t => 
            t.id === dragStateRef.current!.tokenId 
              ? { ...t, x: snapped.x, y: snapped.y }
              : t
          ));
          return;
        }

        const newX = Math.max(0, Math.min(boardSizeRef.current.width, canvasCoords.x - dragStateRef.current.offsetX));
        const newY = Math.max(0, Math.min(boardSizeRef.current.height, canvasCoords.y - dragStateRef.current.offsetY));

        // Get the token being dragged
        const token = tokensRef.current.find(t => t.id === dragStateRef.current!.tokenId);
        if (!token) {
          cleanupDrag();
          return;
        }

        // Apply snapping if enabled for this token
        const snapped = snapToGridPosition(newX, newY, token.snapToGrid);

        // Ensure snapped coordinates are valid
        if (isNaN(snapped.x) || isNaN(snapped.y)) {
          return;
        }

        setTokens(prev => prev.map(t => 
          t.id === dragStateRef.current!.tokenId 
            ? { ...t, x: snapped.x, y: snapped.y }
            : t
        ));
      } catch (error) {
        console.error('Error in handleMouseMove:', error);
        cleanupDrag();
      }
    });
  }, [snapToGridPosition, cleanupDrag, screenToCanvas]);

  // Handle mouse up to end dragging
  const handleMouseUp = useCallback((): void => {
    cleanupDrag();
    
    // Remove event listeners
    document.removeEventListener('mousemove', handleMouseMove);
    document.removeEventListener('mouseup', handleMouseUp);
  }, [handleMouseMove, cleanupDrag]);

  // Handle mouse down on token
  const handleTokenMouseDown = useCallback((e: React.MouseEvent<HTMLCanvasElement>, token: Token): void => {
    try {
      e.preventDefault();
      e.stopPropagation();
      
      if (!canvasRef.current) return;
      
      const rect = canvasRef.current.getBoundingClientRect();
      const canvasCoords = screenToCanvas(e.clientX - rect.left, e.clientY - rect.top);
      
      // Ensure we have valid coordinates
      if (isNaN(canvasCoords.x) || isNaN(canvasCoords.y)) return;
      
      const newDragState = {
        tokenId: token.id,
        offsetX: canvasCoords.x - token.x,
        offsetY: canvasCoords.y - token.y
      };
      
      // Ensure offset values are valid
      if (isNaN(newDragState.offsetX) || isNaN(newDragState.offsetY)) return;
      
      setDragState(newDragState);
      setIsDragging(true);
      isDraggingRef.current = true;
      
      // Add event listeners with capture to ensure we catch all events
      document.addEventListener('mousemove', handleMouseMove, { passive: false, capture: true });
      document.addEventListener('mouseup', handleMouseUp, { passive: false, capture: true });
    } catch (error) {
      console.error('Error in handleTokenMouseDown:', error);
    }
  }, [handleMouseMove, handleMouseUp]);

  // Global mouse event handler to catch edge cases
  useEffect(() => {
    const handleGlobalMouseUp = (e: MouseEvent) => {
      if (isDraggingRef.current) {
        cleanupDrag();
        document.removeEventListener('mousemove', handleMouseMove, { capture: true });
        document.removeEventListener('mouseup', handleMouseUp, { capture: true });
      }
    };

    const handleGlobalMouseLeave = (e: MouseEvent) => {
      if (isDraggingRef.current) {
        cleanupDrag();
        document.removeEventListener('mousemove', handleMouseMove, { capture: true });
        document.removeEventListener('mouseup', handleMouseUp, { capture: true });
      }
    };

    const handleGlobalMouseOut = (e: MouseEvent) => {
      if (isDraggingRef.current && e.relatedTarget === null) {
        // Mouse left the window entirely
        cleanupDrag();
        document.removeEventListener('mousemove', handleMouseMove, { capture: true });
        document.removeEventListener('mouseup', handleMouseUp, { capture: true });
      }
    };

    document.addEventListener('mouseup', handleGlobalMouseUp);
    document.addEventListener('mouseleave', handleGlobalMouseLeave);
    document.addEventListener('mouseout', handleGlobalMouseOut);
    
    return () => {
      document.removeEventListener('mouseup', handleGlobalMouseUp);
      document.removeEventListener('mouseleave', handleGlobalMouseLeave);
      document.removeEventListener('mouseout', handleGlobalMouseOut);
      if (animationFrameRef.current) {
        cancelAnimationFrame(animationFrameRef.current);
      }
    };
  }, [handleMouseMove, handleMouseUp, cleanupDrag]);

  // Handle right-click context menu
  const handleTokenRightClick = useCallback((e: React.MouseEvent<HTMLCanvasElement>, token: Token): void => {
    e.preventDefault();
    e.stopPropagation();
    
    // Remove any existing context menus
    const existingMenus = document.querySelectorAll('.token-context-menu');
    existingMenus.forEach(menu => menu.remove());
    
    // Create context menu
    const contextMenu = document.createElement('div');
    contextMenu.className = 'token-context-menu absolute rounded-md py-1 z-50 min-w-48';
    
    // Set position to absolute and use page coordinates
    contextMenu.style.position = 'absolute';
    contextMenu.style.left = `${e.pageX}px`;
    contextMenu.style.top = `${e.pageY}px`;
    contextMenu.style.zIndex = '9999';
    contextMenu.style.backgroundColor = theme.colors.surface.primary;
    contextMenu.style.border = `1px solid ${theme.colors.surface.border}`;
    contextMenu.style.borderRadius = theme.borderRadius.md;
    contextMenu.style.boxShadow = theme.shadows.lg;
    
    // Toggle grid snap option
    const snapToGridItem = document.createElement('div');
    snapToGridItem.className = 'px-4 py-2 text-sm cursor-pointer';
    snapToGridItem.textContent = token.snapToGrid ? 'Disable Grid Snap' : 'Enable Grid Snap';
    snapToGridItem.style.color = theme.colors.text.primary;
    snapToGridItem.style.padding = theme.spacing.sm + ' ' + theme.spacing.md;
    snapToGridItem.style.fontSize = '14px';
    snapToGridItem.style.cursor = 'pointer';
    snapToGridItem.style.transition = 'background-color 0.15s ease';
    snapToGridItem.onmouseenter = () => {
      snapToGridItem.style.backgroundColor = theme.colors.neutral[100];
    };
    snapToGridItem.onmouseleave = () => {
      snapToGridItem.style.backgroundColor = 'transparent';
    };
    snapToGridItem.onclick = () => {
      setTokens(prev => prev.map(t => 
        t.id === token.id 
          ? { ...t, snapToGrid: !t.snapToGrid }
          : t
      ));
      
      // Reposition the token to grid if snapping was enabled
      setTimeout(() => {
        setTokens(prev => prev.map(t => {
          if (t.id === token.id && t.snapToGrid) {
            const snapped = snapToGridPosition(t.x, t.y, true);
            return { ...t, x: snapped.x, y: snapped.y };
          }
          return t;
        }));
      }, 0);
      
      contextMenu.remove();
    };
    
    // Rename option
    const renameItem = document.createElement('div');
    renameItem.className = 'px-4 py-2 text-sm cursor-pointer';
    renameItem.textContent = 'Rename Token';
    renameItem.style.color = theme.colors.text.primary;
    renameItem.style.padding = theme.spacing.sm + ' ' + theme.spacing.md;
    renameItem.style.fontSize = '14px';
    renameItem.style.cursor = 'pointer';
    renameItem.style.transition = 'background-color 0.15s ease';
    renameItem.onmouseenter = () => {
      renameItem.style.backgroundColor = theme.colors.neutral[100];
    };
    renameItem.onmouseleave = () => {
      renameItem.style.backgroundColor = 'transparent';
    };
    renameItem.onclick = () => {
      const newName = prompt('Enter new name:', token.name);
      if (newName && newName.trim()) {
        setTokens(prev => prev.map(t => 
          t.id === token.id ? { ...t, name: newName.trim() } : t
        ));
      }
      contextMenu.remove();
    };
    
    // Change color option
    const colorItem = document.createElement('div');
    colorItem.className = 'px-4 py-2 text-sm cursor-pointer';
    colorItem.textContent = 'Change Color';
    colorItem.style.color = theme.colors.text.primary;
    colorItem.style.padding = theme.spacing.sm + ' ' + theme.spacing.md;
    colorItem.style.fontSize = '14px';
    colorItem.style.cursor = 'pointer';
    colorItem.style.transition = 'background-color 0.15s ease';
    colorItem.onmouseenter = () => {
      colorItem.style.backgroundColor = theme.colors.neutral[100];
    };
    colorItem.onmouseleave = () => {
      colorItem.style.backgroundColor = 'transparent';
    };
    colorItem.onclick = () => {
      const colors = ['#ff6b6b', '#4ecdc4', '#45b7d1', '#96ceb4', '#feca57', '#ff9ff3', '#54a0ff', '#5f27cd'];
      const randomColor = colors[Math.floor(Math.random() * colors.length)];
      
      setTokens(prev => prev.map(t => 
        t.id === token.id ? { ...t, color: randomColor } : t
      ));
      contextMenu.remove();
    };
    
    // Separator
    const separator = document.createElement('div');
    separator.className = 'border-t my-1';
    separator.style.borderTop = `1px solid ${theme.colors.surface.border}`;
    separator.style.margin = theme.spacing.xs + ' 0';
    
    // Delete option
    const deleteItem = document.createElement('div');
    deleteItem.className = 'px-4 py-2 text-sm cursor-pointer';
    deleteItem.textContent = 'Delete Token';
    deleteItem.style.color = theme.colors.error[600];
    deleteItem.style.padding = theme.spacing.sm + ' ' + theme.spacing.md;
    deleteItem.style.fontSize = '14px';
    deleteItem.style.cursor = 'pointer';
    deleteItem.style.transition = 'background-color 0.15s ease';
    deleteItem.onmouseenter = () => {
      deleteItem.style.backgroundColor = theme.colors.error[50];
    };
    deleteItem.onmouseleave = () => {
      deleteItem.style.backgroundColor = 'transparent';
    };
    deleteItem.onclick = () => {
      setTokens(prev => prev.filter(t => t.id !== token.id));
      contextMenu.remove();
    };
    
    // Add items to menu
    contextMenu.appendChild(snapToGridItem);
    contextMenu.appendChild(renameItem);
    contextMenu.appendChild(colorItem);
    contextMenu.appendChild(separator);
    contextMenu.appendChild(deleteItem);
    
    // Add menu to body
    document.body.appendChild(contextMenu);
    
    // Remove menu when clicking outside
    const removeMenu = () => {
      if (document.body.contains(contextMenu)) {
        contextMenu.remove();
      }
      document.removeEventListener('click', removeMenu);
      document.removeEventListener('contextmenu', removeMenu);
    };
    
    // Add event listeners after a short delay
    setTimeout(() => {
      document.addEventListener('click', removeMenu);
      document.addEventListener('contextmenu', removeMenu);
    }, 100);
  }, [tokens, theme]);

  // Handle mouse move for hover detection
  const handleCanvasMouseMove = useCallback((e: React.MouseEvent<HTMLCanvasElement>): void => {
    if (isDragging) return;
    
    if (!canvasRef.current) return;
    
    const rect = canvasRef.current.getBoundingClientRect();
    const canvasCoords = screenToCanvas(e.clientX - rect.left, e.clientY - rect.top);
    const token = getTokenAtPosition(canvasCoords.x, canvasCoords.y);
    
    setHoveredToken(token ? token.id : null);
  }, [isDragging, getTokenAtPosition, screenToCanvas]);

  // Draw the board
  const drawBoard = useCallback((): void => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    
    const ctx = canvas.getContext('2d');
    if (!ctx) return;
    
    // Clear the canvas
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    
    // Apply scaling and offset transformations
    ctx.save();
    ctx.translate(canvasOffset.x, canvasOffset.y);
    ctx.scale(canvasScale, canvasScale);
    
    // Clear the transformed area
    ctx.clearRect(0, 0, boardSize.width, boardSize.height);
    
    // Draw grid
    if (showGrid) {
      ctx.strokeStyle = theme.colors.neutral[200];
      ctx.lineWidth = 1;
      
      // Vertical lines
      for (let x = 0; x <= boardSize.width; x += gridSize) {
        ctx.beginPath();
        ctx.moveTo(x, 0);
        ctx.lineTo(x, boardSize.height);
        ctx.stroke();
      }
      
      // Horizontal lines
      for (let y = 0; y <= boardSize.height; y += gridSize) {
        ctx.beginPath();
        ctx.moveTo(0, y);
        ctx.lineTo(boardSize.width, y);
        ctx.stroke();
      }
    }
    
    // Draw tokens
    tokens.forEach(token => {
      const isBeingDragged = dragState?.tokenId === token.id;
      const isHovered = token.id === hoveredToken;
      
      // Add shadow for dragged tokens
      if (isBeingDragged) {
        ctx.shadowColor = 'rgba(0, 0, 0, 0.3)';
        ctx.shadowBlur = 10;
        ctx.shadowOffsetX = 2;
        ctx.shadowOffsetY = 2;
      }
      
      ctx.fillStyle = token.color;
      ctx.strokeStyle = isBeingDragged ? theme.colors.primary[500] : isHovered ? theme.colors.neutral[600] : theme.colors.neutral[700];
      ctx.lineWidth = isBeingDragged ? 3 : isHovered ? 2.5 : 2;
      
      ctx.beginPath();
      const radius = isBeingDragged ? token.size + 2 : isHovered ? token.size + 1 : token.size;
      ctx.arc(token.x, token.y, radius, 0, Math.PI * 2);
      ctx.fill();
      ctx.stroke();
      
      // Reset shadow
      ctx.shadowColor = 'transparent';
      ctx.shadowBlur = 0;
      ctx.shadowOffsetX = 0;
      ctx.shadowOffsetY = 0;
      
      // Draw token name
      ctx.fillStyle = theme.colors.text.primary;
      ctx.font = isBeingDragged ? 'bold 12px Arial' : '12px Arial';
      ctx.textAlign = 'center';
      ctx.fillText(token.name, token.x, token.y + radius + 15);
      
      // Draw grid snap indicator
      if (token.snapToGrid) {
        ctx.fillStyle = theme.colors.success[500];
        ctx.font = '10px Arial';
        ctx.fillText('🔒', token.x - radius - 8, token.y - radius - 8);
      }
    });
    
    // Restore the context
    ctx.restore();
  }, [tokens, showGrid, gridSize, boardSize, dragState, hoveredToken, canvasScale, canvasOffset, theme]);

  // Effect to redraw board when dependencies change
  useEffect(() => {
    drawBoard();
  }, [drawBoard]);

  // Handle window resize and update canvas layout
  useEffect(() => {
    const handleResize = () => {
      updateCanvasLayout();
    };

    // Initial layout calculation
    updateCanvasLayout();

    // Add resize listener
    window.addEventListener('resize', handleResize);
    
    // Use ResizeObserver for more precise container size changes
    if (canvasRef.current?.parentElement) {
      const resizeObserver = new ResizeObserver(handleResize);
      resizeObserver.observe(canvasRef.current.parentElement);
      
      return () => {
        window.removeEventListener('resize', handleResize);
        resizeObserver.disconnect();
      };
    }

    return () => {
      window.removeEventListener('resize', handleResize);
    };
  }, [updateCanvasLayout]);

  // Update canvas size when board size changes
  useEffect(() => {
    if (canvasRef.current) {
      const container = canvasRef.current.parentElement;
      if (container) {
        const containerRect = container.getBoundingClientRect();
        canvasRef.current.width = containerRect.width - 32;
        canvasRef.current.height = containerRect.height - 32;
      }
    }
  }, [boardSize]);

  // Cleanup event listeners on unmount
  useEffect(() => {
    return () => {
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);
    };
  }, [handleMouseMove, handleMouseUp]);

  // Add new token
  const addToken = (): void => {
    const newToken: Token = {
      id: Date.now(),
      x: 100,
      y: 100,
      color: `hsl(${Math.random() * 360}, 70%, 60%)`,
      name: `Token ${tokens.length + 1}`,
      size: 25,
      snapToGrid: true
    };
    
    // Snap the new token to grid if grid snapping is enabled
    const snappedToken = {
      ...newToken,
      ...snapToGridPosition(newToken.x, newToken.y, newToken.snapToGrid)
    };
    
    setTokens(prev => [...prev, snappedToken]);
  };

  // Delete token
  const deleteToken = (tokenId: number): void => {
    setTokens(prev => prev.filter(token => token.id !== tokenId));
  };

  // Handle grid size change
  const handleGridSizeChange = (e: React.ChangeEvent<HTMLInputElement>): void => {
    const newGridSize = parseInt(e.target.value);
    setGridSize(newGridSize);
    
    // Reposition tokens to the new grid after a short delay to ensure state is updated
    setTimeout(() => {
      repositionTokensToGrid();
    }, 0);
  };

  // Handle checkbox changes
  const handleShowGridChange = (e: React.ChangeEvent<HTMLInputElement>): void => {
    setShowGrid(e.target.checked);
  };

  // Reposition all tokens to grid (global function)
  const repositionAllTokens = (): void => {
    repositionTokensToGrid();
  };

  return (
    <div className="flex flex-col h-screen bg-gray-100">
      {/* Header */}
      <div className="bg-white shadow-sm border-b p-4">
        <div className="flex items-center justify-between">
          <h1 className="text-2xl font-bold text-gray-800">Game Board</h1>
          <div className="flex gap-4 items-center">
            <button
              onClick={toggleTheme}
              className="flex items-center gap-2 px-4 py-2 bg-gray-100 text-gray-700 rounded-lg hover:bg-gray-200 transition-colors"
            >
              {themeMode === 'light' ? '🌙' : '☀️'}
              {themeMode === 'light' ? 'Dark Mode' : 'Light Mode'}
            </button>
            <button
              onClick={addToken}
              className="flex items-center gap-2 px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors"
            >
              <Users size={16} />
              Add Token
            </button>
          </div>
        </div>
      </div>

      {/* Controls */}
      <div className="bg-white border-b p-3">
        <div className="flex gap-6 items-center">
          <label className="flex items-center gap-2">
            <input
              type="checkbox"
              checked={showGrid}
              onChange={handleShowGridChange}
              className="rounded"
            />
            <Grid3X3 size={16} />
            Show Grid
          </label>
          
          <label className="flex items-center gap-2">
            <span className="text-sm text-gray-600">Grid Size:</span>
            <input
              type="range"
              min="20"
              max="80"
              value={gridSize}
              onChange={handleGridSizeChange}
              className="w-20"
            />
            <span className="text-sm w-8">{gridSize}</span>
          </label>

          <button
            onClick={repositionAllTokens}
            className="flex items-center gap-2 px-3 py-1 bg-gray-100 text-gray-700 rounded hover:bg-gray-200 transition-colors text-sm"
          >
            <Grid3X3 size={14} />
            Snap All to Grid
          </button>
        </div>
      </div>

      {/* Main Content */}
      <div className="flex flex-1 overflow-hidden">
        {/* Sidebar */}
        <div className="w-64 bg-white border-r p-4 overflow-y-auto">
          <h3 className="font-semibold text-gray-800 mb-3">Tokens on Board</h3>
          <div className="space-y-2">
            {tokens.map(token => (
              <div key={token.id} className="flex items-center justify-between p-2 bg-gray-50 rounded">
                <div className="flex items-center gap-2">
                  <div 
                    className="w-4 h-4 rounded-full border"
                    style={{ backgroundColor: token.color }}
                  />
                  <span className="text-sm">{token.name}</span>
                  {token.snapToGrid && <span className="text-xs text-green-600">🔒</span>}
                </div>
                <button
                  onClick={() => deleteToken(token.id)}
                  className="text-red-500 hover:text-red-700 text-xs"
                >
                  ×
                </button>
              </div>
            ))}
          </div>
        </div>

        {/* Canvas Area */}
        <div className="flex-1 overflow-auto bg-gray-50 p-4">
          <div className="bg-white rounded-lg shadow-sm border relative">
            <div className="TokenDisplay">
              {hoveredToken ? <TokenDisplay tokenId={hoveredToken} /> : <>{"No Token Hovered"}</>}
            </div>
            <canvas
              ref={canvasRef}
              className={`border rounded-lg select-none ${
                hoveredToken ? 'cursor-grab' : isDragging ? 'cursor-grabbing' : 'cursor-default'
              }`}
              style={{ 
                width: '100%', 
                height: '100%',
                maxWidth: '100%',
                maxHeight: '100%'
              }}
              onMouseDown={(e) => {
                if (!canvasRef.current) return;
                const rect = canvasRef.current.getBoundingClientRect();
                const canvasCoords = screenToCanvas(e.clientX - rect.left, e.clientY - rect.top);
                const token = getTokenAtPosition(canvasCoords.x, canvasCoords.y);
                
                if (e.button === 0 && token) { // Left click only
                  handleTokenMouseDown(e, token);
                }
              }}
              onContextMenu={(e) => {
                if (!canvasRef.current) return;
                const rect = canvasRef.current.getBoundingClientRect();
                const canvasCoords = screenToCanvas(e.clientX - rect.left, e.clientY - rect.top);
                const token = getTokenAtPosition(canvasCoords.x, canvasCoords.y);
                
                if (token) {
                  e.preventDefault();
                  handleTokenRightClick(e, token);
                } else {
                  e.preventDefault(); // Prevent default context menu on empty space
                }
              }}
              onMouseMove={handleCanvasMouseMove}
              onMouseLeave={() => {
                setHoveredToken(null);
                // If we're dragging and mouse leaves canvas, end the drag
                if (isDraggingRef.current) {
                  cleanupDrag();
                  document.removeEventListener('mousemove', handleMouseMove, { capture: true });
                  document.removeEventListener('mouseup', handleMouseUp, { capture: true });
                }
              }}
            />
          </div>
        </div>
      </div>
    </div>
  );
};

export default GameBoard;