<script setup lang="ts">
import { ref } from 'vue'
import { useThemeStore } from '../stores/themeStore'
import { getRecentChangelogs, type ChangelogItem } from '../data/changelog'

const themeStore = useThemeStore()

const gradientStyle = {
  background: `linear-gradient(to right, var(--theme-primary), var(--theme-secondary))`,
  WebkitBackgroundClip: 'text',
  WebkitTextFillColor: 'transparent',
  backgroundClip: 'text',
}

const changelogs = ref<ChangelogItem[]>(getRecentChangelogs(3))
const initialVersion = changelogs.value[0]?.version || ''
const expandedVersions = ref<Set<string>>(new Set(initialVersion ? [initialVersion] : []))

const toggleExpand = (version: string) => {
  if (expandedVersions.value.has(version)) {
    expandedVersions.value.delete(version)
  } else {
    expandedVersions.value.add(version)
  }
}

const getTypeLabel = (type: ChangelogItem['type']) => {
  switch (type) {
    case 'feature':
      return '新功能'
    case 'bugfix':
      return '修复'
    case 'improvement':
      return '改进'
    case 'breaking':
      return '重大变更'
    default:
      return ''
  }
}

const getTypeColor = (type: ChangelogItem['type']) => {
  switch (type) {
    case 'feature':
      return themeStore.themeColors.primary
    case 'bugfix':
      return '#ef4444'
    case 'improvement':
      return themeStore.themeColors.secondary
    case 'breaking':
      return '#f59e0b'
    default:
      return themeStore.themeColors.primary
  }
}
</script>

<template>
  <div class="h-full p-8 overflow-auto">
    <div class="max-w-5xl mx-auto">
      <!-- 欢迎区域 -->
      <div class="mb-12">
        <h1 class="text-4xl font-bold text-[var(--text-primary)] mb-3" :style="gradientStyle">
          欢迎使用 Roco Helper
        </h1>
        <p class="text-lg text-[var(--text-secondary)]">一个功能强大、设计精美的辅助工具</p>
      </div>
      
      <!-- 功能卡片 -->
      <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
        <div 
          class="group p-6 rounded-2xl border transition-all duration-300 hover:shadow-xl hover:-translate-y-1 cursor-pointer"
          :class="[
            'bg-[var(--bg-secondary)]',
            'border-[var(--border-color)]'
          ]"
          :style="{ borderColor: 'var(--border-color)' }"
          @mouseenter="(e: Event) => { (e.currentTarget as HTMLElement).style.borderColor = themeStore.themeColors.primary }"
          @mouseleave="(e: Event) => { (e.currentTarget as HTMLElement).style.borderColor = 'var(--border-color)' }"
        >
          <div 
            class="w-12 h-12 rounded-xl flex items-center justify-center mb-4 shadow-lg"
            :style="{ 
              background: `linear-gradient(to bottom right, var(--theme-primary), var(--theme-secondary))`,
              boxShadow: `0 10px 15px -3px ${themeStore.themeColors.primary}20`
            }"
          >
            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"/>
            </svg>
          </div>
          <h2 class="text-xl font-semibold text-[var(--text-primary)] mb-2">功能特性</h2>
          <p class="text-[var(--text-secondary)] leading-relaxed">丰富的功能模块，满足您的各种需求，持续更新中</p>
        </div>
        
        <div 
          class="group p-6 rounded-2xl border transition-all duration-300 hover:shadow-xl hover:-translate-y-1 cursor-pointer"
          :class="[
            'bg-[var(--bg-secondary)]',
            'border-[var(--border-color)]'
          ]"
          :style="{ borderColor: 'var(--border-color)' }"
          @mouseenter="(e: Event) => { (e.currentTarget as HTMLElement).style.borderColor = themeStore.themeColors.secondary }"
          @mouseleave="(e: Event) => { (e.currentTarget as HTMLElement).style.borderColor = 'var(--border-color)' }"
        >
          <div 
            class="w-12 h-12 rounded-xl flex items-center justify-center mb-4 shadow-lg"
            :style="{ 
              background: `linear-gradient(to bottom right, var(--theme-secondary), var(--theme-accent))`,
              boxShadow: `0 10px 15px -3px ${themeStore.themeColors.secondary}20`
            }"
          >
            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
              <polyline points="14 2 14 8 20 8"/>
              <line x1="16" y1="13" x2="8" y2="13"/>
              <line x1="16" y1="17" x2="8" y2="17"/>
              <polyline points="10 9 9 9 8 9"/>
            </svg>
          </div>
          <h2 class="text-xl font-semibold text-[var(--text-primary)] mb-2">使用说明</h2>
          <p class="text-[var(--text-secondary)] leading-relaxed">详细的使用文档和教程，帮助您快速上手</p>
        </div>
      </div>

      <!-- 更新日志 -->
      <div class="mt-8">
        <div class="flex items-center gap-3 mb-6">
          <div 
            class="w-10 h-10 rounded-xl flex items-center justify-center"
            :style="{ 
              background: `linear-gradient(to bottom right, var(--theme-primary), var(--theme-secondary))`,
            }"
          >
            <svg width="20" height="20" viewBox="0 0 20 20" fill="none" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M10 2L2 6l8 4 8-4-8-4zM2 14l8 4 8-4M2 10l8 4 8-4"/>
            </svg>
          </div>
          <h2 class="text-2xl font-bold text-[var(--text-primary)]">更新日志</h2>
        </div>

        <div class="space-y-4">
          <div
            v-for="log in changelogs"
            :key="log.version"
            class="rounded-2xl border bg-[var(--bg-secondary)] border-[var(--border-color)] overflow-hidden transition-all"
          >
            <!-- 版本头部 -->
            <button
              @click="toggleExpand(log.version)"
              class="w-full p-5 flex items-center justify-between hover:bg-[var(--bg-tertiary)] transition-colors"
            >
              <div class="flex items-center gap-4 flex-1">
                <div 
                  class="w-8 h-8 rounded-lg flex items-center justify-center text-white text-xs font-bold"
                  :style="{ backgroundColor: getTypeColor(log.type) }"
                >
                  v{{ log.version }}
                </div>
                <div class="flex-1 text-left">
                  <div class="flex items-center gap-3 mb-1">
                    <span class="text-lg font-semibold text-[var(--text-primary)]">版本 {{ log.version }}</span>
                    <span 
                      class="px-2 py-0.5 rounded text-xs font-medium text-white"
                      :style="{ backgroundColor: getTypeColor(log.type) + '80' }"
                    >
                      {{ getTypeLabel(log.type) }}
                    </span>
                  </div>
                  <span class="text-sm text-[var(--text-secondary)]">{{ log.date }}</span>
                </div>
              </div>
              <svg 
                width="20" 
                height="20" 
                viewBox="0 0 20 20" 
                fill="none" 
                stroke="currentColor" 
                stroke-width="2" 
                stroke-linecap="round" 
                stroke-linejoin="round"
                class="text-[var(--text-secondary)] transition-transform"
                :class="{ 'rotate-180': expandedVersions.has(log.version) }"
              >
                <polyline points="6 8 10 12 14 8"/>
              </svg>
            </button>

            <!-- 更新内容 -->
            <div 
              v-if="expandedVersions.has(log.version)"
              class="px-5 pb-5 border-t border-[var(--border-color)] pt-4"
            >
              <ul class="space-y-2">
                <li
                  v-for="(item, itemIndex) in log.items"
                  :key="itemIndex"
                  class="flex items-start gap-3 text-[var(--text-secondary)]"
                >
                  <span 
                    class="mt-1.5 w-1.5 h-1.5 rounded-full flex-shrink-0"
                    :style="{ backgroundColor: getTypeColor(log.type) }"
                  ></span>
                  <span class="flex-1">{{ item }}</span>
                </li>
              </ul>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
</style>
