<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window'
import { useThemeStore } from '../stores/themeStore'

const appWindow = getCurrentWindow()
const themeStore = useThemeStore()

const minimize = async () => {
  await appWindow.minimize()
}

const maximize = async () => {
  await appWindow.toggleMaximize()
}

const close = async () => {
  await appWindow.close()
}
</script>

<template>
  <div 
    data-tauri-drag-region
    class="titlebar h-11 flex items-center justify-between px-4 select-none backdrop-blur-sm"
    :class="themeStore.themeMode === 'dark' ? 'bg-gray-900/90 border-gray-800' : 'bg-white/90 border-gray-200'"
    style="border-bottom: 1px solid var(--border-color);"
  >
    <div class="titlebar-title flex items-center gap-3">
      <div 
        class="w-5 h-5 rounded flex items-center justify-center"
        :style="{ background: `linear-gradient(to bottom right, var(--theme-primary), var(--theme-secondary))` }"
      >
        <span class="text-white text-xs font-bold">R</span>
      </div>
      <span class="text-sm font-semibold text-[var(--text-primary)]">Roco Dream Helper</span>
    </div>
    
    <div class="titlebar-controls flex items-center">
      <button
        @click="minimize"
        class="titlebar-button w-11 h-11 flex items-center justify-center hover:bg-black/5 dark:hover:bg-white/5 transition-colors text-[var(--text-secondary)] hover:text-[var(--text-primary)]"
        title="最小化"
      >
        <svg width="10" height="1" viewBox="0 0 10 1" fill="currentColor">
          <rect width="10" height="1" />
        </svg>
      </button>
      
      <button
        @click="maximize"
        class="titlebar-button w-11 h-11 flex items-center justify-center hover:bg-black/5 dark:hover:bg-white/5 transition-colors text-[var(--text-secondary)] hover:text-[var(--text-primary)]"
        title="最大化/还原"
      >
        <svg width="10" height="10" viewBox="0 0 10 10" fill="none" stroke="currentColor" stroke-width="1.2">
          <rect x="0.5" y="0.5" width="9" height="9" rx="0.5" />
        </svg>
      </button>
      
      <button
        @click="close"
        class="titlebar-button w-11 h-11 flex items-center justify-center hover:bg-red-500 hover:text-white transition-colors text-[var(--text-secondary)]"
        title="关闭"
      >
        <svg width="10" height="10" viewBox="0 0 10 10" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round">
          <line x1="1.5" y1="1.5" x2="8.5" y2="8.5" />
          <line x1="8.5" y1="1.5" x2="1.5" y2="8.5" />
        </svg>
      </button>
    </div>
  </div>
</template>

<style scoped>
.titlebar {
  -webkit-app-region: drag;
}

.titlebar-button {
  -webkit-app-region: no-drag;
}
</style>
