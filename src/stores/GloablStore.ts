import { defineStore } from 'pinia'

export const useGlobalStore = defineStore('globalInfo', () => {
  const test_content = '这是一个测试内容'
  return { test_content }
})