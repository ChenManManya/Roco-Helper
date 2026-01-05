<template>
  <div class="flex flex-col gap-4 p-8 max-w-2xl mx-auto">
    <h2 class="text-2xl font-bold text-gray-800 mb-4">Tailwind CSS 按钮设计</h2>
    
    <!-- 主按钮 - 渐变效果 -->
    <button class="bg-gradient-to-r from-blue-500 to-purple-600 text-white font-semibold py-3 px-8 rounded-lg shadow-lg hover:shadow-xl transform hover:-translate-y-0.5 transition-all duration-300 focus:outline-none focus:ring-2 focus:ring-blue-400 focus:ring-opacity-75">
      主按钮
    </button>
    
    <!-- 次要按钮 - 轮廓效果 -->
    <button class="bg-white border-2 border-blue-500 text-blue-600 font-semibold py-3 px-8 rounded-lg shadow hover:bg-blue-50 transform hover:-translate-y-0.5 transition-all duration-300 focus:outline-none focus:ring-2 focus:ring-blue-400 focus:ring-opacity-75">
      次要按钮
    </button>
    
    <!-- 带图标的按钮 -->
    <button class="bg-gradient-to-r from-green-400 to-emerald-600 text-white font-semibold py-3 px-8 rounded-lg shadow-lg flex items-center gap-2 hover:shadow-xl transform hover:-translate-y-0.5 transition-all duration-300 focus:outline-none focus:ring-2 focus:ring-green-400 focus:ring-opacity-75">
      <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path>
      </svg>
      带图标按钮
    </button>
    
    <!-- 危险按钮 -->
    <button class="bg-gradient-to-r from-red-500 to-pink-600 text-white font-semibold py-3 px-8 rounded-lg shadow-lg hover:shadow-xl transform hover:-translate-y-0.5 transition-all duration-300 focus:outline-none focus:ring-2 focus:ring-red-400 focus:ring-opacity-75">
      危险按钮
    </button>
    
    <!-- 加载状态按钮 -->
    <button class="bg-gradient-to-r from-indigo-500 to-purple-600 text-white font-semibold py-3 px-8 rounded-lg shadow-lg flex items-center justify-center gap-2 hover:shadow-xl transform hover:-translate-y-0.5 transition-all duration-300 focus:outline-none focus:ring-2 focus:ring-indigo-400 focus:ring-opacity-75" disabled>
      <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
      </svg>
      加载中...
    </button>
    
    <!-- 圆角按钮 -->
    <button class="bg-gradient-to-r from-yellow-400 to-orange-500 text-white font-semibold py-3 px-8 rounded-full shadow-lg hover:shadow-xl transform hover:-translate-y-0.5 transition-all duration-300 focus:outline-none focus:ring-2 focus:ring-yellow-400 focus:ring-opacity-75">
      圆角按钮
    </button>
    
    <!-- 文本按钮 -->
    <button class="text-blue-600 font-medium py-3 px-8 hover:text-blue-800 hover:bg-blue-50 rounded-lg transition-all duration-300 focus:outline-none focus:ring-2 focus:ring-blue-400 focus:ring-opacity-75">
      文本按钮
    </button>
    
    <!-- 块级按钮 -->
    <button class="bg-gradient-to-r from-teal-500 to-cyan-600 text-white font-semibold py-3 px-8 rounded-lg shadow-lg hover:shadow-xl transform hover:-translate-y-0.5 transition-all duration-300 focus:outline-none focus:ring-2 focus:ring-teal-400 focus:ring-opacity-75 w-full">
      块级按钮
    </button>
  </div>
</template>

<script setup lang="ts">
// 这个组件展示了多种使用 Tailwind CSS 设计的美观按钮
</script>
