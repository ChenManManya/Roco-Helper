
<script setup lang="ts">
  import { invoke } from '@tauri-apps/api/core'
  async function navigateToLogin() {
    await invoke('navigate_to_login')
  }
  // await invoke('navigate_to_login')
</script>

<template>
  <div style="display: flex; flex-direction: column; padding: 32px; max-width: 800px; margin: 0 auto;">
    <h2 class="text-2xl font-bold text-gray-800 mb-4">内联样式按钮设计</h2>
    
    <!-- 主按钮 - 渐变效果 -->
    <button style="
      background: linear-gradient(to right, #3b82f6, #8b5cf6);
      color: white;
      font-weight: 600;
      padding: 12px 32px;
      border-radius: 8px;
      box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
      transition: all 0.3s ease;
      border: none;
      cursor: pointer;
      margin-bottom: 24px;
    " onmouseover="this.style.transform='translateY(-2px)'; this.style.boxShadow='0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05)'" onmouseout="this.style.transform='translateY(0)'; this.style.boxShadow='0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)'" @click="navigateToLogin">
      去登陆
    </button>
    
    <!-- 次要按钮 - 轮廓效果 -->
    <button style="
      background: white;
      color: #3b82f6;
      border: 2px solid #3b82f6;
      font-weight: 600;
      padding: 12px 32px;
      border-radius: 8px;
      box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
      transition: all 0.3s ease;
      cursor: pointer;
      margin-bottom: 24px;
    " onmouseover="this.style.backgroundColor='#eff6ff'; this.style.transform='translateY(-2px)'; this.style.boxShadow='0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05)'" onmouseout="this.style.backgroundColor='white'; this.style.transform='translateY(0)'; this.style.boxShadow='0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)'">
      次要按钮
    </button>
    
    <!-- 带图标的按钮 -->
    <button style="
      background: linear-gradient(to right, #10b981, #059669);
      color: white;
      font-weight: 600;
      padding: 12px 32px;
      border-radius: 8px;
      box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
      transition: all 0.3s ease;
      border: none;
      cursor: pointer;
      display: flex;
      align-items: center;
      gap: 8px;
      margin-bottom: 24px;
    " onmouseover="this.style.transform='translateY(-2px)'; this.style.boxShadow='0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05)'" onmouseout="this.style.transform='translateY(0)'; this.style.boxShadow='0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)'">
      <svg width="20" height="20" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path>
      </svg>
      带图标按钮
    </button>
    
    <!-- 危险按钮 -->
    <button style="
      background: linear-gradient(to right, #ef4444, #dc2626);
      color: white;
      font-weight: 600;
      padding: 12px 32px;
      border-radius: 8px;
      box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
      transition: all 0.3s ease;
      border: none;
      cursor: pointer;
      margin-bottom: 24px;
    " onmouseover="this.style.transform='translateY(-2px)'; this.style.boxShadow='0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05)'" onmouseout="this.style.transform='translateY(0)'; this.style.boxShadow='0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)'">
      危险按钮
    </button>
    
    <!-- 加载状态按钮 -->
    <button style="
      background: linear-gradient(to right, #6366f1, #8b5cf6);
      color: white;
      font-weight: 600;
      padding: 12px 32px;
      border-radius: 8px;
      box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
      transition: all 0.3s ease;
      border: none;
      cursor: not-allowed;
      display: flex;
      align-items: center;
      justify-content: center;
      gap: 8px;
      margin-bottom: 24px;
    ">
      <svg style="animation: rotate 1s linear infinite; margin-left: -4px; margin-right: 12px; height: 20px; width: 20px; color: white;" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
      </svg>
      加载中...
    </button>
    
    <!-- 圆角按钮 -->
    <button style="
      background: linear-gradient(to right, #f59e0b, #d97706);
      color: white;
      font-weight: 600;
      padding: 12px 32px;
      border-radius: 9999px;
      box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
      transition: all 0.3s ease;
      border: none;
      cursor: pointer;
      margin-bottom: 24px;
    " onmouseover="this.style.transform='translateY(-2px)'; this.style.boxShadow='0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05)'" onmouseout="this.style.transform='translateY(0)'; this.style.boxShadow='0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)'">
      圆角按钮
    </button>
    
    <!-- 文本按钮 -->
    <button style="
      background: transparent;
      color: #3b82f6;
      font-weight: 500;
      padding: 12px 32px;
      border-radius: 8px;
      transition: all 0.3s ease;
      border: none;
      cursor: pointer;
      margin-bottom: 24px;
    " onmouseover="this.style.backgroundColor='#eff6ff'; this.style.color='#2563eb'" onmouseout="this.style.backgroundColor='transparent'; this.style.color='#3b82f6'">
      文本按钮
    </button>
    
    <!-- 块级按钮 -->
    <button style="
      background: linear-gradient(to right, #14b8a6, #0d9488);
      color: white;
      font-weight: 600;
      padding: 12px 32px;
      border-radius: 8px;
      box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
      transition: all 0.3s ease;
      border: none;
      cursor: pointer;
      width: 100%;
      margin-bottom: 24px;
    " onmouseover="this.style.transform='translateY(-2px)'; this.style.boxShadow='0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05)'" onmouseout="this.style.transform='translateY(0)'; this.style.boxShadow='0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)'">
      块级按钮
    </button>
  </div>
</template>

<style>
@keyframes rotate {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}
</style>

