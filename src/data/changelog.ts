export interface ChangelogItem {
  version: string
  date: string
  type: 'feature' | 'bugfix' | 'improvement' | 'breaking'
  items: string[]
}

export const changelog: ChangelogItem[] = [
  {
    version: '0.1.0',
    date: '2024-01-01',
    type: 'feature',
    items: [
      '初始版本发布',
      '实现自定义标题栏和窗口控制',
      '添加侧边栏导航功能',
      '支持亮色/暗色主题切换',
      '支持多主题色选择',
      '支持自定义背景图片',
      '添加设置页面',
      '优化界面设计和用户体验'
    ]
  }
]

export const getLatestChangelog = (): ChangelogItem | null => {
  return changelog.length > 0 ? changelog[0] : null
}

export const getRecentChangelogs = (count: number = 3): ChangelogItem[] => {
  return changelog.slice(0, count)
}
