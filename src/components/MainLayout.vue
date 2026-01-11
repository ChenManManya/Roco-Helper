<script setup lang="ts">
import { computed } from 'vue'
import TitleBar from './TitleBar.vue'
import Sidebar from './Sidebar.vue'
import { useThemeStore } from '../stores/themeStore'

const themeStore = useThemeStore()

const backgroundStyle = computed(() => {
  const style: Record<string, string> = {
    backgroundColor: 'var(--bg-primary)',
  }
  
  if (themeStore.backgroundImage) {
    style.backgroundImage = `url(${themeStore.backgroundImage})`
    style.backgroundSize = 'cover'
    style.backgroundPosition = 'center'
    style.backgroundRepeat = 'no-repeat'
  }
  
  return style
})
</script>

<template>
  <div 
    class="main-layout h-screen flex flex-col overflow-hidden"
    :style="backgroundStyle"
  >
    <!-- 标题栏 -->
    <TitleBar />
    
    <!-- 主内容区 -->
    <div class="flex flex-1 overflow-hidden">
      <!-- 侧边栏 -->
      <Sidebar />
      
      <!-- 内容区域 -->
      <main 
        class="flex-1 overflow-auto bg-[var(--bg-primary)]"
        :class="{ 'bg-[var(--bg-primary)]/95 backdrop-blur-sm': themeStore.backgroundImage }"
      >
        <router-view />
      </main>
    </div>
  </div>
</template>

<style scoped>
.main-layout {
  color: var(--text-primary);
}
</style>
