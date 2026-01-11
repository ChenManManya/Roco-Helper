<script setup lang="ts">
import { useThemeStore } from '../stores/themeStore'
import { useRouter, useRoute } from 'vue-router'

const themeStore = useThemeStore()
const router = useRouter()
const route = useRoute()

const menuItems = [
  { 
    path: '/', 
    label: '首页', 
    icon: `<svg width="20" height="20" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
      <path d="M3 9l7-7 7 7M5 9v7a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2V9" />
    </svg>`
  },
  { 
    path: '/settings', 
    label: '设置', 
    icon: `<svg width="20" height="20" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
      <circle cx="10" cy="10" r="3" />
      <path d="M10 1v2M10 17v2M19 10h-2M3 10H1m15.07-5.07l-1.41 1.41M5.34 5.34L3.93 3.93m12.14 12.14l-1.41 1.41M5.34 14.66L3.93 16.07" />
    </svg>`
  },
  { 
    path: '/about', 
    label: '关于', 
    icon: `<svg width="20" height="20" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
      <circle cx="10" cy="10" r="8" />
      <path d="M10 6v4M10 14h.01" />
    </svg>`
  },
]

const isActive = (path: string) => {
  return route.path === path
}

const navigate = (path: string) => {
  router.push(path)
}
</script>

<template>
  <div 
    class="sidebar h-full transition-all duration-300 flex flex-col"
    :class="[
      themeStore.themeMode === 'dark' 
        ? 'bg-gray-900/50 border-gray-800' 
        : 'bg-gray-50/80 border-gray-200',
      themeStore.sidebarCollapsed ? 'w-16' : 'w-64'
    ]"
    style="border-right: 1px solid var(--border-color);"
  >
    <!-- 折叠按钮 -->
    <div class="p-3 border-b" style="border-color: var(--border-color);">
      <button
        @click="themeStore.toggleSidebar"
        class="w-full h-9 flex items-center justify-center rounded-lg transition-all hover:bg-black/5 dark:hover:bg-white/5 text-[var(--text-secondary)] hover:text-[var(--text-primary)]"
        title="切换侧边栏"
      >
        <svg v-if="!themeStore.sidebarCollapsed" width="18" height="18" viewBox="0 0 18 18" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
          <path d="M12 6l-4 4 4 4M6 6l-4 4 4 4" />
        </svg>
        <svg v-else width="18" height="18" viewBox="0 0 18 18" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
          <path d="M6 6l4 4-4 4M12 6l4 4-4 4" />
        </svg>
      </button>
    </div>

    <!-- 菜单项 -->
    <nav class="flex-1 p-2 space-y-1.5 mt-2">
      <button
        v-for="item in menuItems"
        :key="item.path"
        @click="navigate(item.path)"
        class="w-full flex items-center gap-3 px-3 py-2.5 rounded-xl transition-all duration-200 relative group"
        :class="{
          'text-white shadow-lg': isActive(item.path),
          'text-[var(--text-secondary)] hover:bg-black/5 dark:hover:bg-white/5 hover:text-[var(--text-primary)]': !isActive(item.path)
        }"
        :style="isActive(item.path) ? {
          background: `linear-gradient(to right, var(--theme-primary), var(--theme-secondary))`,
          boxShadow: `0 10px 15px -3px ${themeStore.themeColors.primary}30, 0 4px 6px -2px ${themeStore.themeColors.primary}20`
        } : {}"
        :title="themeStore.sidebarCollapsed ? item.label : ''"
      >
        <span 
          class="flex-shrink-0 flex items-center justify-center"
          v-html="item.icon"
        ></span>
        <span 
          v-if="!themeStore.sidebarCollapsed"
          class="font-medium text-sm flex-1 text-left"
        >
          {{ item.label }}
        </span>
        <div 
          v-if="isActive(item.path) && !themeStore.sidebarCollapsed"
          class="absolute right-2 w-1.5 h-1.5 rounded-full bg-white/80"
        ></div>
      </button>
    </nav>

    <!-- 底部区域 -->
    <div class="p-3 border-t" style="border-color: var(--border-color);">
      <div v-if="!themeStore.sidebarCollapsed" class="text-xs text-[var(--text-secondary)] text-center opacity-60">
        v0.1.0
      </div>
    </div>
  </div>
</template>

<style scoped>
.sidebar {
  min-width: 64px;
}
</style>
