import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

export default defineConfig({
  plugins: [vue()],
  server: {
    proxy: {
      '/front': {
        target: 'https://127.0.0.1:30443',
        changeOrigin: true,
        secure: false
      }
    }
  }
})
