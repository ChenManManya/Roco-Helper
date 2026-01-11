<script setup lang="ts">
import { ref } from 'vue'
import { useThemeStore, type ThemeColor } from '../stores/themeStore'

const themeStore = useThemeStore()

const themeColors: { value: ThemeColor; label: string; color: string }[] = [
  { value: 'blue', label: '蓝色', color: '#3b82f6' },
  { value: 'green', label: '绿色', color: '#10b981' },
  { value: 'purple', label: '紫色', color: '#8b5cf6' },
  { value: 'orange', label: '橙色', color: '#f59e0b' },
  { value: 'red', label: '红色', color: '#ef4444' },
  { value: 'pink', label: '粉色', color: '#ec4899' },
]

const fileInputRef = ref<HTMLInputElement | null>(null)

const handleImageSelect = (event: Event) => {
  const target = event.target as HTMLInputElement
  const file = target.files?.[0]
  if (file) {
    const reader = new FileReader()
    reader.onload = (e) => {
      const result = e.target?.result as string
      themeStore.setBackgroundImage(result)
    }
    reader.readAsDataURL(file)
  }
}

const removeBackgroundImage = () => {
  themeStore.setBackgroundImage(null)
  if (fileInputRef.value) {
    fileInputRef.value.value = ''
  }
}
</script>

<template>
  <div class="h-full p-8 overflow-auto">
    <div class="max-w-4xl mx-auto">
      <h1 
        class="text-4xl font-bold text-[var(--text-primary)] mb-8"
        :style="{
          background: 'linear-gradient(to right, var(--theme-primary), var(--theme-secondary))',
          WebkitBackgroundClip: 'text',
          WebkitTextFillColor: 'transparent',
          backgroundClip: 'text'
        }"
      >
        设置
      </h1>

      <div class="space-y-6">
        <!-- 主题模式设置 -->
        <div class="p-6 rounded-2xl bg-[var(--bg-secondary)] border border-[var(--border-color)]">
          <h2 class="text-xl font-semibold text-[var(--text-primary)] mb-4 flex items-center gap-2">
            <svg width="20" height="20" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <circle cx="10" cy="10" r="3" />
              <path d="M10 1v2M10 17v2M19 10h-2M3 10H1m15.07-5.07l-1.41 1.41M5.34 5.34L3.93 3.93m12.14 12.14l-1.41 1.41M5.34 14.66L3.93 16.07" />
            </svg>
            主题模式
          </h2>
          <div class="flex gap-4">
            <button
              @click="themeStore.setThemeMode('light')"
              class="flex-1 p-4 rounded-xl border-2 transition-all"
              :class="themeStore.themeMode === 'light' 
                ? 'border-[var(--theme-primary)] bg-[var(--theme-primary)]/10' 
                : 'border-[var(--border-color)] hover:border-[var(--theme-primary)]/50 bg-[var(--bg-primary)]'"
            >
              <div class="flex flex-col items-center gap-2">
                <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <circle cx="12" cy="12" r="5" />
                  <line x1="12" y1="1" x2="12" y2="3" />
                  <line x1="12" y1="21" x2="12" y2="23" />
                  <line x1="4.22" y1="4.22" x2="5.64" y2="5.64" />
                  <line x1="18.36" y1="18.36" x2="19.78" y2="19.78" />
                  <line x1="1" y1="12" x2="3" y2="12" />
                  <line x1="21" y1="12" x2="23" y2="12" />
                  <line x1="4.22" y1="19.78" x2="5.64" y2="18.36" />
                  <line x1="18.36" y1="5.64" x2="19.78" y2="4.22" />
                </svg>
                <span class="font-medium text-[var(--text-primary)]">亮色</span>
              </div>
            </button>
            <button
              @click="themeStore.setThemeMode('dark')"
              class="flex-1 p-4 rounded-xl border-2 transition-all"
              :class="themeStore.themeMode === 'dark' 
                ? 'border-[var(--theme-primary)] bg-[var(--theme-primary)]/10' 
                : 'border-[var(--border-color)] hover:border-[var(--theme-primary)]/50 bg-[var(--bg-primary)]'"
            >
              <div class="flex flex-col items-center gap-2">
                <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z" />
                </svg>
                <span class="font-medium text-[var(--text-primary)]">暗色</span>
              </div>
            </button>
          </div>
        </div>

        <!-- 主题色设置 -->
        <div class="p-6 rounded-2xl bg-[var(--bg-secondary)] border border-[var(--border-color)]">
          <h2 class="text-xl font-semibold text-[var(--text-primary)] mb-4 flex items-center gap-2">
            <svg width="20" height="20" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M12 2.69l5.66 5.66a8 8 0 1 1-11.31 0L12 2.69z" />
            </svg>
            主题色
          </h2>
          <div class="grid grid-cols-3 sm:grid-cols-6 gap-3">
            <button
              v-for="color in themeColors"
              :key="color.value"
              @click="themeStore.setThemeColor(color.value)"
              class="p-4 rounded-xl border-2 transition-all hover:scale-105"
              :class="themeStore.themeColor === color.value 
                ? 'border-[var(--theme-primary)] ring-2 ring-[var(--theme-primary)]/30' 
                : 'border-[var(--border-color)] hover:border-[var(--theme-primary)]/50'"
              :style="{ backgroundColor: color.color + '20' }"
              :title="color.label"
            >
              <div class="flex flex-col items-center gap-2">
                <div 
                  class="w-12 h-12 rounded-full shadow-lg"
                  :style="{ backgroundColor: color.color }"
                ></div>
                <span class="text-sm font-medium text-[var(--text-primary)]">{{ color.label }}</span>
              </div>
            </button>
          </div>
        </div>

        <!-- 背景图片设置 -->
        <div class="p-6 rounded-2xl bg-[var(--bg-secondary)] border border-[var(--border-color)]">
          <h2 class="text-xl font-semibold text-[var(--text-primary)] mb-4 flex items-center gap-2">
            <svg width="20" height="20" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <rect x="3" y="3" width="14" height="14" rx="2" ry="2" />
              <circle cx="7.5" cy="7.5" r="1.5" />
              <path d="M17 11l-5-5L3 13" />
            </svg>
            背景图片
          </h2>
          
          <div v-if="themeStore.backgroundImage" class="mb-4">
            <div class="relative rounded-xl overflow-hidden border border-[var(--border-color)] mb-4">
              <img 
                :src="themeStore.backgroundImage" 
                alt="背景预览" 
                class="w-full h-48 object-cover"
              />
              <div class="absolute inset-0 bg-gradient-to-t from-black/50 to-transparent"></div>
            </div>
            <button
              @click="removeBackgroundImage"
              class="px-4 py-2 rounded-lg bg-red-500 hover:bg-red-600 text-white font-medium transition-colors"
            >
              移除背景图片
            </button>
          </div>

          <div v-else class="mb-4">
            <div class="border-2 border-dashed border-[var(--border-color)] rounded-xl p-8 text-center">
              <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="mx-auto mb-4 text-[var(--text-secondary)]">
                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
                <polyline points="17 8 12 3 7 8" />
                <line x1="12" y1="3" x2="12" y2="15" />
              </svg>
              <p class="text-[var(--text-secondary)] mb-4">暂无背景图片</p>
            </div>
          </div>

          <div>
            <input
              ref="fileInputRef"
              type="file"
              accept="image/*"
              @change="handleImageSelect"
              class="hidden"
              id="background-image-input"
            />
            <label
              for="background-image-input"
              class="inline-flex items-center gap-2 px-4 py-2 rounded-lg bg-[var(--theme-primary)] hover:bg-[var(--theme-primary)]/90 text-white font-medium cursor-pointer transition-colors"
            >
              <svg width="20" height="20" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M4 16v1a3 3 0 0 0 3 3h10a3 3 0 0 0 3-3v-1m-4-8l-4-4m0 0L8 8m4-4v12" />
              </svg>
              {{ themeStore.backgroundImage ? '更换背景图片' : '选择背景图片' }}
            </label>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
</style>
