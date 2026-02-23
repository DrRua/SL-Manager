// 主脚本文件
import React, { useState } from 'react'
import ReactDOM from 'react-dom/client'
import { Button, Card, List, Input, Modal, message } from 'antd'
import 'antd/dist/reset.css'

console.log('SL-Manager 初始化成功！')

const { TextArea } = Input

const App = () => {
  const [games, setGames] = useState([])
  const [selectedGame, setSelectedGame] = useState(null)
  const [isAddModalVisible, setIsAddModalVisible] = useState(false)
  const [newGame, setNewGame] = useState({
    name: '',
    savePath: '',
    backupPath: ''
  })

  // 添加新游戏
  const addGame = () => {
    if (!newGame.name || !newGame.savePath || !newGame.backupPath) {
      message.error('请填写完整的游戏信息')
      return
    }

    const updatedGames = [...games, newGame]
    setGames(updatedGames)
    setSelectedGame(newGame)
    setIsAddModalVisible(false)
    setNewGame({ name: '', savePath: '', backupPath: '' })
    message.success('游戏添加成功')
  }

  // 选择游戏
  const handleGameSelect = (game) => {
    setSelectedGame(game)
  }

  return (
    <div style={{ 
      padding: 24, 
      height: 'calc(100vh - 48px)', 
      boxSizing: 'border-box' 
    }}>
      <div style={{ 
        display: 'flex', 
        gap: 24, 
        height: '100%' 
      }}>
        {/* 左侧卡片 - 40% 宽度 */}
        <Card 
          title={
            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
              游戏列表
              <Button 
                type="default" 
                size="small" 
                style={{ 
                  borderColor: '#52c41a', 
                  color: '#52c41a', 
                  borderRadius: 4 
                }} 
                onClick={() => setIsAddModalVisible(true)}
              >
                添加
              </Button>
            </div>
          } 
          style={{ 
            width: '40%', 
            height: '100%' 
          }} 
        >
          <List
            dataSource={games}
            renderItem={(game) => (
              <List.Item
                style={{
                  cursor: 'pointer',
                  backgroundColor: selectedGame?.name === game.name ? '#f0f0f0' : 'transparent'
                }}
                onClick={() => handleGameSelect(game)}
              >
                {game.name}
              </List.Item>
            )}
            locale={{ emptyText: '暂无游戏配置，请点击右上角添加' }}
          />
        </Card>
        
        {/* 右侧卡片 - 60% 宽度 */}
        <Card 
          title="存档管理" 
          style={{ 
            width: '60%', 
            height: '100%' 
          }} 
        >
          {selectedGame ? (
            <div>
              <h3>{selectedGame.name}</h3>
              <p>存档路径: {selectedGame.savePath}</p>
              <p>备份路径: {selectedGame.backupPath}</p>
              <div style={{ marginTop: 24 }}>
                <Button type="default" style={{ marginRight: 8 }}>保存存档</Button>
                <Button type="default">恢复存档</Button>
              </div>
            </div>
          ) : (
            <p>请从左侧选择一个游戏</p>
          )}
        </Card>
      </div>

      {/* 添加游戏模态框 */}
      <Modal
        title="添加游戏配置"
        open={isAddModalVisible}
        onOk={addGame}
        onCancel={() => setIsAddModalVisible(false)}
      >
        <div style={{ marginBottom: 16 }}>
          <label style={{ display: 'block', marginBottom: 8 }}>游戏名称</label>
          <Input
            value={newGame.name}
            onChange={(e) => setNewGame({ ...newGame, name: e.target.value })}
          />
        </div>
        <div style={{ marginBottom: 16 }}>
          <label style={{ display: 'block', marginBottom: 8 }}>存档路径</label>
          <Input
            value={newGame.savePath}
            onChange={(e) => setNewGame({ ...newGame, savePath: e.target.value })}
          />
        </div>
        <div style={{ marginBottom: 16 }}>
          <label style={{ display: 'block', marginBottom: 8 }}>备份路径</label>
          <Input
            value={newGame.backupPath}
            onChange={(e) => setNewGame({ ...newGame, backupPath: e.target.value })}
          />
        </div>
      </Modal>
    </div>
  )
}

ReactDOM.createRoot(document.getElementById('app')).render(<App />)