import { defineConfig, presetIcons, presetUno } from 'unocss'

export default defineConfig({
  rules: [
    ['blur', { 'backdrop-filter': 'blur(64px)' }],
  ],
  shortcuts: [
    ['title-btn', 'w-12px h-12px rounded-full flex items-center cursor-default'],
    ['icon-btn', 'text-16px p1 c-gray/80 rounded-full hover:bg-#464646 hover:c-#fff transition-all duration-300 z-10 cursor-pointer'],
  ],
  presets: [
    presetUno(),
    presetIcons({
      scale: 1.2,
      warn: true,
    }),
  ],
})
