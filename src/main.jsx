// 主脚本文件
import React from 'react'
import ReactDOM from 'react-dom/client'
import { Button } from 'antd'
import 'antd/dist/reset.css'

console.log('SL-Manager 初始化成功！')

const App = () => {
  return (
    <div style={{ padding: 24 }}>
      <h1>SL-Manager</h1>
      <p>游戏存档管理工具</p>
      <Button type="primary">测试按钮</Button>
    </div>
  )
}

ReactDOM.createRoot(document.getElementById('app')).render(<App />)