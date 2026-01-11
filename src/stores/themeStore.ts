import { defineStore } from 'pinia'
import { ref, computed, watch } from 'vue'

export type ThemeMode = 'light' | 'dark'
export type ThemeColor = 'blue' | 'green' | 'purple' | 'orange' | 'red' | 'pink'

export const useThemeStore = defineStore('theme', () => {
  // 主题模式
  const themeMode = ref<ThemeMode>('light')
  
  // 主题色
  const themeColor = ref<ThemeColor>('blue')
  
  // 背景图片
  const backgroundImage = ref<string | null>(null)
  
  // 侧边栏是否折叠
  const sidebarCollapsed = ref(false)

  // 计算主题色值
  const themeColors = computed(() => {
    const colors: Record<ThemeColor, { primary: string; secondary: string; accent: string }> = {
      blue: { primary: '#3b82f6', secondary: '#60a5fa', accent: '#93c5fd' },
      green: { primary: '#10b981', secondary: '#34d399', accent: '#6ee7b7' },
      purple: { primary: '#8b5cf6', secondary: '#a78bfa', accent: '#c4b5fd' },
      orange: { primary: '#f59e0b', secondary: '#fbbf24', accent: '#fcd34d' },
      red: { primary: '#ef4444', secondary: '#f87171', accent: '#fca5a5' },
      pink: { primary: '#ec4899', secondary: '#f472b6', accent: '#f9a8d4' },
    }
    return colors[themeColor.value]
  })

  // 切换主题模式
  const toggleTheme = () => {
    themeMode.value = themeMode.value === 'light' ? 'dark' : 'light'
    applyTheme()
  }

  // 设置主题模式
  const setThemeMode = (mode: ThemeMode) => {
    themeMode.value = mode
    applyTheme()
  }

  // 设置主题色
  const setThemeColor = (color: ThemeColor) => {
    themeColor.value = color
    applyTheme()
  }

  // 设置背景图片
  const setBackgroundImage = (image: string | null) => {
    backgroundImage.value = image
  }

  // 切换侧边栏
  const toggleSidebar = () => {
    sidebarCollapsed.value = !sidebarCollapsed.value
  }

  // 应用主题到 DOM
  const applyTheme = () => {
    const root = document.documentElement
    const colors = themeColors.value
    
    if (themeMode.value === 'dark') {
      root.style.setProperty('--bg-primary', '#0f0f0f')
      root.style.setProperty('--bg-secondary', '#1a1a1a')
      root.style.setProperty('--bg-tertiary', '#252525')
      root.style.setProperty('--text-primary', '#ffffff')
      root.style.setProperty('--text-secondary', '#a0a0a0')
      root.style.setProperty('--border-color', '#2a2a2a')
    } else {
      root.style.setProperty('--bg-primary', '#ffffff')
      root.style.setProperty('--bg-secondary', '#fafafa')
      root.style.setProperty('--bg-tertiary', '#f0f0f0')
      root.style.setProperty('--text-primary', '#1a1a1a')
      root.style.setProperty('--text-secondary', '#6b7280')
      root.style.setProperty('--border-color', '#e5e7eb')
    }
    
    root.style.setProperty('--theme-primary', colors.primary)
    root.style.setProperty('--theme-secondary', colors.secondary)
    root.style.setProperty('--theme-accent', colors.accent)
    
    // 设置渐变色（用于渐变背景）
    root.style.setProperty('--theme-gradient-from', colors.primary)
    root.style.setProperty('--theme-gradient-to', colors.secondary)
  }

  // 初始化时应用主题
  applyTheme()

  // 监听主题变化
  watch([themeMode, themeColor], () => {
    applyTheme()
  })

  return {
    themeMode,
    themeColor,
    backgroundImage,
    sidebarCollapsed,
    themeColors,
    toggleTheme,
    setThemeMode,
    setThemeColor,
    setBackgroundImage,
    toggleSidebar,
  }
})
