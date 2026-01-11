<script setup lang="ts">
import { useThemeStore } from '../stores/themeStore'

const themeStore = useThemeStore()

const gradientStyle = {
  background: `linear-gradient(to right, var(--theme-primary), var(--theme-secondary))`,
  WebkitBackgroundClip: 'text',
  WebkitTextFillColor: 'transparent',
  backgroundClip: 'text',
}
</script>

<template>
  <div class="h-full p-8 overflow-auto">
    <div class="max-w-3xl mx-auto">
      <h1 class="text-4xl font-bold text-[var(--text-primary)] mb-6" :style="gradientStyle">
        关于
      </h1>
      
      <div class="space-y-6">
        <div class="p-8 rounded-2xl bg-[var(--bg-secondary)] border border-[var(--border-color)]">
          <div class="flex items-center gap-4 mb-6">
            <div 
              class="w-16 h-16 rounded-2xl flex items-center justify-center shadow-lg"
              :style="{ background: `linear-gradient(to bottom right, var(--theme-primary), var(--theme-secondary))` }"
            >
              <span class="text-white text-2xl font-bold">R</span>
            </div>
            <div>
              <h2 class="text-2xl font-bold text-[var(--text-primary)]">Roco Dream Helper</h2>
              <p class="text-[var(--text-secondary)]">版本 v0.1.0</p>
            </div>
          </div>
          <p class="text-[var(--text-secondary)] leading-relaxed">
            一个基于 Tauri 构建的现代化桌面应用，采用 Vue 3 + TypeScript + Tailwind CSS 技术栈，提供优雅的用户体验和强大的功能。
          </p>
        </div>
        
        <div class="p-8 rounded-2xl bg-[var(--bg-secondary)] border border-[var(--border-color)]">
          <h2 class="text-xl font-semibold text-[var(--text-primary)] mb-4 flex items-center gap-2">
            <svg width="20" height="20" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M10 2L2 7l8 5 8-5-8-5zM2 17l8 5 8-5M2 12l8 5 8-5"/>
            </svg>
            技术栈
          </h2>
          <div class="grid grid-cols-2 gap-4">
            <div class="flex items-center gap-3 p-3 rounded-lg bg-[var(--bg-primary)] border border-[var(--border-color)]">
              <div class="w-8 h-8 rounded flex items-center justify-center" :style="{ backgroundColor: `${themeStore.themeColors.primary}10` }">
                <span class="font-bold text-sm" :style="{ color: themeStore.themeColors.primary }">T</span>
              </div>
              <span class="text-[var(--text-primary)] font-medium">Tauri</span>
            </div>
            <div class="flex items-center gap-3 p-3 rounded-lg bg-[var(--bg-primary)] border border-[var(--border-color)]">
              <div class="w-8 h-8 rounded bg-green-500/10 flex items-center justify-center">
                <span class="text-green-500 font-bold text-sm">V</span>
              </div>
              <span class="text-[var(--text-primary)] font-medium">Vue 3</span>
            </div>
            <div class="flex items-center gap-3 p-3 rounded-lg bg-[var(--bg-primary)] border border-[var(--border-color)]">
              <div class="w-8 h-8 rounded flex items-center justify-center" :style="{ backgroundColor: `${themeStore.themeColors.secondary}10` }">
                <span class="font-bold text-sm" :style="{ color: themeStore.themeColors.secondary }">TS</span>
              </div>
              <span class="text-[var(--text-primary)] font-medium">TypeScript</span>
            </div>
            <div class="flex items-center gap-3 p-3 rounded-lg bg-[var(--bg-primary)] border border-[var(--border-color)]">
              <div class="w-8 h-8 rounded flex items-center justify-center" :style="{ backgroundColor: `${themeStore.themeColors.accent}10` }">
                <span class="font-bold text-sm" :style="{ color: themeStore.themeColors.accent }">T</span>
              </div>
              <span class="text-[var(--text-primary)] font-medium">Tailwind CSS</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
</style>
